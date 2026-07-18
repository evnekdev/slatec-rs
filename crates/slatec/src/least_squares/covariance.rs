//! Safe covariance estimation over the original `SCOV` and `DCOV` routines.
//!
//! `SCOV` and `DCOV` do not accept a saved solver factorization. They evaluate
//! the residual and Jacobian again at a supplied final parameter vector,
//! factor the fresh Jacobian without column pivoting, and produce a covariance
//! estimate only when every diagonal element of the resulting triangular
//! factor is nonzero. Consequently this module never claims a pseudoinverse
//! or an effective numerical rank for a singular problem.

use alloc::{vec, vec::Vec};
use core::fmt;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::callback_runtime::{
    self, CallbackRuntimeError, ExpertLeastSquaresCallbackFailure, ExpertLeastSquaresF32Callback,
    ExpertLeastSquaresF64Callback,
};
use crate::nonlinear::JacobianMut;

#[cfg(feature = "least-squares-nonlinear-expert")]
use super::ExpertLeastSquaresResult;
use super::LeastSquaresStatus;

/// Scaling convention applied by the covariance routine.
///
/// Original `SCOV` and `DCOV` always apply their own convention. For `M > N`
/// they multiply `(JᵀJ)⁻¹` by `RSS / (M - N)`; for the square `M = N` case
/// they instead use a scale of one. `ResidualVariance` makes the former
/// statistical convention explicit and rejects square problems whose degrees
/// of freedom would be zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CovarianceScaling {
    /// Preserve the exact native convention: `RSS / (M - N)` for `M > N`, or
    /// an unscaled `(JᵀJ)⁻¹` for `M = N`.
    Native,
    /// Require positive residual degrees of freedom and return the native
    /// `RSS / (M - N) · (JᵀJ)⁻¹` estimate.
    ResidualVariance,
}

/// Controls accepted by the covariance wrappers.
///
/// There is deliberately no rank-tolerance field: `SCOV` and `DCOV` accept no
/// such input and classify singularity only when their unpivoted triangular
/// diagonal contains an exact zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CovarianceOptions {
    /// Statistical scaling convention for the returned matrix.
    pub scaling: CovarianceScaling,
}

impl Default for CovarianceOptions {
    fn default() -> Self {
        Self {
            scaling: CovarianceScaling::ResidualVariance,
        }
    }
}

/// Eligibility policy used by [`covariance_from_expert_fit`].
///
/// The covariance routine itself recomputes residuals and a Jacobian at the
/// returned parameters. This policy only decides whether a prior `DNLS1` or
/// `SNLS1` completion status is suitable input for that fresh local estimate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CovarianceEligibility {
    /// Accept only the four native convergence statuses.
    ConvergedOnly,
    /// Also accept documented numerical-termination statuses carrying a
    /// usable final iterate. Callback failures never produce a fit result and
    /// therefore cannot be admitted by this policy.
    AllowNumericalTermination,
}

/// Native rank status represented by a returned covariance matrix.
///
/// `SCOV` and `DCOV` return a matrix only for their full-rank path. A singular
/// factorization is reported as [`CovarianceError::RankDeficient`] instead of
/// a fabricated rank-deficient covariance matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CovarianceStatus {
    /// The native routine found no exact zero on the unpivoted QR diagonal.
    FullRank,
}

/// Full symmetric covariance matrix returned by `SCOV` or `DCOV`.
///
/// `covariance` is an owned full `parameter_count × parameter_count`
/// column-major matrix. The safe layer expands the upper triangle written by
/// SLATEC by mirroring it exactly; the lower native workspace is not exposed.
#[derive(Clone, Debug, PartialEq)]
pub struct CovarianceResult<T = f64> {
    /// Full symmetric column-major matrix, with entry `(row, column)` at
    /// `row + column * parameter_count`.
    pub covariance: Vec<T>,
    /// Number of fitted parameters (`N`), and rows/columns of `covariance`.
    pub parameter_count: usize,
    /// Native full rank. A rank-deficient native call returns an error rather
    /// than an invented effective rank.
    pub rank: usize,
    /// Zero-based parameter permutation. `SCOV`/`DCOV` call QR without
    /// pivoting, so successful results always contain the identity ordering.
    pub permutation: Vec<usize>,
    /// `RSS = Σ rᵢ²` recomputed from the final residual callback evaluation.
    pub residual_sum_of_squares: T,
    /// The scalar multiplying `(JᵀJ)⁻¹` in the returned covariance matrix.
    /// It is `RSS/(M-N)` for `M > N` and one for native square problems.
    pub variance_scale: T,
    /// Native rank classification for this result.
    pub status: CovarianceStatus,
}

impl<T> CovarianceResult<T> {
    /// Returns a shared reference to one zero-based matrix entry.
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.parameter_count || column >= self.parameter_count {
            return None;
        }
        self.covariance
            .get(row + column.checked_mul(self.parameter_count)?)
    }

    /// Returns the complete full column-major covariance storage.
    ///
    /// The slice contains no hidden Fortran workspace or triangular padding.
    pub fn as_column_major_slice(&self) -> &[T] {
        &self.covariance
    }
}

impl CovarianceResult<f64> {
    /// Returns one standard error per parameter, `sqrt(covariance[i,i])`.
    ///
    /// # Errors
    ///
    /// Returns [`CovarianceError::NegativeVarianceDiagonal`] if a native
    /// diagonal is negative, or [`CovarianceError::ZeroVarianceDiagonal`] if
    /// a non-finite value prevents a meaningful standard error.
    pub fn standard_errors(&self) -> Result<Vec<f64>, CovarianceError> {
        standard_errors_f64(self)
    }

    /// Returns a full column-major correlation matrix.
    ///
    /// `correlation[i,j] = covariance[i,j] / (se[i] * se[j])`. It is not
    /// available when any standard error is zero, negative, or non-finite.
    pub fn correlation_matrix(&self) -> Result<Vec<f64>, CovarianceError> {
        correlation_f64(self)
    }
}

impl CovarianceResult<f32> {
    /// Returns one single-precision standard error per parameter.
    ///
    /// This is the `SCOV` counterpart of [`CovarianceResult::<f64>::standard_errors`].
    pub fn standard_errors(&self) -> Result<Vec<f32>, CovarianceError> {
        standard_errors_f32(self)
    }

    /// Returns a full single-precision column-major correlation matrix.
    ///
    /// This is the `SCOV` counterpart of [`CovarianceResult::<f64>::correlation_matrix`].
    pub fn correlation_matrix(&self) -> Result<Vec<f32>, CovarianceError> {
        correlation_f32(self)
    }
}

/// Error returned by safe covariance estimation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CovarianceError {
    /// The parameter vector has no entries.
    EmptyParameters,
    /// The requested residual count is zero.
    EmptyResiduals,
    /// The reviewed covariance contract requires `M >= N`.
    Underdetermined {
        /// Number of residuals (`M`).
        residuals: usize,
        /// Number of parameters (`N`).
        parameters: usize,
    },
    /// A supplied parameter is NaN or infinite.
    NonFiniteParameter {
        /// Zero-based parameter index.
        index: usize,
    },
    /// Residual-variance scaling needs positive `M - N` degrees of freedom.
    NonPositiveDegreesOfFreedom {
        /// Number of residuals (`M`).
        residuals: usize,
        /// Full native rank (`N`) required for a successful covariance.
        rank: usize,
    },
    /// The residual callback panicked before returning to Fortran.
    CallbackPanicked,
    /// A residual callback output was NaN or infinite.
    CallbackReturnedNonFinite {
        /// Zero-based residual index.
        index: usize,
    },
    /// The dense Jacobian callback panicked before returning to Fortran.
    JacobianPanicked,
    /// A logical dense Jacobian entry was NaN, infinite, or left unwritten.
    JacobianReturnedNonFinite {
        /// Zero-based residual row.
        row: usize,
        /// Zero-based parameter column.
        column: usize,
    },
    /// A callback attempted another callback-bearing SLATEC facade.
    NestedNativeCallback,
    /// Checked conversion to the GNU Fortran `INTEGER` profile failed.
    IntegerOverflow {
        /// Argument or internal length that did not fit.
        argument: &'static str,
    },
    /// Checked `M * N` or result-matrix allocation arithmetic overflowed.
    WorkspaceOverflow,
    /// `SCOV` or `DCOV` detected an exact zero in its unpivoted triangular
    /// factor. It returns no covariance and no reliable effective rank.
    RankDeficient {
        /// Parameter count for the singular native factorization.
        parameter_count: usize,
    },
    /// An expert-fit adapter rejected the supplied prior completion status.
    IneligibleFitStatus {
        /// Completion status carried by the expert fit.
        status: LeastSquaresStatus,
    },
    /// A covariance diagonal was negative or non-finite.
    NegativeVarianceDiagonal {
        /// Zero-based parameter index.
        index: usize,
    },
    /// A correlation requested division by a zero standard error.
    ZeroVarianceDiagonal {
        /// Zero-based parameter index.
        index: usize,
    },
    /// The native routine returned an undocumented `INFO` value.
    NativeStatus {
        /// Raw Fortran `INFO` value.
        status: i32,
    },
    /// Native code violated a reviewed pointer, shape, or output invariant.
    NativeContractViolation {
        /// Stable explanation of the failed invariant.
        detail: &'static str,
    },
}

impl fmt::Display for CovarianceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyParameters => write!(formatter, "covariance estimation needs parameters"),
            Self::EmptyResiduals => write!(formatter, "covariance estimation needs residuals"),
            Self::Underdetermined {
                residuals,
                parameters,
            } => write!(
                formatter,
                "covariance estimation requires residual count {residuals} to be at least parameter count {parameters}"
            ),
            Self::NonFiniteParameter { index } => write!(
                formatter,
                "covariance parameter at index {index} must be finite"
            ),
            Self::NonPositiveDegreesOfFreedom { residuals, rank } => write!(
                formatter,
                "residual-variance covariance scaling needs positive degrees of freedom; residuals {residuals}, rank {rank}"
            ),
            Self::CallbackPanicked => write!(formatter, "covariance residual callback panicked"),
            Self::CallbackReturnedNonFinite { index } => write!(
                formatter,
                "covariance residual callback returned a non-finite value at index {index}"
            ),
            Self::JacobianPanicked => write!(formatter, "covariance Jacobian callback panicked"),
            Self::JacobianReturnedNonFinite { row, column } => write!(
                formatter,
                "covariance Jacobian callback left entry ({row}, {column}) non-finite or unwritten"
            ),
            Self::NestedNativeCallback => write!(
                formatter,
                "nested callback-based SLATEC calls are unsupported"
            ),
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "covariance {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => {
                write!(formatter, "covariance workspace-size arithmetic overflowed")
            }
            Self::RankDeficient { parameter_count } => write!(
                formatter,
                "native covariance factorization was singular for {parameter_count} parameters"
            ),
            Self::IneligibleFitStatus { status } => write!(
                formatter,
                "expert least-squares status {status:?} is not eligible for covariance"
            ),
            Self::NegativeVarianceDiagonal { index } => write!(
                formatter,
                "covariance diagonal at index {index} is negative or non-finite"
            ),
            Self::ZeroVarianceDiagonal { index } => write!(
                formatter,
                "covariance diagonal at index {index} has zero variance"
            ),
            Self::NativeStatus { status } => {
                write!(formatter, "unknown covariance native status {status}")
            }
            Self::NativeContractViolation { detail } => {
                write!(
                    formatter,
                    "native covariance contract was violated: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for CovarianceError {}

fn native_integer(value: usize, argument: &'static str) -> Result<FortranInteger, CovarianceError> {
    to_fortran_integer(value).map_err(|_| CovarianceError::IntegerOverflow { argument })
}

fn validate<T: Copy>(parameters: &[T], residual_count: usize) -> Result<(), CovarianceError> {
    if parameters.is_empty() {
        return Err(CovarianceError::EmptyParameters);
    }
    if residual_count == 0 {
        return Err(CovarianceError::EmptyResiduals);
    }
    if residual_count < parameters.len() {
        return Err(CovarianceError::Underdetermined {
            residuals: residual_count,
            parameters: parameters.len(),
        });
    }
    Ok(())
}

fn validate_f64(
    parameters: &[f64],
    residual_count: usize,
    options: CovarianceOptions,
) -> Result<(), CovarianceError> {
    validate(parameters, residual_count)?;
    if let Some((index, _)) = parameters
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(CovarianceError::NonFiniteParameter { index });
    }
    validate_scaling(parameters.len(), residual_count, options)
}

fn validate_f32(
    parameters: &[f32],
    residual_count: usize,
    options: CovarianceOptions,
) -> Result<(), CovarianceError> {
    validate(parameters, residual_count)?;
    if let Some((index, _)) = parameters
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(CovarianceError::NonFiniteParameter { index });
    }
    validate_scaling(parameters.len(), residual_count, options)
}

fn validate_scaling(
    parameter_count: usize,
    residual_count: usize,
    options: CovarianceOptions,
) -> Result<(), CovarianceError> {
    if options.scaling == CovarianceScaling::ResidualVariance && residual_count <= parameter_count {
        return Err(CovarianceError::NonPositiveDegreesOfFreedom {
            residuals: residual_count,
            rank: parameter_count,
        });
    }
    Ok(())
}

fn callback_error(error: CallbackRuntimeError) -> CovarianceError {
    match error {
        CallbackRuntimeError::NestedCallback => CovarianceError::NestedNativeCallback,
    }
}

fn callback_failure(failure: ExpertLeastSquaresCallbackFailure) -> CovarianceError {
    match failure {
        ExpertLeastSquaresCallbackFailure::ResidualPanicked => CovarianceError::CallbackPanicked,
        ExpertLeastSquaresCallbackFailure::JacobianPanicked => CovarianceError::JacobianPanicked,
        ExpertLeastSquaresCallbackFailure::ResidualNonFinite { index } => {
            CovarianceError::CallbackReturnedNonFinite { index }
        }
        ExpertLeastSquaresCallbackFailure::JacobianNonFinite { row, column } => {
            CovarianceError::JacobianReturnedNonFinite { row, column }
        }
        ExpertLeastSquaresCallbackFailure::InvalidPointer => {
            CovarianceError::NativeContractViolation {
                detail: "covariance callback pointer was null or its input/output regions overlapped",
            }
        }
        ExpertLeastSquaresCallbackFailure::DimensionMismatch => {
            CovarianceError::NativeContractViolation {
                detail: "covariance callback M or N differed from its registered context",
            }
        }
        ExpertLeastSquaresCallbackFailure::InvalidLeadingDimension => {
            CovarianceError::NativeContractViolation {
                detail: "covariance callback LDR was invalid",
            }
        }
        ExpertLeastSquaresCallbackFailure::UnexpectedFlag => {
            CovarianceError::NativeContractViolation {
                detail: "covariance callback received an unsupported IFLAG",
            }
        }
    }
}

fn residual_sum_of_squares_f64(values: &[f64]) -> Result<f64, CovarianceError> {
    let mut scale = 0.0_f64;
    let mut sum = 1.0_f64;
    for value in values {
        let magnitude = value.abs();
        if magnitude != 0.0 {
            if scale < magnitude {
                sum = 1.0 + sum * (scale / magnitude).powi(2);
                scale = magnitude;
            } else {
                sum += (magnitude / scale).powi(2);
            }
        }
    }
    let square = if scale == 0.0 {
        0.0
    } else {
        scale * scale * sum
    };
    if square.is_finite() {
        Ok(square)
    } else {
        Err(CovarianceError::NativeContractViolation {
            detail: "finite residuals produced a residual sum of squares outside f64 range",
        })
    }
}

fn residual_sum_of_squares_f32(values: &[f32]) -> Result<f32, CovarianceError> {
    let mut scale = 0.0_f32;
    let mut sum = 1.0_f32;
    for value in values {
        let magnitude = value.abs();
        if magnitude != 0.0 {
            if scale < magnitude {
                sum = 1.0 + sum * (scale / magnitude).powi(2);
                scale = magnitude;
            } else {
                sum += (magnitude / scale).powi(2);
            }
        }
    }
    let square = if scale == 0.0 {
        0.0
    } else {
        scale * scale * sum
    };
    if square.is_finite() {
        Ok(square)
    } else {
        Err(CovarianceError::NativeContractViolation {
            detail: "finite residuals produced a residual sum of squares outside f32 range",
        })
    }
}

fn expand_upper_f64(
    workspace: &[f64],
    leading_dimension: usize,
    parameter_count: usize,
) -> Result<Vec<f64>, CovarianceError> {
    let length = parameter_count
        .checked_mul(parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut covariance = vec![0.0; length];
    for column in 0..parameter_count {
        for row in 0..=column {
            let value = workspace[row + column * leading_dimension];
            if !value.is_finite() {
                return Err(CovarianceError::NativeContractViolation {
                    detail: "native DCOV returned a non-finite covariance entry",
                });
            }
            covariance[row + column * parameter_count] = value;
            covariance[column + row * parameter_count] = value;
        }
    }
    Ok(covariance)
}

fn expand_upper_f32(
    workspace: &[f32],
    leading_dimension: usize,
    parameter_count: usize,
) -> Result<Vec<f32>, CovarianceError> {
    let length = parameter_count
        .checked_mul(parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut covariance = vec![0.0; length];
    for column in 0..parameter_count {
        for row in 0..=column {
            let value = workspace[row + column * leading_dimension];
            if !value.is_finite() {
                return Err(CovarianceError::NativeContractViolation {
                    detail: "native SCOV returned a non-finite covariance entry",
                });
            }
            covariance[row + column * parameter_count] = value;
            covariance[column + row * parameter_count] = value;
        }
    }
    Ok(covariance)
}

#[allow(clippy::too_many_arguments)]
fn run_f64<F, J>(
    parameters: &[f64],
    residual_count: usize,
    residuals: F,
    mut jacobian: J,
    options: CovarianceOptions,
    analytic: bool,
) -> Result<CovarianceResult<f64>, CovarianceError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], JacobianMut<'_, f64>),
{
    validate_f64(parameters, residual_count, options)?;
    let parameter_count = parameters.len();
    // `DCOV` forwards `X` to `DFDJC3` in IOPT=1 mode. The subsidiary
    // temporarily perturbs X while forming forward differences, then restores
    // it. Keep an owned mutable copy even though DCOV documents X unchanged
    // on return; callers may pass literals or other read-only slice storage.
    let mut native_parameters = parameters.to_vec();
    let workspace_len = residual_count
        .checked_mul(parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut residual_vector = vec![0.0; residual_count];
    let mut covariance_workspace = vec![0.0; workspace_len];
    let mut wa1 = vec![0.0; parameter_count];
    let mut wa2 = vec![0.0; parameter_count];
    let mut wa3 = vec![0.0; parameter_count];
    let mut wa4 = vec![0.0; residual_count];
    let mut iopt = if analytic { 2 } else { 1 };
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut ldr = m;
    let mut info = 0;
    let invocation = callback_runtime::with_expert_least_squares_f64(
        parameter_count,
        residual_count,
        analytic,
        residuals,
        move |x, fvec, matrix, leading_dimension| {
            if let Some(view) =
                JacobianMut::new(matrix, residual_count, parameter_count, leading_dimension)
            {
                jacobian(x, fvec, view);
            }
        },
        |callback: ExpertLeastSquaresF64Callback| {
            let _error_scope = crate::runtime::permit_recoverable_native_statuses();
            // SAFETY: M, N, LDR=M, and the exact R[M*N], FVEC[M], WA1..3[N],
            // and WA4[M] layouts are checked; the scoped callback validates
            // all native callback pointers; and this uses the reviewed GNU
            // MinGW `DCOV` ABI while the process-global runtime lock is held.
            unsafe {
                slatec_sys::least_squares::dcov(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    native_parameters.as_mut_ptr(),
                    residual_vector.as_mut_ptr(),
                    covariance_workspace.as_mut_ptr(),
                    &mut ldr,
                    &mut info,
                    wa1.as_mut_ptr(),
                    wa2.as_mut_ptr(),
                    wa3.as_mut_ptr(),
                    wa4.as_mut_ptr(),
                );
            }
        },
    )
    .map_err(callback_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    match info {
        1 => {}
        2 => return Err(CovarianceError::RankDeficient { parameter_count }),
        0 => {
            return Err(CovarianceError::NativeContractViolation {
                detail: "DCOV rejected Rust-validated dimensions",
            });
        }
        value => return Err(CovarianceError::NativeStatus { status: value }),
    }
    let rss = residual_sum_of_squares_f64(&residual_vector)?;
    let variance_scale = if residual_count == parameter_count {
        1.0
    } else {
        rss / (residual_count - parameter_count) as f64
    };
    if !variance_scale.is_finite() {
        return Err(CovarianceError::NativeContractViolation {
            detail: "native DCOV variance scale was non-finite",
        });
    }
    Ok(CovarianceResult {
        covariance: expand_upper_f64(&covariance_workspace, residual_count, parameter_count)?,
        parameter_count,
        rank: parameter_count,
        permutation: (0..parameter_count).collect(),
        residual_sum_of_squares: rss,
        variance_scale,
        status: CovarianceStatus::FullRank,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_f32<F, J>(
    parameters: &[f32],
    residual_count: usize,
    residuals: F,
    mut jacobian: J,
    options: CovarianceOptions,
    analytic: bool,
) -> Result<CovarianceResult<f32>, CovarianceError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], JacobianMut<'_, f32>),
{
    validate_f32(parameters, residual_count, options)?;
    let parameter_count = parameters.len();
    // `SCOV` has the same IOPT=1 temporary-X mutation as `DCOV`.
    let mut native_parameters = parameters.to_vec();
    let workspace_len = residual_count
        .checked_mul(parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut residual_vector = vec![0.0; residual_count];
    let mut covariance_workspace = vec![0.0; workspace_len];
    let mut wa1 = vec![0.0; parameter_count];
    let mut wa2 = vec![0.0; parameter_count];
    let mut wa3 = vec![0.0; parameter_count];
    let mut wa4 = vec![0.0; residual_count];
    let mut iopt = if analytic { 2 } else { 1 };
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut ldr = m;
    let mut info = 0;
    let invocation = callback_runtime::with_expert_least_squares_f32(
        parameter_count,
        residual_count,
        analytic,
        residuals,
        move |x, fvec, matrix, leading_dimension| {
            if let Some(view) =
                JacobianMut::new(matrix, residual_count, parameter_count, leading_dimension)
            {
                jacobian(x, fvec, view);
            }
        },
        |callback: ExpertLeastSquaresF32Callback| {
            let _error_scope = crate::runtime::permit_recoverable_native_statuses();
            // SAFETY: f32 equivalent of the checked `DCOV` call above, using
            // the reviewed `SCOV` ABI and exact source-documented work arrays.
            unsafe {
                slatec_sys::least_squares::scov(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    native_parameters.as_mut_ptr(),
                    residual_vector.as_mut_ptr(),
                    covariance_workspace.as_mut_ptr(),
                    &mut ldr,
                    &mut info,
                    wa1.as_mut_ptr(),
                    wa2.as_mut_ptr(),
                    wa3.as_mut_ptr(),
                    wa4.as_mut_ptr(),
                );
            }
        },
    )
    .map_err(callback_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    match info {
        1 => {}
        2 => return Err(CovarianceError::RankDeficient { parameter_count }),
        0 => {
            return Err(CovarianceError::NativeContractViolation {
                detail: "SCOV rejected Rust-validated dimensions",
            });
        }
        value => return Err(CovarianceError::NativeStatus { status: value }),
    }
    let rss = residual_sum_of_squares_f32(&residual_vector)?;
    let variance_scale = if residual_count == parameter_count {
        1.0
    } else {
        rss / (residual_count - parameter_count) as f32
    };
    if !variance_scale.is_finite() {
        return Err(CovarianceError::NativeContractViolation {
            detail: "native SCOV variance scale was non-finite",
        });
    }
    Ok(CovarianceResult {
        covariance: expand_upper_f32(&covariance_workspace, residual_count, parameter_count)?,
        parameter_count,
        rank: parameter_count,
        permutation: (0..parameter_count).collect(),
        residual_sum_of_squares: rss,
        variance_scale,
        status: CovarianceStatus::FullRank,
    })
}

/// Estimates double-precision covariance with a dense analytic Jacobian.
///
/// Wraps original SLATEC `DCOV` with `IOPT=2`. It evaluates the residual
/// closure once and the Jacobian closure once at `parameters`, then returns
/// the full symmetric expansion of the upper-triangular native covariance.
/// The mathematical result is `RSS/(M-N) · (JᵀJ)⁻¹` for `M>N`; see
/// [`CovarianceScaling::Native`] for the native square-problem convention.
/// `parameters` corresponds to Fortran `X`, `residual_count` to `M`, and the
/// closures jointly supply `FCN`/`FVEC`/`R`.
///
/// Calls allocate the exact native `R[M*N]`, `FVEC[M]`, `WA1..WA3[N]`, and
/// `WA4[M]` arrays, serialize the process-global GNU Fortran runtime, contain
/// callback panics and non-finite values, and restore the legacy XERROR flag.
/// It requires `std`, `alloc`, `least-squares-covariance`, and the validated
/// GNU Fortran x86_64 MinGW native profile.
///
/// # Errors
///
/// Returns [`CovarianceError`] for invalid dimensions, callback failures,
/// singular native Jacobians, checked allocation/integer overflow, or native
/// contract violations. A singular matrix is never converted to a
/// pseudoinverse.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::CovarianceError> {
/// use slatec::least_squares::{CovarianceOptions, estimate_covariance};
/// let covariance = estimate_covariance(
///     &[1.0, 2.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.1, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.2]),
///     |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); }
///                     j.set(0, 1, 0.0).unwrap(); j.set(1, 1, 1.0).unwrap(); j.set(2, 1, 2.0).unwrap(); },
///     CovarianceOptions::default(),
/// )?;
/// assert_eq!(covariance.rank, 2);
/// # Ok(()) }
/// ```
pub fn estimate_covariance<F, J>(
    parameters: &[f64],
    residual_count: usize,
    residuals: F,
    jacobian: J,
    options: CovarianceOptions,
) -> Result<CovarianceResult<f64>, CovarianceError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], JacobianMut<'_, f64>),
{
    run_f64(
        parameters,
        residual_count,
        residuals,
        jacobian,
        options,
        true,
    )
}

/// Estimates single-precision covariance with `SCOV` and a dense analytic
/// Jacobian.
///
/// This is the f32 counterpart of [`estimate_covariance`]. Its column-major
/// Jacobian, full symmetric result, scaling, callback containment, allocation,
/// serialization, and singularity behavior are identical modulo f32 rounding.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::CovarianceError> {
/// use slatec::least_squares::{CovarianceOptions, estimate_covariance_f32};
/// let covariance = estimate_covariance_f32(
///     &[1.0_f32, 2.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.1, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.2]),
///     |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); }
///                     j.set(0, 1, 0.0).unwrap(); j.set(1, 1, 1.0).unwrap(); j.set(2, 1, 2.0).unwrap(); },
///     CovarianceOptions::default(),
/// )?;
/// assert_eq!(covariance.rank, 2);
/// # Ok(()) }
/// ```
pub fn estimate_covariance_f32<F, J>(
    parameters: &[f32],
    residual_count: usize,
    residuals: F,
    jacobian: J,
    options: CovarianceOptions,
) -> Result<CovarianceResult<f32>, CovarianceError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], JacobianMut<'_, f32>),
{
    run_f32(
        parameters,
        residual_count,
        residuals,
        jacobian,
        options,
        true,
    )
}

/// Estimates double-precision covariance with `DCOV` forward differences.
///
/// Wraps original `DCOV` with `IOPT=1`; `residuals` supplies the `FCN` values
/// and the native routine builds a forward-difference Jacobian. The result,
/// scaling, rank policy, full column-major layout, allocation, serialization,
/// and error behavior follow [`estimate_covariance`]. Use an analytic
/// Jacobian where practical to avoid differencing error and extra residual
/// calls.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::CovarianceError> {
/// use slatec::least_squares::{CovarianceOptions, estimate_covariance_finite_difference};
/// let covariance = estimate_covariance_finite_difference(
///     &[1.0, 2.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.1, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.2]),
///     CovarianceOptions::default(),
/// )?;
/// assert!(covariance.variance_scale > 0.0);
/// # Ok(()) }
/// ```
pub fn estimate_covariance_finite_difference<F>(
    parameters: &[f64],
    residual_count: usize,
    residuals: F,
    options: CovarianceOptions,
) -> Result<CovarianceResult<f64>, CovarianceError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    run_f64(
        parameters,
        residual_count,
        residuals,
        |_, _, _| {},
        options,
        false,
    )
}

/// Estimates single-precision covariance with `SCOV` forward differences.
///
/// This is the f32 counterpart of [`estimate_covariance_finite_difference`].
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::CovarianceError> {
/// use slatec::least_squares::{CovarianceOptions, estimate_covariance_finite_difference_f32};
/// let covariance = estimate_covariance_finite_difference_f32(
///     &[1.0_f32, 2.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.1, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.2]),
///     CovarianceOptions::default(),
/// )?;
/// assert!(covariance.variance_scale > 0.0);
/// # Ok(()) }
/// ```
pub fn estimate_covariance_finite_difference_f32<F>(
    parameters: &[f32],
    residual_count: usize,
    residuals: F,
    options: CovarianceOptions,
) -> Result<CovarianceResult<f32>, CovarianceError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    run_f32(
        parameters,
        residual_count,
        residuals,
        |_, _, _| {},
        options,
        false,
    )
}

/// Estimates covariance at a prior double-precision expert-fit result.
///
/// This optional adapter is available only with both
/// `least-squares-covariance` and `least-squares-nonlinear-expert`. It uses
/// `fit.parameters` as Fortran `X`, checks `fit.status` against `eligibility`,
/// then invokes `DCOV` with `IOPT=2`; it does not retain or reuse a solver QR
/// factorization because `DCOV` requires a fresh residual/Jacobian evaluation.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use slatec::least_squares::{CovarianceEligibility, CovarianceOptions, ExpertLeastSquaresOptions, covariance_from_expert_fit, least_squares_with_jacobian};
/// let fit = least_squares_with_jacobian(&[0.0, 0.0], 3, |p, r| r.copy_from_slice(&[p[0]-1.1, p[0]+p[1]-3.0, p[0]+2.0*p[1]-5.2]), |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); } j.set(0,1,0.0).unwrap(); j.set(1,1,1.0).unwrap(); j.set(2,1,2.0).unwrap(); }, ExpertLeastSquaresOptions::default())?;
/// let covariance = covariance_from_expert_fit(&fit, |p, r| r.copy_from_slice(&[p[0]-1.1, p[0]+p[1]-3.0, p[0]+2.0*p[1]-5.2]), |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); } j.set(0,1,0.0).unwrap(); j.set(1,1,1.0).unwrap(); j.set(2,1,2.0).unwrap(); }, CovarianceOptions::default(), CovarianceEligibility::ConvergedOnly)?;
/// assert_eq!(covariance.rank, 2);
/// # Ok(()) }
/// ```
#[cfg(feature = "least-squares-nonlinear-expert")]
pub fn covariance_from_expert_fit<F, J>(
    fit: &ExpertLeastSquaresResult<f64>,
    residuals: F,
    jacobian: J,
    options: CovarianceOptions,
    eligibility: CovarianceEligibility,
) -> Result<CovarianceResult<f64>, CovarianceError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], JacobianMut<'_, f64>),
{
    validate_eligibility(fit.status, eligibility)?;
    estimate_covariance(
        &fit.parameters,
        fit.residuals.len(),
        residuals,
        jacobian,
        options,
    )
}

/// Single-precision `SCOV` counterpart of [`covariance_from_expert_fit`].
///
/// It applies the same fit-status policy and recomputes a fresh dense analytic
/// Jacobian at the returned f32 parameters.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use slatec::least_squares::{CovarianceEligibility, CovarianceOptions, ExpertLeastSquaresOptions, covariance_from_expert_fit_f32, least_squares_with_jacobian_f32};
/// let fit = least_squares_with_jacobian_f32(&[0.0_f32, 0.0], 3, |p, r| r.copy_from_slice(&[p[0]-1.1, p[0]+p[1]-3.0, p[0]+2.0*p[1]-5.2]), |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); } j.set(0,1,0.0).unwrap(); j.set(1,1,1.0).unwrap(); j.set(2,1,2.0).unwrap(); }, ExpertLeastSquaresOptions::single_precision())?;
/// let covariance = covariance_from_expert_fit_f32(&fit, |p, r| r.copy_from_slice(&[p[0]-1.1, p[0]+p[1]-3.0, p[0]+2.0*p[1]-5.2]), |_, _, mut j| { for row in 0..3 { j.set(row, 0, 1.0).unwrap(); } j.set(0,1,0.0).unwrap(); j.set(1,1,1.0).unwrap(); j.set(2,1,2.0).unwrap(); }, CovarianceOptions::default(), CovarianceEligibility::ConvergedOnly)?;
/// assert_eq!(covariance.rank, 2);
/// # Ok(()) }
/// ```
#[cfg(feature = "least-squares-nonlinear-expert")]
pub fn covariance_from_expert_fit_f32<F, J>(
    fit: &ExpertLeastSquaresResult<f32>,
    residuals: F,
    jacobian: J,
    options: CovarianceOptions,
    eligibility: CovarianceEligibility,
) -> Result<CovarianceResult<f32>, CovarianceError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], JacobianMut<'_, f32>),
{
    validate_eligibility(fit.status, eligibility)?;
    estimate_covariance_f32(
        &fit.parameters,
        fit.residuals.len(),
        residuals,
        jacobian,
        options,
    )
}

#[cfg(feature = "least-squares-nonlinear-expert")]
fn validate_eligibility(
    status: LeastSquaresStatus,
    eligibility: CovarianceEligibility,
) -> Result<(), CovarianceError> {
    let accepted = matches!(
        status,
        LeastSquaresStatus::ConvergedResidual
            | LeastSquaresStatus::ConvergedParameters
            | LeastSquaresStatus::ConvergedResidualAndParameters
            | LeastSquaresStatus::ConvergedOrthogonality
    ) || eligibility == CovarianceEligibility::AllowNumericalTermination;
    if accepted {
        Ok(())
    } else {
        Err(CovarianceError::IneligibleFitStatus { status })
    }
}

fn standard_errors_f64(result: &CovarianceResult<f64>) -> Result<Vec<f64>, CovarianceError> {
    let mut output = Vec::with_capacity(result.parameter_count);
    for index in 0..result.parameter_count {
        let value = *result
            .get(index, index)
            .expect("checked covariance diagonal");
        if !value.is_finite() || value < 0.0 {
            return Err(CovarianceError::NegativeVarianceDiagonal { index });
        }
        output.push(value.sqrt());
    }
    Ok(output)
}

fn standard_errors_f32(result: &CovarianceResult<f32>) -> Result<Vec<f32>, CovarianceError> {
    let mut output = Vec::with_capacity(result.parameter_count);
    for index in 0..result.parameter_count {
        let value = *result
            .get(index, index)
            .expect("checked covariance diagonal");
        if !value.is_finite() || value < 0.0 {
            return Err(CovarianceError::NegativeVarianceDiagonal { index });
        }
        output.push(value.sqrt());
    }
    Ok(output)
}

fn correlation_f64(result: &CovarianceResult<f64>) -> Result<Vec<f64>, CovarianceError> {
    let errors = standard_errors_f64(result)?;
    let length = result
        .parameter_count
        .checked_mul(result.parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut output = vec![0.0; length];
    for column in 0..result.parameter_count {
        if errors[column] == 0.0 {
            return Err(CovarianceError::ZeroVarianceDiagonal { index: column });
        }
        for row in 0..result.parameter_count {
            if errors[row] == 0.0 {
                return Err(CovarianceError::ZeroVarianceDiagonal { index: row });
            }
            output[row + column * result.parameter_count] = result.covariance
                [row + column * result.parameter_count]
                / (errors[row] * errors[column]);
        }
    }
    Ok(output)
}

fn correlation_f32(result: &CovarianceResult<f32>) -> Result<Vec<f32>, CovarianceError> {
    let errors = standard_errors_f32(result)?;
    let length = result
        .parameter_count
        .checked_mul(result.parameter_count)
        .ok_or(CovarianceError::WorkspaceOverflow)?;
    let mut output = vec![0.0; length];
    for column in 0..result.parameter_count {
        if errors[column] == 0.0 {
            return Err(CovarianceError::ZeroVarianceDiagonal { index: column });
        }
        for row in 0..result.parameter_count {
            if errors[row] == 0.0 {
                return Err(CovarianceError::ZeroVarianceDiagonal { index: row });
            }
            output[row + column * result.parameter_count] = result.covariance
                [row + column * result.parameter_count]
                / (errors[row] * errors[column]);
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::{CovarianceError, CovarianceOptions, CovarianceScaling, validate_scaling};

    #[test]
    fn residual_variance_requires_positive_degrees_of_freedom() {
        assert!(matches!(
            validate_scaling(2, 2, CovarianceOptions::default()),
            Err(CovarianceError::NonPositiveDegreesOfFreedom { .. })
        ));
        assert!(
            validate_scaling(
                2,
                2,
                CovarianceOptions {
                    scaling: CovarianceScaling::Native
                }
            )
            .is_ok()
        );
    }
}
