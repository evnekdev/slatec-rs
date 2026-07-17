//! Dense equality- and inequality-constrained linear least squares.
//!
//! This module wraps the original SLATEC `DLSEI` and `LSEI` drivers.  They
//! minimize `||A x - b||₂` subject to exact equality rows `E x = f` and
//! lower-sided linear inequalities `G x ≥ h`.  The native drivers overwrite
//! their augmented matrix and work arrays; this façade always uses owned,
//! initialized copies.

use alloc::{vec, vec::Vec};
use core::fmt;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::linear_least_squares::MatrixRef;

/// A dense block of exact linear equations `matrix * x = rhs`.
///
/// The matrix is a checked column-major [`MatrixRef`].  The safe wrapper copies
/// the data before calling Fortran, so neither the matrix nor the right-hand
/// side is modified.
#[derive(Clone, Copy, Debug)]
pub struct LinearConstraintBlock<'a, T> {
    /// Coefficient matrix with one row per equation.
    pub matrix: MatrixRef<'a, T>,
    /// Right-hand side with one entry per matrix row.
    pub rhs: &'a [T],
}

/// Lower-sided linear inequalities `matrix * x ≥ lower_bounds`.
///
/// This direction is the original `LSEI`/`DLSEI` contract: the routine receives
/// rows `(G H)` and requires `Gx .GE. H`.  It is not silently transformed into
/// an opposite-sign convention.
#[derive(Clone, Copy, Debug)]
pub struct GreaterEqualConstraints<'a, T> {
    /// Inequality coefficient matrix `G` in column-major storage.
    pub matrix: MatrixRef<'a, T>,
    /// Lower right-hand side `h`, with one entry per inequality row.
    pub lower_bounds: &'a [T],
}

/// Immutable dense constrained least-squares problem.
///
/// The solver minimizes `||objective_matrix * x - objective_rhs||₂`, subject
/// to optional exact [`LinearConstraintBlock`] rows and optional
/// [`GreaterEqualConstraints`] rows.  The native block order is always
/// equality, objective, then inequality.
#[derive(Clone, Copy, Debug)]
pub struct ConstrainedLeastSquaresProblem<'a, T> {
    /// Objective matrix `A` in `min ||A x - b||₂`.
    pub objective_matrix: MatrixRef<'a, T>,
    /// Objective right-hand side `b`.
    pub objective_rhs: &'a [T],
    /// Optional exact equality block `E x = f`.
    pub equalities: Option<LinearConstraintBlock<'a, T>>,
    /// Optional lower-sided inequality block `G x ≥ h`.
    pub inequalities: Option<GreaterEqualConstraints<'a, T>>,
}

/// Reviewed rank-control option for `LSEI` and `DLSEI`.
///
/// A supplied value is sent to both native rank tests: `PRGOPT` keys 4 (the
/// equality block) and 5 (the reduced objective).  The source clamps it to at
/// least the profile machine precision.  Covariance, scaling, diagnostic, and
/// raw linked-list controls deliberately remain deferred.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstrainedLeastSquaresOptions<T> {
    /// Optional non-negative rank-determination tolerance for both native rank
    /// tests.  `None` retains the source default, `sqrt(machine_epsilon)`.
    pub rank_tolerance: Option<T>,
}

impl<T> Default for ConstrainedLeastSquaresOptions<T> {
    fn default() -> Self {
        Self {
            rank_tolerance: None,
        }
    }
}

/// Native rank estimates returned by `LSEI` or `DLSEI`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstraintRanks {
    /// `IP(1)`: estimated rank of the equality coefficient matrix `E`.
    pub equality: usize,
    /// `IP(2)`: estimated rank of the reduced least-squares problem.
    pub reduced_objective: usize,
}

/// Meaningful completion states whose solution and residual outputs are
/// defined by the reviewed native contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstrainedLeastSquaresStatus {
    /// Equality and inequality constraints are compatible and satisfied
    /// (`MODE = 0`).
    Converged,
    /// The equality rows are contradictory (`MODE = 1`).
    ///
    /// SLATEC returns a generalized-inverse solution minimizing `||f - Ex||₂`;
    /// the solution, equality residual, and objective residual remain defined.
    EqualityConstraintsContradictory,
}

/// Result of a constrained linear least-squares solve.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstrainedLeastSquaresResult<T> {
    /// Solution vector in the caller's original column order.
    pub solution: Vec<T>,
    /// Native `RNORME`, the Euclidean length `||f - E x||₂`.
    pub equality_residual_norm: T,
    /// Native `RNORML`, the Euclidean length `||b - A x||₂`.
    pub objective_residual_norm: T,
    /// Native rank estimates with their distinct equality and reduced-objective
    /// meanings preserved.
    pub ranks: ConstraintRanks,
    /// Native completion state.
    pub status: ConstrainedLeastSquaresStatus,
    inequality_slacks: Vec<T>,
}

impl<T> ConstrainedLeastSquaresResult<T> {
    /// Returns `Gx - h` for every supplied inequality in caller row order.
    ///
    /// A non-negative entry satisfies its corresponding `Gx ≥ h` constraint;
    /// no inequality block produces an empty slice.
    pub fn inequality_slacks(&self) -> &[T] {
        &self.inequality_slacks
    }
}

impl ConstrainedLeastSquaresResult<f64> {
    /// Returns the smallest recomputed inequality slack, if any inequalities
    /// were supplied.
    pub fn minimum_inequality_slack(&self) -> Option<f64> {
        self.inequality_slacks.iter().copied().reduce(f64::min)
    }
}

impl ConstrainedLeastSquaresResult<f32> {
    /// Returns the smallest recomputed inequality slack, if any inequalities
    /// were supplied.
    pub fn minimum_inequality_slack(&self) -> Option<f32> {
        self.inequality_slacks.iter().copied().reduce(f32::min)
    }
}

/// Validation failure or contained native termination from a constrained
/// least-squares call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstrainedLeastSquaresError {
    /// The objective matrix has no variable columns.
    EmptyVariables,
    /// Equality, objective, and inequality blocks all had zero rows.
    EmptyRows,
    /// A matrix, right-hand side, or block dimension was inconsistent.
    DimensionMismatch {
        /// Invalid input relation.
        argument: &'static str,
    },
    /// An input matrix, right-hand side, or rank tolerance was not finite.
    NonFiniteInput {
        /// Collection or scalar name.
        argument: &'static str,
        /// Zero-based index, or zero for a scalar control.
        index: usize,
    },
    /// A supplied rank tolerance was negative.
    InvalidRankTolerance,
    /// A dimension did not fit GNU Fortran `INTEGER`.
    IntegerOverflow {
        /// Native argument whose conversion failed.
        argument: &'static str,
    },
    /// Checked augmented-storage or workspace arithmetic overflowed `usize`.
    WorkspaceOverflow,
    /// The inequality rows are contradictory (`MODE = 2`), so the native
    /// contract defines no solution output.
    InequalityConstraintsInfeasible,
    /// Equality and inequality rows are jointly contradictory (`MODE = 3`), so
    /// the native contract defines no solution output.
    ConstraintsInfeasible,
    /// `MODE = 4` reported a usage, options, or workspace error that safe
    /// validation should have prevented.
    NativeContractViolation {
        /// Stable explanation of the unexpected native condition.
        detail: &'static str,
    },
    /// An unreviewed native `MODE` value was observed.
    NativeStatus {
        /// Raw SLATEC `MODE` value.
        status: i32,
    },
}

impl fmt::Display for ConstrainedLeastSquaresError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariables => write!(formatter, "constrained least squares needs variables"),
            Self::EmptyRows => write!(
                formatter,
                "constrained least squares needs at least one row"
            ),
            Self::DimensionMismatch { argument } => {
                write!(
                    formatter,
                    "constrained least-squares dimension mismatch for {argument}"
                )
            }
            Self::NonFiniteInput { argument, index } => write!(
                formatter,
                "constrained least-squares {argument} at index {index} must be finite"
            ),
            Self::InvalidRankTolerance => {
                write!(
                    formatter,
                    "constrained least-squares rank tolerance must be non-negative"
                )
            }
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "constrained least-squares {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => {
                write!(formatter, "constrained least-squares workspace overflowed")
            }
            Self::InequalityConstraintsInfeasible => {
                write!(
                    formatter,
                    "constrained least-squares inequalities are contradictory"
                )
            }
            Self::ConstraintsInfeasible => write!(
                formatter,
                "constrained least-squares equality and inequality constraints are contradictory"
            ),
            Self::NativeContractViolation { detail } => {
                write!(
                    formatter,
                    "native constrained least-squares contract was violated: {detail}"
                )
            }
            Self::NativeStatus { status } => {
                write!(formatter, "unknown LSEI/DLSEI MODE value {status}")
            }
        }
    }
}

impl std::error::Error for ConstrainedLeastSquaresError {}

/// Solves a double-precision dense linearly constrained least-squares problem.
///
/// Wraps the original SLATEC `DLSEI` driver.  It minimizes
/// `||A x - b||₂`, satisfies `E x = f` where possible, and enforces
/// `G x ≥ h`.  The public equality, objective, and inequality blocks map to
/// the native `W` row order `E`, `A`, `G`, with right-hand sides in column
/// `N + 1`; the wrapper constructs and owns that mutable column-major array.
///
/// `options.rank_tolerance` maps to both native `PRGOPT` rank controls.  The
/// covariance option exists in the historical routine but is intentionally not
/// enabled: it overwrites `W` with a covariance matrix and needs a separately
/// reviewed statistical policy.
///
/// Calls serialize the GNU MinGW SLATEC runtime and scope the legacy XERROR
/// control while holding that lock.  The prior error setting is restored by
/// RAII.  This hosted allocating API requires `std`, `alloc`,
/// `least-squares-linear-constrained`, a selected backend, and the validated
/// GNU Fortran `x86_64-w64-mingw32` profile; it does not claim bare-metal use.
///
/// # Errors
///
/// Rust validates all shapes, finiteness, integer conversions, and workspace
/// arithmetic before FFI.  Contradictory inequalities (`MODE = 2`) and jointly
/// contradictory equality/inequality rows (`MODE = 3`) return structured
/// errors because the native routine leaves solution outputs undefined.
/// Contradictory equalities alone (`MODE = 1`) instead return the documented
/// generalized-inverse result with [`ConstrainedLeastSquaresStatus`].
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-constrained", feature = "source-build"))]
/// # {
/// use slatec::constrained_least_squares::{
///     solve_constrained_least_squares, ConstrainedLeastSquaresOptions,
///     ConstrainedLeastSquaresProblem, GreaterEqualConstraints,
/// };
/// use slatec::linear_least_squares::MatrixRef;
/// let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1)?;
/// let inequalities = GreaterEqualConstraints {
///     matrix: MatrixRef::column_major(&[1.0_f64], 1, 1, 1)?,
///     lower_bounds: &[2.0],
/// };
/// let fit = solve_constrained_least_squares(
///     ConstrainedLeastSquaresProblem {
///         objective_matrix: objective,
///         objective_rhs: &[0.0],
///         equalities: None,
///         inequalities: Some(inequalities),
///     },
///     ConstrainedLeastSquaresOptions::default(),
/// )?;
/// assert!((fit.solution[0] - 2.0).abs() < 1e-12);
/// # Ok::<(), slatec::constrained_least_squares::ConstrainedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_constrained_least_squares(
    problem: ConstrainedLeastSquaresProblem<'_, f64>,
    options: ConstrainedLeastSquaresOptions<f64>,
) -> Result<ConstrainedLeastSquaresResult<f64>, ConstrainedLeastSquaresError> {
    run_f64(prepare(problem, options, option_array_f64)?)
}

/// Solves the single-precision counterpart of [`solve_constrained_least_squares`].
///
/// Wraps the original SLATEC `LSEI` driver with the same block order,
/// inequality direction, owned workspace, rank-tolerance policy, runtime lock,
/// and XERROR restoration.  Single precision is less robust for nearly
/// dependent rows and nearly active inequalities.
///
/// # Errors
///
/// Returns the same categories of validation, infeasibility, and native
/// contract failures as [`solve_constrained_least_squares`].
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-constrained", feature = "source-build"))]
/// # {
/// use slatec::constrained_least_squares::{
///     solve_constrained_least_squares_f32, ConstrainedLeastSquaresOptions,
///     ConstrainedLeastSquaresProblem,
/// };
/// use slatec::linear_least_squares::MatrixRef;
/// let objective = MatrixRef::column_major(&[1.0_f32], 1, 1, 1)?;
/// let fit = solve_constrained_least_squares_f32(
///     ConstrainedLeastSquaresProblem {
///         objective_matrix: objective,
///         objective_rhs: &[3.0],
///         equalities: None,
///         inequalities: None,
///     },
///     ConstrainedLeastSquaresOptions::default(),
/// )?;
/// assert!((fit.solution[0] - 3.0).abs() < 1e-4);
/// # Ok::<(), slatec::constrained_least_squares::ConstrainedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_constrained_least_squares_f32(
    problem: ConstrainedLeastSquaresProblem<'_, f32>,
    options: ConstrainedLeastSquaresOptions<f32>,
) -> Result<ConstrainedLeastSquaresResult<f32>, ConstrainedLeastSquaresError> {
    run_f32(prepare(problem, options, option_array_f32)?)
}

struct Prepared<'a, T> {
    problem: ConstrainedLeastSquaresProblem<'a, T>,
    augmented: Vec<T>,
    options: Vec<T>,
    equality_rows: usize,
    objective_rows: usize,
    inequality_rows: usize,
    variables: usize,
    workspace: usize,
    integer_workspace: usize,
}

fn prepare<T: Copy + Default + Finite>(
    problem: ConstrainedLeastSquaresProblem<'_, T>,
    options: ConstrainedLeastSquaresOptions<T>,
    option_array: fn(
        ConstrainedLeastSquaresOptions<T>,
    ) -> Result<Vec<T>, ConstrainedLeastSquaresError>,
) -> Result<Prepared<'_, T>, ConstrainedLeastSquaresError> {
    let variables = problem.objective_matrix.columns();
    if variables == 0 {
        return Err(ConstrainedLeastSquaresError::EmptyVariables);
    }
    let objective_rows = problem.objective_matrix.rows();
    validate_block(
        problem.objective_matrix,
        problem.objective_rhs,
        variables,
        "objective",
    )?;
    let equality_rows = match problem.equalities {
        Some(block) => {
            validate_block(block.matrix, block.rhs, variables, "equalities")?;
            block.matrix.rows()
        }
        None => 0,
    };
    let inequality_rows = match problem.inequalities {
        Some(block) => {
            validate_block(block.matrix, block.lower_bounds, variables, "inequalities")?;
            block.matrix.rows()
        }
        None => 0,
    };
    let rows = equality_rows
        .checked_add(objective_rows)
        .and_then(|value| value.checked_add(inequality_rows))
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?;
    if rows == 0 {
        return Err(ConstrainedLeastSquaresError::EmptyRows);
    }
    let columns = variables
        .checked_add(1)
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let augmented_len = rows
        .checked_mul(columns)
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let mut augmented = vec![T::default(); augmented_len];
    let mut row_offset = 0;
    if let Some(block) = problem.equalities {
        copy_block(
            &mut augmented,
            rows,
            variables,
            row_offset,
            block.matrix,
            block.rhs,
        );
        row_offset += block.matrix.rows();
    }
    copy_block(
        &mut augmented,
        rows,
        variables,
        row_offset,
        problem.objective_matrix,
        problem.objective_rhs,
    );
    row_offset += objective_rows;
    if let Some(block) = problem.inequalities {
        copy_block(
            &mut augmented,
            rows,
            variables,
            row_offset,
            block.matrix,
            block.lower_bounds,
        );
    }
    let workspace = workspace_length(equality_rows, objective_rows, inequality_rows, variables)?;
    let integer_workspace = integer_workspace_length(inequality_rows, variables)?;
    for (name, value) in [
        ("MDW", rows),
        ("ME", equality_rows),
        ("MA", objective_rows),
        ("MG", inequality_rows),
        ("N", variables),
        ("WS length", workspace),
        ("IP length", integer_workspace),
    ] {
        to_fortran_integer(value)
            .map_err(|_| ConstrainedLeastSquaresError::IntegerOverflow { argument: name })?;
    }
    Ok(Prepared {
        problem,
        augmented,
        options: option_array(options)?,
        equality_rows,
        objective_rows,
        inequality_rows,
        variables,
        workspace,
        integer_workspace,
    })
}

fn validate_block<T: Copy + Finite>(
    matrix: MatrixRef<'_, T>,
    rhs: &[T],
    variables: usize,
    name: &'static str,
) -> Result<(), ConstrainedLeastSquaresError> {
    if matrix.columns() != variables {
        return Err(ConstrainedLeastSquaresError::DimensionMismatch { argument: name });
    }
    if rhs.len() != matrix.rows() {
        return Err(ConstrainedLeastSquaresError::DimensionMismatch { argument: name });
    }
    finite_slice(matrix.as_column_major_slice(), name)?;
    finite_slice(rhs, name)
}

fn copy_block<T: Copy>(
    augmented: &mut [T],
    leading_dimension: usize,
    variables: usize,
    row_offset: usize,
    matrix: MatrixRef<'_, T>,
    rhs: &[T],
) {
    for column in 0..variables {
        for row in 0..matrix.rows() {
            augmented[row_offset + row + column * leading_dimension] = *matrix
                .get(row, column)
                .expect("MatrixRef checked logical dimensions");
        }
    }
    for row in 0..matrix.rows() {
        augmented[row_offset + row + variables * leading_dimension] = rhs[row];
    }
}

fn workspace_length(
    equality_rows: usize,
    objective_rows: usize,
    inequality_rows: usize,
    variables: usize,
) -> Result<usize, ConstrainedLeastSquaresError> {
    let two_me_plus_n = equality_rows
        .checked_add(variables)
        .and_then(|value| value.checked_mul(2))
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?;
    let k = objective_rows
        .checked_add(inequality_rows)
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?
        .max(variables);
    let inequality_term = inequality_rows
        .checked_add(2)
        .and_then(|rows| {
            variables
                .checked_add(7)
                .and_then(|columns| rows.checked_mul(columns))
        })
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?;
    two_me_plus_n
        .checked_add(k)
        .and_then(|value| value.checked_add(inequality_term))
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)
}

fn integer_workspace_length(
    inequality_rows: usize,
    variables: usize,
) -> Result<usize, ConstrainedLeastSquaresError> {
    inequality_rows
        .checked_add(
            variables
                .checked_mul(2)
                .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)?,
        )
        .and_then(|value| value.checked_add(2))
        .ok_or(ConstrainedLeastSquaresError::WorkspaceOverflow)
}

fn option_array_f64(
    options: ConstrainedLeastSquaresOptions<f64>,
) -> Result<Vec<f64>, ConstrainedLeastSquaresError> {
    match options.rank_tolerance {
        None => Ok(vec![1.0]),
        Some(value) if !value.is_finite() => Err(ConstrainedLeastSquaresError::NonFiniteInput {
            argument: "rank_tolerance",
            index: 0,
        }),
        Some(value) if value < 0.0 => Err(ConstrainedLeastSquaresError::InvalidRankTolerance),
        Some(value) => Ok(vec![4.0, 4.0, value, 7.0, 5.0, value, 1.0]),
    }
}

fn option_array_f32(
    options: ConstrainedLeastSquaresOptions<f32>,
) -> Result<Vec<f32>, ConstrainedLeastSquaresError> {
    match options.rank_tolerance {
        None => Ok(vec![1.0]),
        Some(value) if !value.is_finite() => Err(ConstrainedLeastSquaresError::NonFiniteInput {
            argument: "rank_tolerance",
            index: 0,
        }),
        Some(value) if value < 0.0 => Err(ConstrainedLeastSquaresError::InvalidRankTolerance),
        Some(value) => Ok(vec![4.0, 4.0, value, 7.0, 5.0, value, 1.0]),
    }
}

fn run_f64(
    mut prepared: Prepared<'_, f64>,
) -> Result<ConstrainedLeastSquaresResult<f64>, ConstrainedLeastSquaresError> {
    let mut solution = vec![0.0; prepared.variables];
    let mut workspace = vec![0.0; prepared.workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    integer_workspace[0] = native_integer("WS length", prepared.workspace)?;
    integer_workspace[1] = native_integer("IP length", prepared.integer_workspace)?;
    let mut mdw = native_integer(
        "MDW",
        prepared.equality_rows + prepared.objective_rows + prepared.inequality_rows,
    )?;
    let mut equality_rows = native_integer("ME", prepared.equality_rows)?;
    let mut objective_rows = native_integer("MA", prepared.objective_rows)?;
    let mut inequality_rows = native_integer("MG", prepared.inequality_rows)?;
    let mut variables = native_integer("N", prepared.variables)?;
    let mut equality_norm = 0.0;
    let mut objective_norm = 0.0;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_least_squares_statuses();
    // SAFETY: all scalar dimensions use the reviewed GNU Fortran INTEGER ABI;
    // W(MDW,N+1), PRGOPT, X, WS, and IP are owned, initialized, nonaliasing,
    // and sized by the exact LSEI formula. The shared runtime lock and RAII
    // XERROR scope serialize process-global state and restore it after return.
    unsafe {
        slatec_sys::linear_least_squares::dlsei(
            prepared.augmented.as_mut_ptr(),
            &mut mdw,
            &mut equality_rows,
            &mut objective_rows,
            &mut inequality_rows,
            &mut variables,
            prepared.options.as_mut_ptr(),
            solution.as_mut_ptr(),
            &mut equality_norm,
            &mut objective_norm,
            &mut mode,
            workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(
        prepared.problem,
        solution,
        equality_norm,
        objective_norm,
        mode,
        integer_workspace,
        slacks_f64,
    )
}

fn run_f32(
    mut prepared: Prepared<'_, f32>,
) -> Result<ConstrainedLeastSquaresResult<f32>, ConstrainedLeastSquaresError> {
    let mut solution = vec![0.0_f32; prepared.variables];
    let mut workspace = vec![0.0_f32; prepared.workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    integer_workspace[0] = native_integer("WS length", prepared.workspace)?;
    integer_workspace[1] = native_integer("IP length", prepared.integer_workspace)?;
    let mut mdw = native_integer(
        "MDW",
        prepared.equality_rows + prepared.objective_rows + prepared.inequality_rows,
    )?;
    let mut equality_rows = native_integer("ME", prepared.equality_rows)?;
    let mut objective_rows = native_integer("MA", prepared.objective_rows)?;
    let mut inequality_rows = native_integer("MG", prepared.inequality_rows)?;
    let mut variables = native_integer("N", prepared.variables)?;
    let mut equality_norm = 0.0_f32;
    let mut objective_norm = 0.0_f32;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_least_squares_statuses();
    // SAFETY: f32 counterpart of run_f64. All LSEI arrays are owned,
    // initialized, exactly sized, nonaliasing, and guarded by the shared
    // native-runtime lock and restoring XERROR scope.
    unsafe {
        slatec_sys::linear_least_squares::lsei(
            prepared.augmented.as_mut_ptr(),
            &mut mdw,
            &mut equality_rows,
            &mut objective_rows,
            &mut inequality_rows,
            &mut variables,
            prepared.options.as_mut_ptr(),
            solution.as_mut_ptr(),
            &mut equality_norm,
            &mut objective_norm,
            &mut mode,
            workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(
        prepared.problem,
        solution,
        equality_norm,
        objective_norm,
        mode,
        integer_workspace,
        slacks_f32,
    )
}

fn native_integer(
    argument: &'static str,
    value: usize,
) -> Result<FortranInteger, ConstrainedLeastSquaresError> {
    to_fortran_integer(value)
        .map_err(|_| ConstrainedLeastSquaresError::IntegerOverflow { argument })
}

fn finish<T: Copy + Finite>(
    problem: ConstrainedLeastSquaresProblem<'_, T>,
    solution: Vec<T>,
    equality_norm: T,
    objective_norm: T,
    mode: FortranInteger,
    integer_workspace: Vec<FortranInteger>,
    slacks: fn(ConstrainedLeastSquaresProblem<'_, T>, &[T]) -> Vec<T>,
) -> Result<ConstrainedLeastSquaresResult<T>, ConstrainedLeastSquaresError> {
    let status = match mode {
        0 => ConstrainedLeastSquaresStatus::Converged,
        1 => ConstrainedLeastSquaresStatus::EqualityConstraintsContradictory,
        2 => return Err(ConstrainedLeastSquaresError::InequalityConstraintsInfeasible),
        3 => return Err(ConstrainedLeastSquaresError::ConstraintsInfeasible),
        4 => {
            return Err(ConstrainedLeastSquaresError::NativeContractViolation {
                detail: "LSEI/DLSEI rejected safe dimensions, options, or workspace",
            });
        }
        value => return Err(ConstrainedLeastSquaresError::NativeStatus { status: value }),
    };
    finite_slice(&solution, "native solution")?;
    finite(equality_norm, "native equality residual norm", 0)?;
    finite(objective_norm, "native objective residual norm", 0)?;
    let equality_rank = usize::try_from(integer_workspace[0]).map_err(|_| {
        ConstrainedLeastSquaresError::NativeContractViolation {
            detail: "LSEI/DLSEI returned a negative equality rank",
        }
    })?;
    let reduced_objective_rank = usize::try_from(integer_workspace[1]).map_err(|_| {
        ConstrainedLeastSquaresError::NativeContractViolation {
            detail: "LSEI/DLSEI returned a negative reduced-objective rank",
        }
    })?;
    Ok(ConstrainedLeastSquaresResult {
        inequality_slacks: slacks(problem, &solution),
        solution,
        equality_residual_norm: equality_norm,
        objective_residual_norm: objective_norm,
        ranks: ConstraintRanks {
            equality: equality_rank,
            reduced_objective: reduced_objective_rank,
        },
        status,
    })
}

fn slacks_f64(problem: ConstrainedLeastSquaresProblem<'_, f64>, solution: &[f64]) -> Vec<f64> {
    let Some(block) = problem.inequalities else {
        return Vec::new();
    };
    (0..block.matrix.rows())
        .map(|row| {
            let mut slack = -block.lower_bounds[row];
            for (column, value) in solution.iter().enumerate() {
                slack += *block.matrix.get(row, column).expect("validated") * value;
            }
            slack
        })
        .collect()
}

fn slacks_f32(problem: ConstrainedLeastSquaresProblem<'_, f32>, solution: &[f32]) -> Vec<f32> {
    let Some(block) = problem.inequalities else {
        return Vec::new();
    };
    (0..block.matrix.rows())
        .map(|row| {
            let mut slack = -block.lower_bounds[row];
            for (column, value) in solution.iter().enumerate() {
                slack += *block.matrix.get(row, column).expect("validated") * value;
            }
            slack
        })
        .collect()
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
) -> Result<(), ConstrainedLeastSquaresError> {
    if value.finite() {
        Ok(())
    } else {
        Err(ConstrainedLeastSquaresError::NonFiniteInput { argument, index })
    }
}

fn finite_slice<T: Copy + Finite>(
    data: &[T],
    argument: &'static str,
) -> Result<(), ConstrainedLeastSquaresError> {
    data.iter()
        .copied()
        .enumerate()
        .find(|(_, value)| !value.finite())
        .map_or(Ok(()), |(index, _)| {
            Err(ConstrainedLeastSquaresError::NonFiniteInput { argument, index })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_workspace_matches_reviewed_formula() {
        assert_eq!(
            workspace_length(1, 2, 3, 4).unwrap(),
            2 * (1 + 4) + 5 + (3 + 2) * (4 + 7)
        );
        assert_eq!(integer_workspace_length(3, 4).unwrap(), 13);
    }

    #[test]
    fn rank_tolerance_hides_native_option_language() {
        assert_eq!(
            option_array_f64(ConstrainedLeastSquaresOptions::default()).unwrap(),
            vec![1.0]
        );
        assert_eq!(
            option_array_f64(ConstrainedLeastSquaresOptions {
                rank_tolerance: Some(1e-8)
            })
            .unwrap(),
            vec![4.0, 4.0, 1e-8, 7.0, 5.0, 1e-8, 1.0]
        );
        assert!(matches!(
            option_array_f32(ConstrainedLeastSquaresOptions {
                rank_tolerance: Some(-1.0)
            }),
            Err(ConstrainedLeastSquaresError::InvalidRankTolerance)
        ));
    }

    #[test]
    fn native_storage_orders_equality_objective_then_inequality() {
        let equality = LinearConstraintBlock {
            matrix: MatrixRef::column_major(&[1.0_f64, 2.0], 1, 2, 1).unwrap(),
            rhs: &[3.0],
        };
        let objective = MatrixRef::column_major(&[4.0_f64, 5.0, 6.0, 7.0], 2, 2, 2).unwrap();
        let inequalities = GreaterEqualConstraints {
            matrix: MatrixRef::column_major(&[8.0_f64, 9.0], 1, 2, 1).unwrap(),
            lower_bounds: &[10.0],
        };
        let prepared = prepare(
            ConstrainedLeastSquaresProblem {
                objective_matrix: objective,
                objective_rhs: &[11.0, 12.0],
                equalities: Some(equality),
                inequalities: Some(inequalities),
            },
            ConstrainedLeastSquaresOptions::default(),
            option_array_f64,
        )
        .unwrap();
        assert_eq!(
            prepared.augmented,
            vec![
                1.0, 4.0, 5.0, 8.0, 2.0, 6.0, 7.0, 9.0, 3.0, 11.0, 12.0, 10.0
            ]
        );
    }

    #[test]
    fn native_storage_represents_each_permitted_block_combination() {
        let empty_objective = MatrixRef::column_major(&[] as &[f64], 0, 1, 0).unwrap();
        let equality_only = prepare(
            ConstrainedLeastSquaresProblem {
                objective_matrix: empty_objective,
                objective_rhs: &[],
                equalities: Some(LinearConstraintBlock {
                    matrix: MatrixRef::column_major(&[2.0_f64], 1, 1, 1).unwrap(),
                    rhs: &[3.0],
                }),
                inequalities: None,
            },
            ConstrainedLeastSquaresOptions::default(),
            option_array_f64,
        )
        .unwrap();
        assert_eq!(equality_only.augmented, vec![2.0, 3.0]);

        let objective_only = prepare(
            ConstrainedLeastSquaresProblem {
                objective_matrix: MatrixRef::column_major(&[4.0_f64], 1, 1, 1).unwrap(),
                objective_rhs: &[5.0],
                equalities: None,
                inequalities: None,
            },
            ConstrainedLeastSquaresOptions::default(),
            option_array_f64,
        )
        .unwrap();
        assert_eq!(objective_only.augmented, vec![4.0, 5.0]);

        let inequality_only = prepare(
            ConstrainedLeastSquaresProblem {
                objective_matrix: empty_objective,
                objective_rhs: &[],
                equalities: None,
                inequalities: Some(GreaterEqualConstraints {
                    matrix: MatrixRef::column_major(&[6.0_f64], 1, 1, 1).unwrap(),
                    lower_bounds: &[7.0],
                }),
            },
            ConstrainedLeastSquaresOptions::default(),
            option_array_f64,
        )
        .unwrap();
        assert_eq!(inequality_only.augmented, vec![6.0, 7.0]);
    }

    #[test]
    fn shape_errors_are_rejected_before_native_execution() {
        let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
        assert!(matches!(
            prepare(
                ConstrainedLeastSquaresProblem {
                    objective_matrix: objective,
                    objective_rhs: &[],
                    equalities: None,
                    inequalities: None,
                },
                ConstrainedLeastSquaresOptions::default(),
                option_array_f64,
            ),
            Err(ConstrainedLeastSquaresError::DimensionMismatch { .. })
        ));
    }
}
