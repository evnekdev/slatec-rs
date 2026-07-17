//! Shared dense matrix views and safe weighted nonnegative linear
//! least-squares fitting.
//!
//! This module wraps SLATEC `DWNNLS` and `WNNLS`. Despite their historical
//! name, the routines do **not** accept caller-supplied statistical weights.
//! They solve `min ||A x - b||₂` while treating a separate block `E x = f`
//! as exact equality equations and constraining selected variables to be
//! nonnegative. The original routines require all free variables before all
//! constrained variables; this facade applies a private stable permutation and
//! restores the caller's variable order in its result.

#[cfg(feature = "least-squares-linear-nonnegative")]
use alloc::{vec, vec::Vec};
use core::fmt;

#[cfg(feature = "least-squares-linear-nonnegative")]
use slatec_core::to_fortran_integer;
#[cfg(feature = "least-squares-linear-nonnegative")]
use slatec_sys::FortranInteger;

/// Storage order accepted by [`MatrixRef`].
///
/// `WNNLS` and `DWNNLS` use Fortran column-major arrays. The safe facade
/// intentionally accepts only this layout, avoiding an ambiguous row-major
/// conversion policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixLayout {
    /// Fortran column-major storage: `(row, column)` is at
    /// `row + column * leading_dimension`.
    ColumnMajor,
}

/// Borrowed checked dense matrix view used by a nonnegative least-squares
/// problem.
///
/// The view never grants native code mutable access to its slice. The wrapper
/// copies it into a separate augmented array because `WNNLS`/`DWNNLS` overwrite
/// that native storage.
#[derive(Clone, Copy, Debug)]
pub struct MatrixRef<'a, T> {
    data: &'a [T],
    rows: usize,
    columns: usize,
    leading_dimension: usize,
    layout: MatrixLayout,
}

impl<'a, T> MatrixRef<'a, T> {
    /// Creates a checked column-major matrix view.
    ///
    /// `data` must contain at least `leading_dimension * columns` entries and
    /// `leading_dimension` must be at least `rows`. The physical storage may
    /// contain padding at the end of every column; padding is not treated as a
    /// logical matrix element.
    ///
    /// # Errors
    ///
    /// Returns [`LinearLeastSquaresError::DimensionMismatch`] for an invalid
    /// leading dimension or insufficient physical storage, and
    /// [`LinearLeastSquaresError::WorkspaceOverflow`] if the checked storage
    /// multiplication overflows `usize`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use slatec::linear_least_squares::MatrixRef;
    /// let matrix = MatrixRef::column_major(&[1.0_f64, 2.0, 3.0, 4.0], 2, 2, 2)?;
    /// assert_eq!(matrix.get(1, 0), Some(&2.0));
    /// # Ok::<(), slatec::linear_least_squares::LinearLeastSquaresError>(())
    /// ```
    pub fn column_major(
        data: &'a [T],
        rows: usize,
        columns: usize,
        leading_dimension: usize,
    ) -> Result<Self, LinearLeastSquaresError> {
        if leading_dimension < rows {
            return Err(LinearLeastSquaresError::DimensionMismatch {
                argument: "matrix leading_dimension",
            });
        }
        let needed = leading_dimension
            .checked_mul(columns)
            .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?;
        if data.len() < needed {
            return Err(LinearLeastSquaresError::DimensionMismatch {
                argument: "matrix physical storage",
            });
        }
        Ok(Self {
            data,
            rows,
            columns,
            leading_dimension,
            layout: MatrixLayout::ColumnMajor,
        })
    }

    /// Returns the logical row count.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the logical column count.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the physical Fortran column stride.
    pub const fn leading_dimension(&self) -> usize {
        self.leading_dimension
    }

    /// Returns the storage layout, currently always column-major.
    pub const fn layout(&self) -> MatrixLayout {
        self.layout
    }

    /// Returns one logical zero-based matrix entry, or `None` when either
    /// index is outside the matrix dimensions.
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.data.get(
            column
                .checked_mul(self.leading_dimension)?
                .checked_add(row)?,
        )
    }

    /// Returns the physical column-major storage, including any padding.
    pub const fn as_column_major_slice(&self) -> &'a [T] {
        self.data
    }
}

/// Constraint applied to one model variable.
#[cfg(feature = "least-squares-linear-nonnegative")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VariableConstraint {
    /// The variable is unrestricted in sign.
    Free,
    /// The variable is constrained to be greater than or equal to zero.
    Nonnegative,
}

/// Typed input to [`solve_nonnegative_least_squares`] or
/// [`solve_nonnegative_least_squares_f32`].
///
/// `least_squares_matrix` and `least_squares_rhs` describe `A x ≈ b`.
/// `equality_matrix` and `equality_rhs`, when both supplied, describe exact
/// equations `E x = f`. All matrices use [`MatrixRef`] column-major storage.
/// `variable_constraints` has exactly one entry per column/variable.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "least-squares-linear-nonnegative")]
pub struct NonnegativeLeastSquaresProblem<'a, T> {
    /// Coefficient matrix `A` for the least-squares rows.
    pub least_squares_matrix: MatrixRef<'a, T>,
    /// Right-hand side `b` for the least-squares rows.
    pub least_squares_rhs: &'a [T],
    /// Optional equality coefficient matrix `E`.
    pub equality_matrix: Option<MatrixRef<'a, T>>,
    /// Optional equality right-hand side `f`; present exactly when
    /// [`Self::equality_matrix`] is present.
    pub equality_rhs: Option<&'a [T]>,
    /// Free/nonnegative selection in the caller's parameter order.
    pub variable_constraints: &'a [VariableConstraint],
}

/// Reviewed native completion state from `WNNLS` or `DWNNLS`.
#[cfg(feature = "least-squares-linear-nonnegative")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonnegativeLeastSquaresStatus {
    /// The native active-set iteration completed normally (`MODE = 0`).
    Converged,
    /// Native active-set iteration reached its documented `3 * (N - L)`
    /// bound (`MODE = 1`); the returned solution is still available.
    MaximumIterations,
}

/// Result of a safe weighted nonnegative least-squares solve.
///
/// All vectors use the caller's original variable order. The separate norms
/// are recomputed against the immutable caller inputs; `native_residual_norm`
/// is the original SLATEC `RNORM` output and can include the equality block.
#[derive(Clone, Debug, PartialEq)]
#[cfg(feature = "least-squares-linear-nonnegative")]
pub struct NonnegativeLeastSquaresResult<T> {
    /// Estimated variables in the caller's original order.
    pub solution: Vec<T>,
    /// Original SLATEC `RNORM` output.
    pub native_residual_norm: T,
    /// `||A x - b||₂`, recomputed from the original least-squares block.
    pub least_squares_residual_norm: T,
    /// `||E x - f||₂`, zero when no equality block was supplied.
    pub equality_residual_norm: T,
    /// Reviewed native completion state.
    pub status: NonnegativeLeastSquaresStatus,
}

/// Validation or native-contract failure from a nonnegative least-squares
/// operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinearLeastSquaresError {
    /// The problem has no variables.
    EmptyVariables,
    /// Neither equality nor least-squares rows were supplied.
    EmptyRows,
    /// A matrix, right-hand-side, leading-dimension, or constraint-vector
    /// shape did not match the problem dimensions.
    DimensionMismatch {
        /// The invalid argument or relation.
        argument: &'static str,
    },
    /// An input scalar was NaN or infinite.
    NonFiniteInput {
        /// Input collection or field containing the bad value.
        argument: &'static str,
        /// Zero-based physical/logical index in that collection.
        index: usize,
    },
    /// A count cannot be represented by the validated GNU Fortran `INTEGER`.
    IntegerOverflow {
        /// Count or dimension that did not fit.
        argument: &'static str,
    },
    /// Checked augmented-array or native-workspace arithmetic overflowed.
    WorkspaceOverflow,
    /// Native `MODE = 2` reported an input/workspace contract failure that the
    /// safe prechecks should have prevented.
    NativeContractViolation {
        /// Stable explanation of the violated contract.
        detail: &'static str,
    },
    /// A native completion code outside the reviewed `MODE = 0, 1, 2`
    /// contract was observed.
    NativeStatus {
        /// Raw SLATEC `MODE` value.
        status: i32,
    },
}

impl fmt::Display for LinearLeastSquaresError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariables => write!(formatter, "nonnegative least squares needs variables"),
            Self::EmptyRows => write!(
                formatter,
                "nonnegative least squares needs equality or least-squares rows"
            ),
            Self::DimensionMismatch { argument } => write!(
                formatter,
                "nonnegative least-squares dimension mismatch for {argument}"
            ),
            Self::NonFiniteInput { argument, index } => write!(
                formatter,
                "nonnegative least-squares {argument} at index {index} must be finite"
            ),
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "nonnegative least-squares {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => write!(
                formatter,
                "nonnegative least-squares workspace arithmetic overflowed"
            ),
            Self::NativeContractViolation { detail } => write!(
                formatter,
                "native nonnegative least-squares contract was violated: {detail}"
            ),
            Self::NativeStatus { status } => {
                write!(formatter, "unknown WNNLS/DWNNLS MODE value {status}")
            }
        }
    }
}

impl std::error::Error for LinearLeastSquaresError {}

/// Solves a double-precision equality-constrained nonnegative linear
/// least-squares problem.
///
/// Wraps the SLATEC routine `DWNNLS` (double precision, SLATEC constrained
/// linear least-squares family). It minimizes `||A x - b||₂`, enforces the
/// optional equality block `E x = f` through the original routine's internal
/// equality weighting, and requires only variables marked
/// [`VariableConstraint::Nonnegative`] to satisfy `x >= 0`.
///
/// The arguments map to native `W(MDW,N+1)`, `ME`, `MA`, `N`, and `L`: this
/// wrapper builds mutable `W` internally, puts equality rows before
/// least-squares rows, privately permutes free variables before nonnegative
/// variables, and passes no user program options (`PRGOPT(1)=1`). It allocates
/// the exact documented `WORK[ME+MA+5*N]` and `IWORK[ME+MA+N]` arrays.
///
/// Calls serialize the GNU MinGW native runtime because `DWNNLS` reaches
/// saved machine-constant and legacy-error support. The API requires `std`,
/// `alloc`, `least-squares-linear-nonnegative`, a selected native backend,
/// and the validated `x86_64-w64-mingw32` GNU Fortran profile; it is not a
/// bare-metal claim. It does not expose general bounds, linear-programming
/// dual variables, rank, or infeasibility because `DWNNLS` has no reviewed
/// structured output for them.
///
/// # Errors
///
/// Returns validation errors for empty/mismatched/non-finite input, checked
/// integer/workspace overflow, or an impossible native contract status.
/// `MODE = 1` is retained as [`NonnegativeLeastSquaresStatus::MaximumIterations`]
/// with its approximate solution.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-nonnegative", feature = "source-build"))]
/// # {
/// use slatec::linear_least_squares::{MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint, solve_nonnegative_least_squares};
/// let a = MatrixRef::column_major(&[1.0_f64, 1.0, 1.0, -1.0], 2, 2, 2)?;
/// let fit = solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
///     least_squares_matrix: a, least_squares_rhs: &[1.0, 0.0],
///     equality_matrix: None, equality_rhs: None,
///     variable_constraints: &[VariableConstraint::Nonnegative, VariableConstraint::Nonnegative],
/// })?;
/// assert!(fit.solution.iter().all(|value| *value >= -1e-12));
/// # Ok::<(), slatec::linear_least_squares::LinearLeastSquaresError>(())
/// # }
/// ```
#[cfg(feature = "least-squares-linear-nonnegative")]
pub fn solve_nonnegative_least_squares(
    problem: NonnegativeLeastSquaresProblem<'_, f64>,
) -> Result<NonnegativeLeastSquaresResult<f64>, LinearLeastSquaresError> {
    let prepared = PreparedF64::new(problem)?;
    run_f64(prepared)
}

/// Solves the single-precision counterpart of
/// [`solve_nonnegative_least_squares`].
///
/// Wraps SLATEC `WNNLS` using the same `E x = f`, `A x ≈ b`, stable variable
/// permutation, exact native workspace formulas, profile requirement, and
/// error policy. Single precision has materially less room for active-set and
/// equality residual tolerances than the `DWNNLS` wrapper.
///
/// # Errors
///
/// Returns the same validation and native-contract errors as the
/// double-precision wrapper.
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(feature = "least-squares-linear-nonnegative", feature = "source-build"))]
/// # {
/// use slatec::linear_least_squares::{MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint, solve_nonnegative_least_squares_f32};
/// let a = MatrixRef::column_major(&[1.0_f32, 1.0], 2, 1, 2)?;
/// let fit = solve_nonnegative_least_squares_f32(NonnegativeLeastSquaresProblem {
///     least_squares_matrix: a, least_squares_rhs: &[1.0, 1.0],
///     equality_matrix: None, equality_rhs: None,
///     variable_constraints: &[VariableConstraint::Nonnegative],
/// })?;
/// assert!((fit.solution[0] - 1.0).abs() < 1e-4);
/// # Ok::<(), slatec::linear_least_squares::LinearLeastSquaresError>(())
/// # }
/// ```
#[cfg(feature = "least-squares-linear-nonnegative")]
pub fn solve_nonnegative_least_squares_f32(
    problem: NonnegativeLeastSquaresProblem<'_, f32>,
) -> Result<NonnegativeLeastSquaresResult<f32>, LinearLeastSquaresError> {
    let prepared = PreparedF32::new(problem)?;
    run_f32(prepared)
}

#[cfg(feature = "least-squares-linear-nonnegative")]
struct PreparedF64<'a> {
    problem: NonnegativeLeastSquaresProblem<'a, f64>,
    native_to_user: Vec<usize>,
    augmented: Vec<f64>,
    mdw: usize,
    me: usize,
    ma: usize,
    free: usize,
    work: usize,
    iwork: usize,
}
#[cfg(feature = "least-squares-linear-nonnegative")]
struct PreparedF32<'a> {
    problem: NonnegativeLeastSquaresProblem<'a, f32>,
    native_to_user: Vec<usize>,
    augmented: Vec<f32>,
    mdw: usize,
    me: usize,
    ma: usize,
    free: usize,
    work: usize,
    iwork: usize,
}

#[cfg(feature = "least-squares-linear-nonnegative")]
macro_rules! impl_prepared {
    ($name:ident, $scalar:ty) => {
        impl<'a> $name<'a> {
            fn new(
                problem: NonnegativeLeastSquaresProblem<'a, $scalar>,
            ) -> Result<Self, LinearLeastSquaresError> {
                let (me, ma, n) = validate_problem(&problem)?;
                let native_to_user = native_to_user(problem.variable_constraints);
                let mdw = me
                    .checked_add(ma)
                    .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?;
                let augmented_len = mdw
                    .checked_mul(
                        n.checked_add(1)
                            .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?,
                    )
                    .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?;
                let work = mdw
                    .checked_add(
                        n.checked_mul(5)
                            .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?,
                    )
                    .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?;
                let iwork = mdw
                    .checked_add(n)
                    .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?;
                let mut augmented = vec![0.0 as $scalar; augmented_len];
                fill_augmented(&problem, &native_to_user, &mut augmented, mdw, me, ma);
                for (name, value) in [
                    ("MDW", mdw),
                    ("ME", me),
                    ("MA", ma),
                    ("N", n),
                    (
                        "L",
                        native_to_user
                            .iter()
                            .take_while(|&&index| {
                                problem.variable_constraints[index] == VariableConstraint::Free
                            })
                            .count(),
                    ),
                    ("WORK length", work),
                    ("IWORK length", iwork),
                ] {
                    to_fortran_integer(value)
                        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: name })?;
                }
                let free = native_to_user
                    .iter()
                    .take_while(|&&index| {
                        problem.variable_constraints[index] == VariableConstraint::Free
                    })
                    .count();
                Ok(Self {
                    problem,
                    native_to_user,
                    augmented,
                    mdw,
                    me,
                    ma,
                    free,
                    work,
                    iwork,
                })
            }
        }
    };
}
#[cfg(feature = "least-squares-linear-nonnegative")]
impl_prepared!(PreparedF64, f64);
#[cfg(feature = "least-squares-linear-nonnegative")]
impl_prepared!(PreparedF32, f32);

#[cfg(feature = "least-squares-linear-nonnegative")]
fn validate_problem<T: Copy + Finite>(
    problem: &NonnegativeLeastSquaresProblem<'_, T>,
) -> Result<(usize, usize, usize), LinearLeastSquaresError> {
    let n = problem.least_squares_matrix.columns();
    if n == 0 {
        return Err(LinearLeastSquaresError::EmptyVariables);
    }
    let ma = problem.least_squares_matrix.rows();
    if problem.least_squares_rhs.len() != ma {
        return Err(LinearLeastSquaresError::DimensionMismatch {
            argument: "least_squares_rhs",
        });
    }
    if problem.variable_constraints.len() != n {
        return Err(LinearLeastSquaresError::DimensionMismatch {
            argument: "variable_constraints",
        });
    }
    let (me, equality, equality_rhs) = match (problem.equality_matrix, problem.equality_rhs) {
        (Some(matrix), Some(rhs)) => (matrix.rows(), Some(matrix), rhs),
        (None, None) => (0, None, &[] as &[T]),
        _ => {
            return Err(LinearLeastSquaresError::DimensionMismatch {
                argument: "equality_matrix/equality_rhs",
            });
        }
    };
    if ma
        .checked_add(me)
        .ok_or(LinearLeastSquaresError::WorkspaceOverflow)?
        == 0
    {
        return Err(LinearLeastSquaresError::EmptyRows);
    }
    if let Some(matrix) = equality {
        if matrix.columns() != n {
            return Err(LinearLeastSquaresError::DimensionMismatch {
                argument: "equality_matrix columns",
            });
        }
        if equality_rhs.len() != me {
            return Err(LinearLeastSquaresError::DimensionMismatch {
                argument: "equality_rhs",
            });
        }
    }
    finite_slice(
        problem.least_squares_matrix.as_column_major_slice(),
        "least_squares_matrix",
    )?;
    finite_slice(problem.least_squares_rhs, "least_squares_rhs")?;
    if let Some(matrix) = equality {
        finite_slice(matrix.as_column_major_slice(), "equality_matrix")?;
    }
    finite_slice(equality_rhs, "equality_rhs")?;
    Ok((me, ma, n))
}

#[cfg(feature = "least-squares-linear-nonnegative")]
trait Finite {
    fn finite(self) -> bool;
}
#[cfg(feature = "least-squares-linear-nonnegative")]
impl Finite for f64 {
    fn finite(self) -> bool {
        self.is_finite()
    }
}
#[cfg(feature = "least-squares-linear-nonnegative")]
impl Finite for f32 {
    fn finite(self) -> bool {
        self.is_finite()
    }
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn finite_slice<T: Copy + Finite>(
    data: &[T],
    argument: &'static str,
) -> Result<(), LinearLeastSquaresError> {
    data.iter()
        .copied()
        .enumerate()
        .find(|(_, value)| !value.finite())
        .map_or(Ok(()), |(index, _)| {
            Err(LinearLeastSquaresError::NonFiniteInput { argument, index })
        })
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn native_to_user(constraints: &[VariableConstraint]) -> Vec<usize> {
    constraints
        .iter()
        .enumerate()
        .filter_map(|(index, constraint)| {
            (*constraint == VariableConstraint::Free).then_some(index)
        })
        .chain(
            constraints
                .iter()
                .enumerate()
                .filter_map(|(index, constraint)| {
                    (*constraint == VariableConstraint::Nonnegative).then_some(index)
                }),
        )
        .collect()
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn fill_augmented<T: Copy + Default>(
    problem: &NonnegativeLeastSquaresProblem<'_, T>,
    native_to_user: &[usize],
    augmented: &mut [T],
    mdw: usize,
    me: usize,
    ma: usize,
) {
    if let (Some(equality), Some(rhs)) = (problem.equality_matrix, problem.equality_rhs) {
        for (native_column, &user_column) in native_to_user.iter().enumerate() {
            for row in 0..me {
                augmented[row + native_column * mdw] =
                    *equality.get(row, user_column).expect("validated matrix");
            }
        }
        for row in 0..me {
            augmented[row + native_to_user.len() * mdw] = rhs[row];
        }
    }
    for (native_column, &user_column) in native_to_user.iter().enumerate() {
        for row in 0..ma {
            augmented[me + row + native_column * mdw] = *problem
                .least_squares_matrix
                .get(row, user_column)
                .expect("validated matrix");
        }
    }
    for row in 0..ma {
        augmented[me + row + native_to_user.len() * mdw] = problem.least_squares_rhs[row];
    }
}

#[cfg(feature = "least-squares-linear-nonnegative")]
fn run_f64(
    mut prepared: PreparedF64<'_>,
) -> Result<NonnegativeLeastSquaresResult<f64>, LinearLeastSquaresError> {
    let n = prepared.native_to_user.len();
    let mut solution_native = vec![0.0; n];
    let mut work = vec![0.0; prepared.work];
    let mut iwork = vec![0_i32; prepared.iwork];
    iwork[0] = to_fortran_integer(prepared.work).map_err(|_| {
        LinearLeastSquaresError::IntegerOverflow {
            argument: "WORK length",
        }
    })?;
    iwork[1] = to_fortran_integer(prepared.iwork).map_err(|_| {
        LinearLeastSquaresError::IntegerOverflow {
            argument: "IWORK length",
        }
    })?;
    let mut mdw = to_fortran_integer(prepared.mdw)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "MDW" })?;
    let mut me = to_fortran_integer(prepared.me)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "ME" })?;
    let mut ma = to_fortran_integer(prepared.ma)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "MA" })?;
    let mut variables = to_fortran_integer(n)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "N" })?;
    let mut free = to_fortran_integer(prepared.free)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "L" })?;
    let mut options = [1.0];
    let mut norm = 0.0;
    let mut mode = 0;
    let _guard = crate::runtime::lock_native();
    // SAFETY: all dimensions and the exact documented W, WORK, and IWORK sizes were checked; mutable storage is owned, initialized, and nonaliasing; the process-global runtime lock covers saved machine/error state.
    unsafe {
        slatec_sys::linear_least_squares::dwnnls(
            prepared.augmented.as_mut_ptr(),
            &mut mdw,
            &mut me,
            &mut ma,
            &mut variables,
            &mut free,
            options.as_mut_ptr(),
            solution_native.as_mut_ptr(),
            &mut norm,
            &mut mode,
            iwork.as_mut_ptr(),
            work.as_mut_ptr(),
        )
    };
    finish_f64(
        prepared.problem,
        prepared.native_to_user,
        solution_native,
        norm,
        mode,
    )
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn run_f32(
    mut prepared: PreparedF32<'_>,
) -> Result<NonnegativeLeastSquaresResult<f32>, LinearLeastSquaresError> {
    let n = prepared.native_to_user.len();
    let mut solution_native = vec![0.0_f32; n];
    let mut work = vec![0.0_f32; prepared.work];
    let mut iwork = vec![0_i32; prepared.iwork];
    iwork[0] = to_fortran_integer(prepared.work).map_err(|_| {
        LinearLeastSquaresError::IntegerOverflow {
            argument: "WORK length",
        }
    })?;
    iwork[1] = to_fortran_integer(prepared.iwork).map_err(|_| {
        LinearLeastSquaresError::IntegerOverflow {
            argument: "IWORK length",
        }
    })?;
    let mut mdw = to_fortran_integer(prepared.mdw)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "MDW" })?;
    let mut me = to_fortran_integer(prepared.me)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "ME" })?;
    let mut ma = to_fortran_integer(prepared.ma)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "MA" })?;
    let mut variables = to_fortran_integer(n)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "N" })?;
    let mut free = to_fortran_integer(prepared.free)
        .map_err(|_| LinearLeastSquaresError::IntegerOverflow { argument: "L" })?;
    let mut options = [1.0_f32];
    let mut norm = 0.0_f32;
    let mut mode = 0;
    let _guard = crate::runtime::lock_native();
    // SAFETY: f32 counterpart of run_f64: all native dimensions/storage were checked, initialized, owned, nonaliasing, and held under the shared runtime lock.
    unsafe {
        slatec_sys::linear_least_squares::wnnls(
            prepared.augmented.as_mut_ptr(),
            &mut mdw,
            &mut me,
            &mut ma,
            &mut variables,
            &mut free,
            options.as_mut_ptr(),
            solution_native.as_mut_ptr(),
            &mut norm,
            &mut mode,
            iwork.as_mut_ptr(),
            work.as_mut_ptr(),
        )
    };
    finish_f32(
        prepared.problem,
        prepared.native_to_user,
        solution_native,
        norm,
        mode,
    )
}

#[cfg(feature = "least-squares-linear-nonnegative")]
fn status(mode: FortranInteger) -> Result<NonnegativeLeastSquaresStatus, LinearLeastSquaresError> {
    match mode {
        0 => Ok(NonnegativeLeastSquaresStatus::Converged),
        1 => Ok(NonnegativeLeastSquaresStatus::MaximumIterations),
        2 => Err(LinearLeastSquaresError::NativeContractViolation {
            detail: "WNNLS/DWNNLS returned MODE=2 after safe validation",
        }),
        value => Err(LinearLeastSquaresError::NativeStatus { status: value }),
    }
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn finish_f64(
    problem: NonnegativeLeastSquaresProblem<'_, f64>,
    native_to_user: Vec<usize>,
    native: Vec<f64>,
    native_residual_norm: f64,
    mode: i32,
) -> Result<NonnegativeLeastSquaresResult<f64>, LinearLeastSquaresError> {
    let solution = restore(native_to_user, native);
    let (least, equality) = norms_f64(problem, &solution);
    Ok(NonnegativeLeastSquaresResult {
        solution,
        native_residual_norm,
        least_squares_residual_norm: least,
        equality_residual_norm: equality,
        status: status(mode)?,
    })
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn finish_f32(
    problem: NonnegativeLeastSquaresProblem<'_, f32>,
    native_to_user: Vec<usize>,
    native: Vec<f32>,
    native_residual_norm: f32,
    mode: i32,
) -> Result<NonnegativeLeastSquaresResult<f32>, LinearLeastSquaresError> {
    let solution = restore(native_to_user, native);
    let (least, equality) = norms_f32(problem, &solution);
    Ok(NonnegativeLeastSquaresResult {
        solution,
        native_residual_norm,
        least_squares_residual_norm: least,
        equality_residual_norm: equality,
        status: status(mode)?,
    })
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn restore<T: Default + Copy>(native_to_user: Vec<usize>, native: Vec<T>) -> Vec<T> {
    let mut solution = vec![T::default(); native.len()];
    for (native_index, user_index) in native_to_user.into_iter().enumerate() {
        solution[user_index] = native[native_index];
    }
    solution
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn norms_f64(problem: NonnegativeLeastSquaresProblem<'_, f64>, solution: &[f64]) -> (f64, f64) {
    let least = norm_f64(
        problem.least_squares_matrix,
        problem.least_squares_rhs,
        solution,
    );
    let equality = match (problem.equality_matrix, problem.equality_rhs) {
        (Some(matrix), Some(rhs)) => norm_f64(matrix, rhs, solution),
        _ => 0.0,
    };
    (least, equality)
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn norms_f32(problem: NonnegativeLeastSquaresProblem<'_, f32>, solution: &[f32]) -> (f32, f32) {
    let least = norm_f32(
        problem.least_squares_matrix,
        problem.least_squares_rhs,
        solution,
    );
    let equality = match (problem.equality_matrix, problem.equality_rhs) {
        (Some(matrix), Some(rhs)) => norm_f32(matrix, rhs, solution),
        _ => 0.0,
    };
    (least, equality)
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn norm_f64(matrix: MatrixRef<'_, f64>, rhs: &[f64], solution: &[f64]) -> f64 {
    let mut sum = 0.0;
    for row in 0..matrix.rows() {
        let mut residual = -rhs[row];
        for column in 0..matrix.columns() {
            residual += *matrix.get(row, column).expect("validated") * solution[column];
        }
        sum += residual * residual;
    }
    sum.sqrt()
}
#[cfg(feature = "least-squares-linear-nonnegative")]
fn norm_f32(matrix: MatrixRef<'_, f32>, rhs: &[f32], solution: &[f32]) -> f32 {
    let mut sum = 0.0_f32;
    for row in 0..matrix.rows() {
        let mut residual = -rhs[row];
        for column in 0..matrix.columns() {
            residual += *matrix.get(row, column).expect("validated") * solution[column];
        }
        sum += residual * residual;
    }
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_rejects_short_storage() {
        assert!(matches!(
            MatrixRef::<f64>::column_major(&[1.0], 2, 1, 2),
            Err(LinearLeastSquaresError::DimensionMismatch { .. })
        ));
    }
    #[test]
    #[cfg(feature = "least-squares-linear-nonnegative")]
    fn transform_orders_equality_then_least_squares_and_permuted_columns() {
        let a = MatrixRef::column_major(&[1.0_f64, 3.0, 2.0, 4.0], 2, 2, 2).unwrap();
        let e = MatrixRef::column_major(&[5.0_f64, 6.0], 1, 2, 1).unwrap();
        let p = NonnegativeLeastSquaresProblem {
            least_squares_matrix: a,
            least_squares_rhs: &[7.0, 8.0],
            equality_matrix: Some(e),
            equality_rhs: Some(&[9.0]),
            variable_constraints: &[VariableConstraint::Nonnegative, VariableConstraint::Free],
        };
        let prepared = PreparedF64::new(p).unwrap();
        assert_eq!(prepared.native_to_user, vec![1, 0]);
        assert_eq!(
            prepared.augmented,
            vec![6.0, 2.0, 4.0, 5.0, 1.0, 3.0, 9.0, 7.0, 8.0]
        );
    }
    #[test]
    #[cfg(feature = "least-squares-linear-nonnegative")]
    fn invalid_constraint_count_is_prechecked() {
        let a = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
        assert!(matches!(
            solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
                least_squares_matrix: a,
                least_squares_rhs: &[1.0],
                equality_matrix: None,
                equality_rhs: None,
                variable_constraints: &[]
            }),
            Err(LinearLeastSquaresError::DimensionMismatch { .. })
        ));
    }
}
