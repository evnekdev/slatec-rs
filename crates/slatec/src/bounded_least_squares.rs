//! Dense bounded linear least-squares fitting.
//!
//! This module wraps the original SLATEC `DBOLS` and `SBOLS` drivers. They
//! solve `min ||A x - b||₂` subject to independently selected closed lower
//! and upper bounds. The native routines overwrite their augmented matrix,
//! bound arrays, and work arrays; this safe façade owns all of that mutable
//! storage and never mutates the caller's matrix or right-hand side.

use alloc::{vec, vec::Vec};
use core::fmt;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::linear_least_squares::MatrixRef;

/// Closed bounds for one least-squares variable.
///
/// This is the shared [`crate::linear_least_squares::VariableBounds`] type,
/// re-exported at its original path for API compatibility.
pub use crate::linear_least_squares::VariableBounds;

/// Column-scaling policy exposed from the reviewed `SBOLS`/`DBOLS` option
/// array.
///
/// Scaling changes internal numerical conditioning only. Bounds and the
/// returned solution remain expressed in the caller's original variable
/// units.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BoundedLeastSquaresScaling {
    /// Scale each nonzero column by its largest-magnitude entry. This is the
    /// native default when no option is supplied.
    #[default]
    Nominal,
    /// Scale each nonzero column to Euclidean length one.
    EuclideanColumns,
    /// Disable native column scaling.
    Identity,
}

/// Reviewed optional controls for a bounded least-squares solve.
///
/// The safe API deliberately does not expose the raw `IOPT` language,
/// sequential row accumulation, debug output, user-supplied scaling storage,
/// or low-level `SBOLSM`/`DBOLSM` rank-tolerance controls. Those protocols
/// require caller-managed mutable state or can issue legacy messages for
/// ordinary tuning values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BoundedLeastSquaresOptions {
    /// Native column-scaling policy. [`BoundedLeastSquaresScaling::Nominal`]
    /// preserves the SLATEC default.
    pub scaling: BoundedLeastSquaresScaling,
    /// Optional positive maximum number of native drop/add iterations.
    ///
    /// When omitted, the original `SBOLSM`/`DBOLSM` default is
    /// `5 * max(row_count, variable_count)`. A supplied limit is translated
    /// through a private, validated option array. Reaching it returns an
    /// approximate solution with [`BoundedLeastSquaresStatus::MaximumIterations`].
    pub maximum_iterations: Option<usize>,
}

/// Immutable caller-owned dense bounded least-squares problem.
///
/// `matrix` describes `A` and `rhs` describes `b`. The matrix must use the
/// checked column-major [`MatrixRef`] layout. `bounds` contains exactly one
/// [`VariableBounds`] value for each matrix column.
#[derive(Clone, Copy, Debug)]
pub struct BoundedLeastSquaresProblem<'a, T> {
    /// Coefficient matrix `A` in column-major storage.
    pub matrix: MatrixRef<'a, T>,
    /// Right-hand side vector `b`, with one entry per matrix row.
    pub rhs: &'a [T],
    /// Per-variable closed bound selection in caller column order.
    pub bounds: &'a [VariableBounds<T>],
}

/// Completion state reported by a bounded least-squares driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedLeastSquaresStatus {
    /// The native active-set process completed normally (`MODE >= 0`).
    ///
    /// Although positive native `MODE` contains a count related to its private
    /// active-set representation, the driver does not return a stable mapping
    /// to the caller's variables, so that count is intentionally not exposed.
    Converged,
    /// The requested or native-default drop/add iteration limit was reached
    /// (`MODE = -22`). The returned solution is an approximate bounded fit.
    MaximumIterations,
}

/// Result of a bounded linear least-squares solve.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundedLeastSquaresResult<T> {
    /// Estimated variables in the same order as the caller's matrix columns
    /// and [`BoundedLeastSquaresProblem::bounds`] slice.
    pub solution: Vec<T>,
    /// Native `RNORM`: the minimum Euclidean residual length `||A x - b||₂`.
    ///
    /// It is not an RMS residual and it is not squared.
    pub residual_norm: T,
    /// Native completion state after safe status mapping.
    pub status: BoundedLeastSquaresStatus,
}

/// Validation or contained native-contract failure from a bounded
/// least-squares solve.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedLeastSquaresError {
    /// The matrix has zero columns.
    EmptyVariables,
    /// The matrix has zero rows.
    EmptyRows,
    /// A vector length or matrix dimension disagrees with the problem model.
    DimensionMismatch {
        /// The invalid argument or dimension relationship.
        argument: &'static str,
    },
    /// A matrix, right-hand-side, or finite bound value was NaN or infinite.
    NonFiniteInput {
        /// Input collection or field containing the non-finite value.
        argument: &'static str,
        /// Zero-based index in the named collection.
        index: usize,
    },
    /// A two-sided interval had `lower > upper`.
    InconsistentBounds {
        /// Zero-based variable index.
        index: usize,
    },
    /// A requested native iteration limit was zero.
    InvalidIterationLimit,
    /// A count could not be represented by the validated GNU Fortran
    /// `INTEGER` type.
    IntegerOverflow {
        /// Count or dimension that did not fit native `INTEGER`.
        argument: &'static str,
    },
    /// Checked augmented-storage or workspace arithmetic overflowed `usize`.
    WorkspaceOverflow,
    /// A safe-precondition or internal-storage failure nevertheless reached
    /// the native driver.
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

impl fmt::Display for BoundedLeastSquaresError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariables => write!(formatter, "bounded least squares needs variables"),
            Self::EmptyRows => write!(formatter, "bounded least squares needs matrix rows"),
            Self::DimensionMismatch { argument } => {
                write!(
                    formatter,
                    "bounded least-squares dimension mismatch for {argument}"
                )
            }
            Self::NonFiniteInput { argument, index } => write!(
                formatter,
                "bounded least-squares {argument} at index {index} must be finite"
            ),
            Self::InconsistentBounds { index } => {
                write!(
                    formatter,
                    "bounded least-squares bounds at index {index} are inconsistent"
                )
            }
            Self::InvalidIterationLimit => {
                write!(
                    formatter,
                    "bounded least-squares iteration limit must be positive"
                )
            }
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "bounded least-squares {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => {
                write!(
                    formatter,
                    "bounded least-squares workspace arithmetic overflowed"
                )
            }
            Self::NativeContractViolation { detail } => write!(
                formatter,
                "native bounded least-squares contract was violated: {detail}"
            ),
            Self::NativeStatus { status } => {
                write!(formatter, "unknown SBOLS/DBOLS MODE value {status}")
            }
        }
    }
}

impl std::error::Error for BoundedLeastSquaresError {}

/// Solves a double-precision bounded linear least-squares problem.
///
/// Wraps the original SLATEC `DBOLS` driver (double precision, constrained
/// linear least-squares family). It solves `min ||A x - b||₂` with each
/// component constrained by [`VariableBounds`]. Bounds are closed; use
/// [`VariableBounds::Unbounded`] instead of Rust infinities. `Fixed(value)`
/// is represented by equal native lower and upper bounds.
///
/// `matrix` and `rhs` correspond to native `W(:, 1:NCOLS)` and
/// `W(:, NCOLS + 1)`. This wrapper creates that mutable column-major
/// augmented matrix internally because `DBOLS` overwrites it. It also owns
/// `BL`, `BU`, `IND`, `IOPT`, `X`, `RW[5*N]`, and `IW[2*N]` with checked
/// allocation arithmetic. The only public option controls map to the
/// reviewed column-scaling and iteration-limit selectors.
///
/// Calls serialize the GNU MinGW native runtime because `DBOLS` and `DBOLSM`
/// use saved state, machine constants, and the legacy error system. A scoped
/// error control permits the documented `MODE = -22` iteration-limit return
/// and restores the prior XERROR setting before Rust regains control. The API
/// requires `std`, `alloc`, `least-squares-linear-bounded`, a selected native
/// backend, and the validated `x86_64-w64-mingw32` GNU Fortran profile; it is
/// not a bare-metal claim.
///
/// # Errors
///
/// Returns errors for empty or mismatched inputs, non-finite data, inconsistent
/// two-sided bounds, zero/overflowing iteration limits, workspace overflow, or
/// an unexpected native status. The ordinary native invalid-input paths are
/// rejected before the FFI call. A maximum iteration limit remains a successful
/// result carrying [`BoundedLeastSquaresStatus::MaximumIterations`].
///
/// # Numerical considerations
///
/// The driver supplies no reviewed numerical-rank output. Rank-deficient
/// inputs may still return a bounded minimizer, but it need not be unique.
/// This function does not implement general linear equality constraints,
/// arbitrary inequalities, or linear programming.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-bounded", feature = "source-build"))]
/// # {
/// use slatec::bounded_least_squares::{
///     solve_bounded_least_squares, BoundedLeastSquaresOptions,
///     BoundedLeastSquaresProblem, VariableBounds,
/// };
/// use slatec::linear_least_squares::MatrixRef;
/// let matrix = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
/// let fit = solve_bounded_least_squares(
///     BoundedLeastSquaresProblem {
///         matrix,
///         rhs: &[-1.0, 2.0],
///         bounds: &[VariableBounds::Lower(0.0), VariableBounds::Unbounded],
///     },
///     BoundedLeastSquaresOptions::default(),
/// )?;
/// assert!((fit.solution[0] - 0.0).abs() < 1e-12);
/// assert!((fit.solution[1] - 2.0).abs() < 1e-12);
/// # Ok::<(), slatec::bounded_least_squares::BoundedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_bounded_least_squares(
    problem: BoundedLeastSquaresProblem<'_, f64>,
    options: BoundedLeastSquaresOptions,
) -> Result<BoundedLeastSquaresResult<f64>, BoundedLeastSquaresError> {
    run_f64(prepare(problem, options)?)
}

/// Solves the single-precision counterpart of
/// [`solve_bounded_least_squares`].
///
/// Wraps SLATEC `SBOLS` with the same bound encoding, owned augmented storage,
/// options policy, runtime serialization, and error handling as the
/// double-precision wrapper. Single precision has less room for ill-conditioned
/// designs and near-active bounds.
///
/// # Errors
///
/// Returns the same validation and native-contract errors as the
/// double-precision wrapper.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-bounded", feature = "source-build"))]
/// # {
/// use slatec::bounded_least_squares::{
///     solve_bounded_least_squares_f32, BoundedLeastSquaresOptions,
///     BoundedLeastSquaresProblem, VariableBounds,
/// };
/// use slatec::linear_least_squares::MatrixRef;
/// let matrix = MatrixRef::column_major(&[1.0_f32], 1, 1, 1)?;
/// let fit = solve_bounded_least_squares_f32(
///     BoundedLeastSquaresProblem {
///         matrix,
///         rhs: &[3.0],
///         bounds: &[VariableBounds::Upper(2.0)],
///     },
///     BoundedLeastSquaresOptions::default(),
/// )?;
/// assert!((fit.solution[0] - 2.0).abs() < 1e-4);
/// # Ok::<(), slatec::bounded_least_squares::BoundedLeastSquaresError>(())
/// # }
/// ```
pub fn solve_bounded_least_squares_f32(
    problem: BoundedLeastSquaresProblem<'_, f32>,
    options: BoundedLeastSquaresOptions,
) -> Result<BoundedLeastSquaresResult<f32>, BoundedLeastSquaresError> {
    run_f32(prepare(problem, options)?)
}

struct Prepared<'a, T> {
    augmented: Vec<T>,
    lower: Vec<T>,
    upper: Vec<T>,
    bound_types: Vec<FortranInteger>,
    options: Vec<FortranInteger>,
    rows: usize,
    variables: usize,
    real_workspace: usize,
    integer_workspace: usize,
    _problem: BoundedLeastSquaresProblem<'a, T>,
}

fn prepare<T: Copy + Default + Finite + PartialOrd>(
    problem: BoundedLeastSquaresProblem<'_, T>,
    options: BoundedLeastSquaresOptions,
) -> Result<Prepared<'_, T>, BoundedLeastSquaresError> {
    let rows = problem.matrix.rows();
    let variables = problem.matrix.columns();
    if rows == 0 {
        return Err(BoundedLeastSquaresError::EmptyRows);
    }
    if variables == 0 {
        return Err(BoundedLeastSquaresError::EmptyVariables);
    }
    if problem.rhs.len() != rows {
        return Err(BoundedLeastSquaresError::DimensionMismatch { argument: "rhs" });
    }
    if problem.bounds.len() != variables {
        return Err(BoundedLeastSquaresError::DimensionMismatch { argument: "bounds" });
    }
    finite_slice(problem.matrix.as_column_major_slice(), "matrix")?;
    finite_slice(problem.rhs, "rhs")?;

    let (lower, upper, bound_types) = encode_bounds(problem.bounds)?;
    let columns = variables
        .checked_add(1)
        .ok_or(BoundedLeastSquaresError::WorkspaceOverflow)?;
    let augmented_len = rows
        .checked_mul(columns)
        .ok_or(BoundedLeastSquaresError::WorkspaceOverflow)?;
    let mut augmented = vec![T::default(); augmented_len];
    for column in 0..variables {
        for row in 0..rows {
            augmented[row + column * rows] = *problem
                .matrix
                .get(row, column)
                .expect("MatrixRef checked logical dimensions");
        }
    }
    for row in 0..rows {
        augmented[row + variables * rows] = problem.rhs[row];
    }
    let real_workspace = variables
        .checked_mul(5)
        .ok_or(BoundedLeastSquaresError::WorkspaceOverflow)?;
    let integer_workspace = variables
        .checked_mul(2)
        .ok_or(BoundedLeastSquaresError::WorkspaceOverflow)?;
    for (name, value) in [
        ("MDW", rows),
        ("MROWS", rows),
        ("NCOLS", variables),
        ("RW length", real_workspace),
        ("IW length", integer_workspace),
    ] {
        to_fortran_integer(value)
            .map_err(|_| BoundedLeastSquaresError::IntegerOverflow { argument: name })?;
    }
    let option_array = option_array(options)?;
    Ok(Prepared {
        augmented,
        lower,
        upper,
        bound_types,
        options: option_array,
        rows,
        variables,
        real_workspace,
        integer_workspace,
        _problem: problem,
    })
}

fn option_array(
    options: BoundedLeastSquaresOptions,
) -> Result<Vec<FortranInteger>, BoundedLeastSquaresError> {
    let scaling = match options.scaling {
        BoundedLeastSquaresScaling::Nominal => None,
        BoundedLeastSquaresScaling::EuclideanColumns => Some(2),
        BoundedLeastSquaresScaling::Identity => Some(3),
    };
    let iterations = match options.maximum_iterations {
        Some(value) => {
            if value == 0 {
                return Err(BoundedLeastSquaresError::InvalidIterationLimit);
            }
            Some(to_fortran_integer(value).map_err(|_| {
                BoundedLeastSquaresError::IntegerOverflow {
                    argument: "maximum_iterations",
                }
            })?)
        }
        None => None,
    };
    Ok(match (scaling, iterations) {
        (None, None) => vec![99],
        (Some(scale), None) => vec![3, scale, 99],
        (None, Some(limit)) => vec![5, 4, 99, 4, limit, 99],
        (Some(scale), Some(limit)) => vec![3, scale, 5, 6, 99, 4, limit, 99],
    })
}

fn encode_bounds<T: Copy + Default + Finite + PartialOrd>(
    bounds: &[VariableBounds<T>],
) -> Result<(Vec<T>, Vec<T>, Vec<FortranInteger>), BoundedLeastSquaresError> {
    let mut lower = vec![T::default(); bounds.len()];
    let mut upper = vec![T::default(); bounds.len()];
    let mut types = vec![0; bounds.len()];
    for (index, bound) in bounds.iter().copied().enumerate() {
        match bound {
            VariableBounds::Unbounded => types[index] = 4,
            VariableBounds::Lower(value) => {
                finite(value, "lower bound", index)?;
                lower[index] = value;
                types[index] = 1;
            }
            VariableBounds::Upper(value) => {
                finite(value, "upper bound", index)?;
                upper[index] = value;
                types[index] = 2;
            }
            VariableBounds::Between {
                lower: lo,
                upper: hi,
            } => {
                finite(lo, "lower bound", index)?;
                finite(hi, "upper bound", index)?;
                if lo > hi {
                    return Err(BoundedLeastSquaresError::InconsistentBounds { index });
                }
                lower[index] = lo;
                upper[index] = hi;
                types[index] = 3;
            }
            VariableBounds::Fixed(value) => {
                finite(value, "fixed bound", index)?;
                lower[index] = value;
                upper[index] = value;
                types[index] = 3;
            }
        }
    }
    Ok((lower, upper, types))
}

fn run_f64(
    mut prepared: Prepared<'_, f64>,
) -> Result<BoundedLeastSquaresResult<f64>, BoundedLeastSquaresError> {
    let mut solution = vec![0.0; prepared.variables];
    let mut real_workspace = vec![0.0; prepared.real_workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    let mut leading_dimension = native_integer("MDW", prepared.rows)?;
    let mut rows = native_integer("MROWS", prepared.rows)?;
    let mut variables = native_integer("NCOLS", prepared.variables)?;
    let mut norm = 0.0;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_native_statuses();
    // SAFETY: all scalar dimensions fit the reviewed GNU Fortran INTEGER ABI;
    // W(MDW,NCOLS+1), bound arrays, IOPT, X, RW=5*NCOLS, and IW=2*NCOLS are
    // owned, initialized, correctly sized, and nonaliasing. The held runtime
    // lock and scoped XERROR policy cover saved native state and documented
    // MODE=-22 recovery.
    unsafe {
        slatec_sys::linear_least_squares::dbols(
            prepared.augmented.as_mut_ptr(),
            &mut leading_dimension,
            &mut rows,
            &mut variables,
            prepared.lower.as_mut_ptr(),
            prepared.upper.as_mut_ptr(),
            prepared.bound_types.as_mut_ptr(),
            prepared.options.as_mut_ptr(),
            solution.as_mut_ptr(),
            &mut norm,
            &mut mode,
            real_workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(solution, norm, mode)
}

fn run_f32(
    mut prepared: Prepared<'_, f32>,
) -> Result<BoundedLeastSquaresResult<f32>, BoundedLeastSquaresError> {
    let mut solution = vec![0.0_f32; prepared.variables];
    let mut real_workspace = vec![0.0_f32; prepared.real_workspace];
    let mut integer_workspace = vec![0; prepared.integer_workspace];
    let mut leading_dimension = native_integer("MDW", prepared.rows)?;
    let mut rows = native_integer("MROWS", prepared.rows)?;
    let mut variables = native_integer("NCOLS", prepared.variables)?;
    let mut norm = 0.0_f32;
    let mut mode = 0;
    let _lock = crate::runtime::lock_native();
    let _errors = crate::runtime::permit_recoverable_native_statuses();
    // SAFETY: f32 counterpart of run_f64. All reviewed native arrays are
    // owned, initialized, correctly sized, nonaliasing, and protected by the
    // process-global runtime/XERROR scope.
    unsafe {
        slatec_sys::linear_least_squares::sbols(
            prepared.augmented.as_mut_ptr(),
            &mut leading_dimension,
            &mut rows,
            &mut variables,
            prepared.lower.as_mut_ptr(),
            prepared.upper.as_mut_ptr(),
            prepared.bound_types.as_mut_ptr(),
            prepared.options.as_mut_ptr(),
            solution.as_mut_ptr(),
            &mut norm,
            &mut mode,
            real_workspace.as_mut_ptr(),
            integer_workspace.as_mut_ptr(),
        )
    };
    finish(solution, norm, mode)
}

fn native_integer(
    argument: &'static str,
    value: usize,
) -> Result<FortranInteger, BoundedLeastSquaresError> {
    to_fortran_integer(value).map_err(|_| BoundedLeastSquaresError::IntegerOverflow { argument })
}

fn finish<T: Copy + Finite>(
    solution: Vec<T>,
    residual_norm: T,
    mode: FortranInteger,
) -> Result<BoundedLeastSquaresResult<T>, BoundedLeastSquaresError> {
    finite_slice(&solution, "native solution")?;
    finite(residual_norm, "native residual norm", 0)?;
    let status = match mode {
        value if value >= 0 => BoundedLeastSquaresStatus::Converged,
        -22 => BoundedLeastSquaresStatus::MaximumIterations,
        value @ (-38..=-2) => {
            return Err(BoundedLeastSquaresError::NativeContractViolation {
                detail: native_contract_detail(value),
            });
        }
        value => return Err(BoundedLeastSquaresError::NativeStatus { status: value }),
    };
    Ok(BoundedLeastSquaresResult {
        solution,
        residual_norm,
        status,
    })
}

fn native_contract_detail(mode: FortranInteger) -> &'static str {
    match mode {
        -17..=-2 => "SBOLS/DBOLS rejected safe dimensions, bounds, or option storage",
        -38..=-23 => {
            "SBOLSM/DBOLSM rejected an internally constructed option or workspace contract"
        }
        _ => "SBOLS/DBOLS returned an unexpected contract status",
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
) -> Result<(), BoundedLeastSquaresError> {
    if value.finite() {
        Ok(())
    } else {
        Err(BoundedLeastSquaresError::NonFiniteInput { argument, index })
    }
}

fn finite_slice<T: Copy + Finite>(
    data: &[T],
    argument: &'static str,
) -> Result<(), BoundedLeastSquaresError> {
    data.iter()
        .copied()
        .enumerate()
        .find(|(_, value)| !value.finite())
        .map_or(Ok(()), |(index, _)| {
            Err(BoundedLeastSquaresError::NonFiniteInput { argument, index })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_every_bound_variant() {
        let (lower, upper, kinds) = encode_bounds(&[
            VariableBounds::Unbounded,
            VariableBounds::Lower(1.0_f64),
            VariableBounds::Upper(2.0),
            VariableBounds::Between {
                lower: -1.0,
                upper: 3.0,
            },
            VariableBounds::Fixed(4.0),
        ])
        .unwrap();
        assert_eq!(kinds, vec![4, 1, 2, 3, 3]);
        assert_eq!(lower, vec![0.0, 1.0, 0.0, -1.0, 4.0]);
        assert_eq!(upper, vec![0.0, 0.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn options_keep_raw_protocol_private() {
        assert_eq!(
            option_array(BoundedLeastSquaresOptions::default()).unwrap(),
            vec![99]
        );
        assert_eq!(
            option_array(BoundedLeastSquaresOptions {
                scaling: BoundedLeastSquaresScaling::Identity,
                maximum_iterations: Some(7),
            })
            .unwrap(),
            vec![3, 3, 5, 6, 99, 4, 7, 99]
        );
        assert!(matches!(
            option_array(BoundedLeastSquaresOptions {
                scaling: BoundedLeastSquaresScaling::Nominal,
                maximum_iterations: Some(0),
            }),
            Err(BoundedLeastSquaresError::InvalidIterationLimit)
        ));
    }

    #[test]
    fn augmented_storage_is_owned_column_major() {
        let matrix = MatrixRef::column_major(&[1.0_f64, 3.0, 2.0, 4.0], 2, 2, 2).unwrap();
        let prepared = prepare(
            BoundedLeastSquaresProblem {
                matrix,
                rhs: &[5.0, 6.0],
                bounds: &[VariableBounds::Unbounded, VariableBounds::Lower(0.0)],
            },
            BoundedLeastSquaresOptions::default(),
        )
        .unwrap();
        assert_eq!(prepared.augmented, vec![1.0, 3.0, 2.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn inconsistent_interval_is_rejected_before_native_execution() {
        assert!(matches!(
            encode_bounds(&[VariableBounds::<f64>::Between {
                lower: 2.0,
                upper: 1.0,
            }]),
            Err(BoundedLeastSquaresError::InconsistentBounds { index: 0 })
        ));
    }
}
