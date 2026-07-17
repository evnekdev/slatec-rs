//! Dense least-squares fitting with bounds on variables and linear forms.
//!
//! This module wraps the original SLATEC `SBOCLS` and `DBOCLS` drivers.  They
//! minimize `||E x - f||₂` while introducing an auxiliary vector `y = C x`.
//! Bounds apply both to the original variables `x` and to the components of
//! `y`, thereby representing equality, one-sided, and interval constraints on
//! linear forms without exposing the mutable Fortran workspace protocol.

use alloc::{vec, vec::Vec};
use core::fmt;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::linear_least_squares::{MatrixRef, VariableBounds};

/// Bounds on the auxiliary values `y = C x` in a bounded constrained fit.
///
/// Each row of `matrix` defines one component of `y`.  The corresponding
/// entry of `bounds` applies a closed equality, lower, upper, interval, or no
/// bound to that linear form.  Thus `VariableBounds::Fixed(value)` represents
/// `C[row, :] x = value`, and `VariableBounds::Lower(value)` represents
/// `C[row, :] x >= value`.
#[derive(Clone, Copy, Debug)]
pub struct BoundedLinearConstraints<'a, T> {
    /// Constraint coefficient matrix `C` in checked Fortran column-major
    /// storage.
    pub matrix: MatrixRef<'a, T>,
    /// One bound selection for every row of [`Self::matrix`], applied to
    /// `C x` in row order.
    pub bounds: &'a [VariableBounds<T>],
}

/// Input to [`solve_bounded_constrained_least_squares`] or
/// [`solve_bounded_constrained_least_squares_f32`].
///
/// It represents
///
/// ```text
/// minimize ||E x - f||₂
/// subject to C x = y, with bounds on x and y.
/// ```
///
/// Bounds on `y` express the public linear-constraint relations directly.
/// The original driver may enlarge infeasible `y` bounds to obtain a closest
/// reachable problem; the result reports that case explicitly.
#[derive(Clone, Copy, Debug)]
pub struct BoundedConstrainedLeastSquaresProblem<'a, T> {
    /// Objective coefficient matrix `E` in checked Fortran column-major
    /// storage. It may have zero rows when at least one constraint row exists.
    pub objective_matrix: MatrixRef<'a, T>,
    /// Objective right-hand side `f`, with one entry per objective row.
    pub objective_rhs: &'a [T],
    /// Linear-form matrix and bounds for the auxiliary values `y = C x`.
    pub constraints: BoundedLinearConstraints<'a, T>,
    /// Closed bounds on original variables `x`, with one entry per objective
    /// matrix column.
    pub variable_bounds: &'a [VariableBounds<T>],
}

/// Safe option set for a bounded constrained least-squares solve.
///
/// The zero-sized type deliberately selects the reviewed native defaults.
/// The `IOPT` protocol also supports accumulation, mutable user scaling, and
/// recursively embedded `DBOLS`/`SBOLS` and `DBOLSM`/`SBOLSM` option arrays.
/// Those facilities require mutable protocol state or have not yet received a
/// safe public contract, so they remain deferred rather than exposing raw
/// selector integers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BoundedConstrainedLeastSquaresOptions;

/// Native completion state for a bounded constrained least-squares solve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedConstrainedLeastSquaresStatus {
    /// The native active-set process completed normally (`MODE >= 0`).
    Converged,
    /// The subsidiary bounded solver reached its drop/add iteration limit
    /// (`MODE = -22`) and returned an approximate solution.
    MaximumIterations,
}

/// Whether the original requested bounds on the linear forms `C x` were
/// simultaneously reachable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstraintFeasibility {
    /// The native `RNORMC` output was exactly zero, as documented for a
    /// feasible constraint/bound system.
    Feasible,
    /// The native preprocessing returned a nonzero `RNORMC` and enlarged one
    /// or more auxiliary-value bounds to obtain a reachable problem.
    ///
    /// The result's solution is useful for the perturbed native problem, but
    /// callers must not treat it as satisfying every originally requested
    /// bound on `C x`.
    BoundsRelaxed,
}

/// Result of a bounded constrained least-squares solve.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundedConstrainedLeastSquaresResult<T> {
    /// Estimated original variables in caller column order.
    pub solution: Vec<T>,
    /// Native `RNORM`: the Euclidean objective residual `||E x - f||₂`.
    ///
    /// It is not squared and it is not an RMS residual.
    pub objective_residual_norm: T,
    /// Native `RNORMC`: the minimum residual length for `C x - y = 0` during
    /// the driver's constraint-reachability preprocessing.
    ///
    /// It is normally zero.  A nonzero value means the driver enlarged bounds
    /// on auxiliary values `y`; see [`Self::constraint_feasibility`].
    pub constraint_residual_norm: T,
    /// Feasibility classification derived directly from the reviewed native
    /// `RNORMC` convention.
    pub constraint_feasibility: ConstraintFeasibility,
    /// Native solve-completion state after safe status mapping.
    pub status: BoundedConstrainedLeastSquaresStatus,
}

/// Validation or contained native-contract failure from a bounded constrained
/// least-squares solve.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedConstrainedLeastSquaresError {
    /// The objective matrix has no variables.
    EmptyVariables,
    /// Both the objective and constraint blocks have zero rows.
    EmptyProblem,
    /// A vector length or matrix shape disagrees with the public model.
    DimensionMismatch {
        /// The invalid argument or dimension relationship.
        argument: &'static str,
    },
    /// A matrix, right-hand side, or finite bound value was NaN or infinite.
    NonFiniteInput {
        /// Input collection or field containing the non-finite value.
        argument: &'static str,
        /// Zero-based index in the named collection.
        index: usize,
    },
    /// A two-sided variable or linear-form interval had `lower > upper`.
    InconsistentBounds {
        /// Whether the invalid bound belongs to `x` or to `C x`.
        argument: &'static str,
        /// Zero-based variable or constraint-row index.
        index: usize,
    },
    /// A count could not be represented by the validated GNU Fortran
    /// `INTEGER` type.
    IntegerOverflow {
        /// Count or dimension that did not fit native `INTEGER`.
        argument: &'static str,
    },
    /// Checked native-storage or workspace arithmetic overflowed `usize`.
    WorkspaceOverflow,
    /// The native driver rejected a safe internally constructed storage or
    /// option contract.
    NativeContractViolation {
        /// Stable detail about the unexpected native condition.
        detail: &'static str,
    },
    /// An unreviewed native `MODE` value was observed.
    NativeStatus {
        /// Raw SLATEC `MODE` value.
        status: i32,
    },
}

impl fmt::Display for BoundedConstrainedLeastSquaresError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariables => write!(
                formatter,
                "bounded constrained least squares needs variables"
            ),
            Self::EmptyProblem => write!(
                formatter,
                "bounded constrained least squares needs objective or constraint rows"
            ),
            Self::DimensionMismatch { argument } => write!(
                formatter,
                "bounded constrained least-squares dimension mismatch for {argument}"
            ),
            Self::NonFiniteInput { argument, index } => write!(
                formatter,
                "bounded constrained least-squares {argument} at index {index} must be finite"
            ),
            Self::InconsistentBounds { argument, index } => write!(
                formatter,
                "bounded constrained least-squares {argument} bounds at index {index} are inconsistent"
            ),
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "bounded constrained least-squares {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => write!(
                formatter,
                "bounded constrained least-squares workspace arithmetic overflowed"
            ),
            Self::NativeContractViolation { detail } => write!(
                formatter,
                "native bounded constrained least-squares contract was violated: {detail}"
            ),
            Self::NativeStatus { status } => {
                write!(formatter, "unknown SBOCLS/DBOCLS MODE value {status}")
            }
        }
    }
}

impl std::error::Error for BoundedConstrainedLeastSquaresError {}

/// Solves a double-precision bounded constrained least-squares problem.
///
/// Wraps the original SLATEC `DBOCLS` routine. It minimizes `||E x - f||₂`
/// while bounding both `x` and each linear form `C x`. A fixed constraint-row
/// bound is an equality; lower/upper bounds are `C x >= lower` and
/// `C x <= upper`; a two-sided bound supplies both inequalities. This differs
/// from [`crate::bounded_least_squares::solve_bounded_least_squares`], which
/// bounds only variables, and from [`crate::constrained_least_squares`],
/// which has no variable bounds.
///
/// `objective_matrix`/`objective_rhs` correspond to the `E,F` part of native
/// `W`; `constraints.matrix` corresponds to `C`. The wrapper owns the mutable
/// `W(MDW, NCOLS + MCON + 1)`, `BL`, `BU`, `IND`, `IOPT[17]`,
/// `X[2*(NCOLS+MCON)+2]`, `RW[6*NCOLS+5*MCON]`, and
/// `IW[2*(NCOLS+MCON)]` arrays. It uses the original default option policy.
///
/// Calls serialize the GNU MinGW native runtime because the drivers use saved
/// state, machine constants, and the legacy error system. A scoped XERROR
/// control permits only the documented `MODE = -22` numerical termination and
/// restores the previous setting before return. This API requires `std`,
/// `alloc`, `least-squares-linear-bounded-constrained`, a selected native
/// backend, and the validated `x86_64-w64-mingw32` GNU Fortran profile; it is
/// not a bare-metal claim.
///
/// # Errors
///
/// Returns errors before FFI for invalid shapes, non-finite data, inconsistent
/// intervals, nonrepresentable native dimensions, or workspace overflow. A
/// native maximum-iteration termination is returned as
/// [`BoundedConstrainedLeastSquaresStatus::MaximumIterations`]. A nonzero
/// constraint residual is a structured result classification rather than a
/// rank error because `DBOCLS` documents it as bound relaxation.
///
/// # Numerical considerations
///
/// `DBOCLS` provides no reviewed public numerical-rank output; rank-deficient
/// problems may have non-unique minimizers. The driver may preprocess an
/// inconsistent `C x` bound system by enlarging the auxiliary bounds. Inspect
/// [`BoundedConstrainedLeastSquaresResult::constraint_feasibility`] before
/// relying on the original constraints.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-bounded-constrained", feature = "source-build"))]
/// # {
/// use slatec::bounded_constrained_least_squares::{
///     solve_bounded_constrained_least_squares, BoundedConstrainedLeastSquaresOptions,
///     BoundedConstrainedLeastSquaresProblem, BoundedLinearConstraints,
/// };
/// use slatec::linear_least_squares::{MatrixRef, VariableBounds};
/// let objective = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
/// let constraints = MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1)?;
/// let fit = solve_bounded_constrained_least_squares(
///     BoundedConstrainedLeastSquaresProblem {
///         objective_matrix: objective,
///         objective_rhs: &[2.0, 2.0],
///         constraints: BoundedLinearConstraints {
///             matrix: constraints,
///             bounds: &[VariableBounds::Fixed(1.0)],
///         },
///         variable_bounds: &[VariableBounds::Lower(0.0), VariableBounds::Lower(0.0)],
///     },
///     BoundedConstrainedLeastSquaresOptions,
/// )?;
/// assert!((fit.solution[0] - 0.5).abs() < 1e-10);
/// assert_eq!(fit.constraint_feasibility, slatec::bounded_constrained_least_squares::ConstraintFeasibility::Feasible);
/// # Ok::<(), slatec::bounded_constrained_least_squares::BoundedConstrainedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_bounded_constrained_least_squares(
    problem: BoundedConstrainedLeastSquaresProblem<'_, f64>,
    options: BoundedConstrainedLeastSquaresOptions,
) -> Result<BoundedConstrainedLeastSquaresResult<f64>, BoundedConstrainedLeastSquaresError> {
    let prepared = prepare(problem, options)?;
    run_f64(prepared)
}

/// Solves the single-precision counterpart of
/// [`solve_bounded_constrained_least_squares`].
///
/// Wraps SLATEC `SBOCLS` with the same bounded-linear-form model, copied
/// native storage, default-option policy, serialized runtime, and contained
/// XERROR handling as the double-precision wrapper. Single precision is more
/// sensitive to ill-conditioned objective or constraint matrices.
///
/// # Errors
///
/// Returns the same validation and native-contract errors as the
/// double-precision function.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-bounded-constrained", feature = "source-build"))]
/// # {
/// use slatec::bounded_constrained_least_squares::{
///     solve_bounded_constrained_least_squares_f32, BoundedConstrainedLeastSquaresOptions,
///     BoundedConstrainedLeastSquaresProblem, BoundedLinearConstraints,
/// };
/// use slatec::linear_least_squares::{MatrixRef, VariableBounds};
/// let objective = MatrixRef::column_major(&[1.0_f32], 1, 1, 1)?;
/// let constraints = MatrixRef::column_major(&[1.0_f32], 1, 1, 1)?;
/// let fit = solve_bounded_constrained_least_squares_f32(
///     BoundedConstrainedLeastSquaresProblem {
///         objective_matrix: objective,
///         objective_rhs: &[3.0],
///         constraints: BoundedLinearConstraints { matrix: constraints, bounds: &[VariableBounds::Upper(2.0)] },
///         variable_bounds: &[VariableBounds::Unbounded],
///     },
///     BoundedConstrainedLeastSquaresOptions,
/// )?;
/// assert!((fit.solution[0] - 2.0).abs() < 1e-4);
/// # Ok::<(), slatec::bounded_constrained_least_squares::BoundedConstrainedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_bounded_constrained_least_squares_f32(
    problem: BoundedConstrainedLeastSquaresProblem<'_, f32>,
    options: BoundedConstrainedLeastSquaresOptions,
) -> Result<BoundedConstrainedLeastSquaresResult<f32>, BoundedConstrainedLeastSquaresError> {
    let prepared = prepare(problem, options)?;
    run_f32(prepared)
}

struct Prepared<T> {
    augmented: Vec<T>,
    lower: Vec<T>,
    upper: Vec<T>,
    bound_types: Vec<FortranInteger>,
    options: Vec<FortranInteger>,
    leading_dimension: usize,
    constraint_rows: usize,
    objective_rows: usize,
    variables: usize,
    solution_storage: usize,
    real_workspace: usize,
    integer_workspace: usize,
}

fn prepare<T: Copy + Default + Finite + PartialOrd>(
    problem: BoundedConstrainedLeastSquaresProblem<'_, T>,
    _options: BoundedConstrainedLeastSquaresOptions,
) -> Result<Prepared<T>, BoundedConstrainedLeastSquaresError> {
    let variables = problem.objective_matrix.columns();
    let objective_rows = problem.objective_matrix.rows();
    let constraint_rows = problem.constraints.matrix.rows();
    if variables == 0 {
        return Err(BoundedConstrainedLeastSquaresError::EmptyVariables);
    }
    if objective_rows == 0 && constraint_rows == 0 {
        return Err(BoundedConstrainedLeastSquaresError::EmptyProblem);
    }
    if problem.objective_rhs.len() != objective_rows {
        return Err(BoundedConstrainedLeastSquaresError::DimensionMismatch {
            argument: "objective_rhs",
        });
    }
    if problem.constraints.matrix.columns() != variables {
        return Err(BoundedConstrainedLeastSquaresError::DimensionMismatch {
            argument: "constraints.matrix columns",
        });
    }
    if problem.constraints.bounds.len() != constraint_rows {
        return Err(BoundedConstrainedLeastSquaresError::DimensionMismatch {
            argument: "constraints.bounds",
        });
    }
    if problem.variable_bounds.len() != variables {
        return Err(BoundedConstrainedLeastSquaresError::DimensionMismatch {
            argument: "variable_bounds",
        });
    }
    finite_slice(
        problem.objective_matrix.as_column_major_slice(),
        "objective_matrix",
    )?;
    finite_slice(problem.objective_rhs, "objective_rhs")?;
    finite_slice(
        problem.constraints.matrix.as_column_major_slice(),
        "constraints.matrix",
    )?;

    let total_variables = variables
        .checked_add(constraint_rows)
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let (lower, upper, bound_types) =
        encode_bounds(problem.variable_bounds, problem.constraints.bounds)?;
    let leading_dimension = constraint_rows
        .checked_add(objective_rows.max(variables))
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let columns = total_variables
        .checked_add(1)
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let augmented_len = leading_dimension
        .checked_mul(columns)
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let mut augmented = vec![T::default(); augmented_len];
    fill_augmented(
        &problem,
        &mut augmented,
        leading_dimension,
        variables,
        constraint_rows,
        objective_rows,
    );

    let solution_storage = total_variables
        .checked_mul(2)
        .and_then(|value| value.checked_add(2))
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let real_workspace = variables
        .checked_mul(6)
        .and_then(|value| {
            constraint_rows
                .checked_mul(5)
                .and_then(|rows| value.checked_add(rows))
        })
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let integer_workspace = total_variables
        .checked_mul(2)
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    for (name, value) in [
        ("MDW", leading_dimension),
        ("MCON", constraint_rows),
        ("MROWS", objective_rows),
        ("NCOLS", variables),
        ("NCOLS + MCON", total_variables),
        ("X length", solution_storage),
        ("RW length", real_workspace),
        ("IW length", integer_workspace),
        ("IOPT length", 17),
    ] {
        to_fortran_integer(value)
            .map_err(|_| BoundedConstrainedLeastSquaresError::IntegerOverflow { argument: name })?;
    }
    let mut options = vec![0; 17];
    options[0] = 99;
    Ok(Prepared {
        augmented,
        lower,
        upper,
        bound_types,
        options,
        leading_dimension,
        constraint_rows,
        objective_rows,
        variables,
        solution_storage,
        real_workspace,
        integer_workspace,
    })
}

fn fill_augmented<T: Copy + Default>(
    problem: &BoundedConstrainedLeastSquaresProblem<'_, T>,
    augmented: &mut [T],
    leading_dimension: usize,
    variables: usize,
    constraint_rows: usize,
    objective_rows: usize,
) {
    for column in 0..variables {
        for row in 0..constraint_rows {
            augmented[row + column * leading_dimension] = *problem
                .constraints
                .matrix
                .get(row, column)
                .expect("validated constraint matrix");
        }
        for row in 0..objective_rows {
            augmented[constraint_rows + row + column * leading_dimension] = *problem
                .objective_matrix
                .get(row, column)
                .expect("validated objective matrix");
        }
    }
    for row in 0..objective_rows {
        augmented[constraint_rows + row + variables * leading_dimension] =
            problem.objective_rhs[row];
    }
}

fn encode_bounds<T: Copy + Default + Finite + PartialOrd>(
    variable_bounds: &[VariableBounds<T>],
    constraint_bounds: &[VariableBounds<T>],
) -> Result<(Vec<T>, Vec<T>, Vec<FortranInteger>), BoundedConstrainedLeastSquaresError> {
    let total = variable_bounds
        .len()
        .checked_add(constraint_bounds.len())
        .ok_or(BoundedConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let mut lower = vec![T::default(); total];
    let mut upper = vec![T::default(); total];
    let mut types = vec![0; total];
    encode_bound_slice(
        variable_bounds,
        "variable_bounds",
        0,
        &mut lower,
        &mut upper,
        &mut types,
    )?;
    encode_bound_slice(
        constraint_bounds,
        "constraints.bounds",
        variable_bounds.len(),
        &mut lower,
        &mut upper,
        &mut types,
    )?;
    Ok((lower, upper, types))
}

fn encode_bound_slice<T: Copy + Default + Finite + PartialOrd>(
    bounds: &[VariableBounds<T>],
    argument: &'static str,
    offset: usize,
    lower: &mut [T],
    upper: &mut [T],
    kinds: &mut [FortranInteger],
) -> Result<(), BoundedConstrainedLeastSquaresError> {
    for (index, bound) in bounds.iter().copied().enumerate() {
        let native_index = offset + index;
        match bound {
            VariableBounds::Unbounded => kinds[native_index] = 4,
            VariableBounds::Lower(value) => {
                finite(value, argument, index)?;
                lower[native_index] = value;
                kinds[native_index] = 1;
            }
            VariableBounds::Upper(value) => {
                finite(value, argument, index)?;
                upper[native_index] = value;
                kinds[native_index] = 2;
            }
            VariableBounds::Between {
                lower: lo,
                upper: hi,
            } => {
                finite(lo, argument, index)?;
                finite(hi, argument, index)?;
                if lo > hi {
                    return Err(BoundedConstrainedLeastSquaresError::InconsistentBounds {
                        argument,
                        index,
                    });
                }
                lower[native_index] = lo;
                upper[native_index] = hi;
                kinds[native_index] = 3;
            }
            VariableBounds::Fixed(value) => {
                finite(value, argument, index)?;
                lower[native_index] = value;
                upper[native_index] = value;
                kinds[native_index] = 3;
            }
        }
    }
    Ok(())
}

fn run_f64(
    mut prepared: Prepared<f64>,
) -> Result<BoundedConstrainedLeastSquaresResult<f64>, BoundedConstrainedLeastSquaresError> {
    let mut output = vec![0.0; prepared.solution_storage];
    let mut real_workspace = vec![0.0; prepared.real_workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    let mut leading_dimension = native_integer("MDW", prepared.leading_dimension)?;
    let mut constraint_rows = native_integer("MCON", prepared.constraint_rows)?;
    let mut objective_rows = native_integer("MROWS", prepared.objective_rows)?;
    let mut variables = native_integer("NCOLS", prepared.variables)?;
    let mut constraint_norm = 0.0;
    let mut objective_norm = 0.0;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_least_squares_statuses();
    // SAFETY: every native dimension and exact reviewed storage formula was
    // checked; W, BL, BU, IND, IOPT, X, RW, and IW are initialized, owned,
    // nonaliasing allocations. The shared lock and RAII XERROR scope protect
    // saved native state and the documented MODE=-22 return path.
    unsafe {
        slatec_sys::linear_least_squares::dbocls(
            prepared.augmented.as_mut_ptr(),
            &mut leading_dimension,
            &mut constraint_rows,
            &mut objective_rows,
            &mut variables,
            prepared.lower.as_mut_ptr(),
            prepared.upper.as_mut_ptr(),
            prepared.bound_types.as_mut_ptr(),
            prepared.options.as_mut_ptr(),
            output.as_mut_ptr(),
            &mut constraint_norm,
            &mut objective_norm,
            &mut mode,
            real_workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(
        output,
        prepared.variables,
        constraint_norm,
        objective_norm,
        mode,
    )
}

fn run_f32(
    mut prepared: Prepared<f32>,
) -> Result<BoundedConstrainedLeastSquaresResult<f32>, BoundedConstrainedLeastSquaresError> {
    let mut output = vec![0.0_f32; prepared.solution_storage];
    let mut real_workspace = vec![0.0_f32; prepared.real_workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    let mut leading_dimension = native_integer("MDW", prepared.leading_dimension)?;
    let mut constraint_rows = native_integer("MCON", prepared.constraint_rows)?;
    let mut objective_rows = native_integer("MROWS", prepared.objective_rows)?;
    let mut variables = native_integer("NCOLS", prepared.variables)?;
    let mut constraint_norm = 0.0_f32;
    let mut objective_norm = 0.0_f32;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_least_squares_statuses();
    // SAFETY: f32 counterpart of run_f64. All reviewed native arrays are
    // owned, initialized, correctly sized, nonaliasing, and protected by the
    // process-global runtime and scoped XERROR restoration guard.
    unsafe {
        slatec_sys::linear_least_squares::sbocls(
            prepared.augmented.as_mut_ptr(),
            &mut leading_dimension,
            &mut constraint_rows,
            &mut objective_rows,
            &mut variables,
            prepared.lower.as_mut_ptr(),
            prepared.upper.as_mut_ptr(),
            prepared.bound_types.as_mut_ptr(),
            prepared.options.as_mut_ptr(),
            output.as_mut_ptr(),
            &mut constraint_norm,
            &mut objective_norm,
            &mut mode,
            real_workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(
        output,
        prepared.variables,
        constraint_norm,
        objective_norm,
        mode,
    )
}

fn native_integer(
    argument: &'static str,
    value: usize,
) -> Result<FortranInteger, BoundedConstrainedLeastSquaresError> {
    to_fortran_integer(value)
        .map_err(|_| BoundedConstrainedLeastSquaresError::IntegerOverflow { argument })
}

fn finish<T: Copy + Default + Finite + PartialEq>(
    output: Vec<T>,
    variables: usize,
    constraint_residual_norm: T,
    objective_residual_norm: T,
    mode: FortranInteger,
) -> Result<BoundedConstrainedLeastSquaresResult<T>, BoundedConstrainedLeastSquaresError> {
    let solution = output.into_iter().take(variables).collect::<Vec<_>>();
    finite_slice(&solution, "native solution")?;
    finite(
        constraint_residual_norm,
        "native constraint residual norm",
        0,
    )?;
    finite(objective_residual_norm, "native objective residual norm", 0)?;
    let status = match mode {
        value if value >= 0 => BoundedConstrainedLeastSquaresStatus::Converged,
        -22 => BoundedConstrainedLeastSquaresStatus::MaximumIterations,
        value @ (-57..=-23) | value @ (-19..=-2) => {
            return Err(
                BoundedConstrainedLeastSquaresError::NativeContractViolation {
                    detail: native_contract_detail(value),
                },
            );
        }
        value => return Err(BoundedConstrainedLeastSquaresError::NativeStatus { status: value }),
    };
    let constraint_feasibility = if constraint_residual_norm == T::default() {
        ConstraintFeasibility::Feasible
    } else {
        ConstraintFeasibility::BoundsRelaxed
    };
    Ok(BoundedConstrainedLeastSquaresResult {
        solution,
        objective_residual_norm,
        constraint_residual_norm,
        constraint_feasibility,
        status,
    })
}

fn native_contract_detail(mode: FortranInteger) -> &'static str {
    match mode {
        -57..=-41 => "SBOCLS/DBOCLS rejected safe dimensions, bounds, or option storage",
        -38..=-23 => {
            "SBOLSM/DBOLSM rejected an internally constructed option or workspace contract"
        }
        -19..=-2 => "SBOLS/DBOLS rejected an internally constructed bounded solve contract",
        _ => "SBOCLS/DBOCLS returned an unexpected contract status",
    }
}

trait Finite {
    fn finite(self) -> bool;
}

impl Finite for f64 {
    fn finite(self) -> bool {
        self.is_finite()
    }
}

impl Finite for f32 {
    fn finite(self) -> bool {
        self.is_finite()
    }
}

fn finite<T: Finite>(
    value: T,
    argument: &'static str,
    index: usize,
) -> Result<(), BoundedConstrainedLeastSquaresError> {
    if value.finite() {
        Ok(())
    } else {
        Err(BoundedConstrainedLeastSquaresError::NonFiniteInput { argument, index })
    }
}

fn finite_slice<T: Copy + Finite>(
    data: &[T],
    argument: &'static str,
) -> Result<(), BoundedConstrainedLeastSquaresError> {
    data.iter()
        .copied()
        .enumerate()
        .find(|(_, value)| !value.finite())
        .map_or(Ok(()), |(index, _)| {
            Err(BoundedConstrainedLeastSquaresError::NonFiniteInput { argument, index })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_encoding_keeps_variables_before_constraint_values() {
        let (lower, upper, kinds) = encode_bounds(
            &[VariableBounds::Lower(1.0_f64), VariableBounds::Unbounded],
            &[VariableBounds::Fixed(3.0), VariableBounds::Upper(4.0)],
        )
        .unwrap();
        assert_eq!(kinds, vec![1, 4, 3, 2]);
        assert_eq!(lower, vec![1.0, 0.0, 3.0, 0.0]);
        assert_eq!(upper, vec![0.0, 0.0, 3.0, 4.0]);
    }

    #[test]
    fn native_storage_orders_constraints_then_objective() {
        let objective = MatrixRef::column_major(&[1.0_f64, 3.0, 2.0, 4.0], 2, 2, 2).unwrap();
        let constraints = MatrixRef::column_major(&[5.0_f64, 6.0], 1, 2, 1).unwrap();
        let prepared = prepare(
            BoundedConstrainedLeastSquaresProblem {
                objective_matrix: objective,
                objective_rhs: &[7.0, 8.0],
                constraints: BoundedLinearConstraints {
                    matrix: constraints,
                    bounds: &[VariableBounds::Fixed(9.0)],
                },
                variable_bounds: &[VariableBounds::Unbounded, VariableBounds::Lower(0.0)],
            },
            BoundedConstrainedLeastSquaresOptions,
        )
        .unwrap();
        assert_eq!(prepared.leading_dimension, 3);
        assert_eq!(
            prepared.augmented,
            vec![5.0, 1.0, 3.0, 6.0, 2.0, 4.0, 0.0, 7.0, 8.0, 0.0, 0.0, 0.0]
        );
    }

    #[test]
    fn exact_workspace_formula_is_checked() {
        let objective = MatrixRef::column_major(&[1.0_f64, 2.0, 3.0], 3, 1, 3).unwrap();
        let constraints = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
        let prepared = prepare(
            BoundedConstrainedLeastSquaresProblem {
                objective_matrix: objective,
                objective_rhs: &[1.0, 2.0, 3.0],
                constraints: BoundedLinearConstraints {
                    matrix: constraints,
                    bounds: &[VariableBounds::Fixed(1.0)],
                },
                variable_bounds: &[VariableBounds::Unbounded],
            },
            BoundedConstrainedLeastSquaresOptions,
        )
        .unwrap();
        assert_eq!(prepared.solution_storage, 6);
        assert_eq!(prepared.real_workspace, 11);
        assert_eq!(prepared.integer_workspace, 4);
    }

    #[test]
    fn inconsistent_constraint_interval_is_rejected_before_ffi() {
        let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
        let constraints = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
        assert!(matches!(
            prepare(
                BoundedConstrainedLeastSquaresProblem {
                    objective_matrix: objective,
                    objective_rhs: &[1.0],
                    constraints: BoundedLinearConstraints {
                        matrix: constraints,
                        bounds: &[VariableBounds::Between {
                            lower: 2.0,
                            upper: 1.0
                        }],
                    },
                    variable_bounds: &[VariableBounds::Unbounded],
                },
                BoundedConstrainedLeastSquaresOptions,
            ),
            Err(BoundedConstrainedLeastSquaresError::InconsistentBounds {
                argument: "constraints.bounds",
                ..
            })
        ));
    }
}
