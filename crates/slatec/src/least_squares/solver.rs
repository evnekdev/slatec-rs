use alloc::vec;

use crate::callback_runtime::{
    self, CallbackRuntimeError, LeastSquaresCallbackFailure, LeastSquaresF32Callback,
    LeastSquaresF64Callback,
};
use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use super::LeastSquaresError;

/// Controls accepted by the residual-only nonlinear least-squares easy drivers.
///
/// `DNLS1E` and `SNLS1E` use this one value as both their internal residual
/// and parameter convergence tolerance. The finite-difference driver fixes
/// its other controls: `GTOL = 0`, `EPSFCN = 0`, automatic scaling, a step
/// bound factor of 100, and a maximum of `200 * (N + 1)` residual calls.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeastSquaresOptions<T = f64> {
    /// Non-negative convergence tolerance passed to the Fortran `TOL`
    /// argument. Zero requests the driver's working-precision limit.
    pub tolerance: T,
}

impl Default for LeastSquaresOptions<f64> {
    fn default() -> Self {
        Self { tolerance: 1.0e-10 }
    }
}

impl LeastSquaresOptions<f32> {
    /// Returns a practical starting tolerance for single-precision `SNLS1E`.
    pub const fn single_precision() -> Self {
        Self { tolerance: 1.0e-5 }
    }
}

/// Meaningful `INFO` completion states from `SNLS1E` and `DNLS1E`.
///
/// All variants retain the native final parameters and residuals in
/// [`LeastSquaresResult`]. The final three states describe a driver that could
/// not improve one convergence criterion at the requested tolerance; they are
/// not Rust ABI or callback failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeastSquaresStatus {
    /// Both actual and predicted reductions in the residual sum of squares are
    /// small relative to `tolerance`.
    ConvergedResidual,
    /// The relative change in the parameter vector is small.
    ConvergedParameters,
    /// Both residual-reduction and parameter-change convergence tests passed.
    ConvergedResidualAndParameters,
    /// The residual is orthogonal to the Jacobian columns to working precision.
    ConvergedOrthogonality,
    /// The easy driver's fixed residual-callback budget was exhausted.
    MaximumEvaluations,
    /// Further reduction in the residual sum of squares is limited by working
    /// precision at the requested tolerance.
    ResidualToleranceTooSmall,
    /// Further improvement in the parameters is limited by working precision.
    ParameterToleranceTooSmall,
}

/// Result returned by a nonlinear least-squares easy driver.
///
/// `parameters` and `residuals` are the final Fortran `X` and `FVEC` arrays.
/// The wrapper recomputes `residual_norm = sqrt(sum_i residuals[i]^2)` using a
/// scaled Euclidean norm and `cost = residual_norm^2 / 2`. `function_evaluations`
/// counts all calls made through the contained Rust residual trampoline.
#[derive(Clone, Debug, PartialEq)]
pub struct LeastSquaresResult<T = f64> {
    /// Final parameter vector returned through Fortran argument `X`.
    pub parameters: alloc::vec::Vec<T>,
    /// Final residual vector returned through Fortran argument `FVEC`.
    pub residuals: alloc::vec::Vec<T>,
    /// One half of the recomputed residual sum of squares.
    pub cost: T,
    /// Recomputed Euclidean norm of [`Self::residuals`].
    pub residual_norm: T,
    /// Interpreted native `INFO` completion status.
    pub status: LeastSquaresStatus,
    /// Number of contained Rust residual-callback invocations.
    pub function_evaluations: usize,
}

fn workspace_len(
    parameter_count: usize,
    residual_count: usize,
) -> Result<usize, LeastSquaresError> {
    // IOPT=1 contract in SNLS1E/DNLS1E: LWA >= N * (M + 5) + M.
    let residual_plus_five = residual_count
        .checked_add(5)
        .ok_or(LeastSquaresError::WorkspaceOverflow)?;
    parameter_count
        .checked_mul(residual_plus_five)
        .and_then(|value| value.checked_add(residual_count))
        .ok_or(LeastSquaresError::WorkspaceOverflow)
}

fn native_integer(
    value: usize,
    argument: &'static str,
) -> Result<FortranInteger, LeastSquaresError> {
    to_fortran_integer(value).map_err(|_| LeastSquaresError::IntegerOverflow { argument })
}

fn validate_f64(
    initial: &[f64],
    residual_count: usize,
    options: LeastSquaresOptions<f64>,
) -> Result<(), LeastSquaresError> {
    if initial.is_empty() {
        return Err(LeastSquaresError::EmptyParameters);
    }
    if residual_count == 0 {
        return Err(LeastSquaresError::EmptyResiduals);
    }
    if residual_count < initial.len() {
        return Err(LeastSquaresError::Underdetermined {
            residuals: residual_count,
            parameters: initial.len(),
        });
    }
    if let Some((index, _)) = initial
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(LeastSquaresError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidTolerance);
    }
    Ok(())
}

fn validate_f32(
    initial: &[f32],
    residual_count: usize,
    options: LeastSquaresOptions<f32>,
) -> Result<(), LeastSquaresError> {
    if initial.is_empty() {
        return Err(LeastSquaresError::EmptyParameters);
    }
    if residual_count == 0 {
        return Err(LeastSquaresError::EmptyResiduals);
    }
    if residual_count < initial.len() {
        return Err(LeastSquaresError::Underdetermined {
            residuals: residual_count,
            parameters: initial.len(),
        });
    }
    if let Some((index, _)) = initial
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(LeastSquaresError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidTolerance);
    }
    Ok(())
}

fn callback_failure(failure: LeastSquaresCallbackFailure) -> LeastSquaresError {
    match failure {
        LeastSquaresCallbackFailure::Panicked => LeastSquaresError::CallbackPanicked,
        LeastSquaresCallbackFailure::NonFinite { index } => {
            LeastSquaresError::CallbackReturnedNonFinite { index }
        }
        LeastSquaresCallbackFailure::InvalidPointer => LeastSquaresError::NativeContractViolation {
            detail: "native least-squares callback pointer was null or overlapped",
        },
        LeastSquaresCallbackFailure::DimensionMismatch => {
            LeastSquaresError::NativeContractViolation {
                detail: "native M or N did not match the registered least-squares callback",
            }
        }
        LeastSquaresCallbackFailure::UnexpectedFlag => LeastSquaresError::NativeContractViolation {
            detail: "DNLS1E/SNLS1E IOPT=1 callback received an unexpected IFLAG",
        },
    }
}

fn callback_runtime_error(error: CallbackRuntimeError) -> LeastSquaresError {
    match error {
        CallbackRuntimeError::NestedCallback => LeastSquaresError::NestedNativeCallback,
    }
}

fn native_status(status: FortranInteger) -> Result<LeastSquaresStatus, LeastSquaresError> {
    match status {
        1 => Ok(LeastSquaresStatus::ConvergedResidual),
        2 => Ok(LeastSquaresStatus::ConvergedParameters),
        3 => Ok(LeastSquaresStatus::ConvergedResidualAndParameters),
        4 => Ok(LeastSquaresStatus::ConvergedOrthogonality),
        5 => Ok(LeastSquaresStatus::MaximumEvaluations),
        6 => Ok(LeastSquaresStatus::ResidualToleranceTooSmall),
        7 => Ok(LeastSquaresStatus::ParameterToleranceTooSmall),
        value => Err(LeastSquaresError::NativeStatus { status: value }),
    }
}

fn norm_f64(values: &[f64]) -> f64 {
    let mut scale = 0.0_f64;
    let mut sum = 1.0_f64;
    for value in values {
        let magnitude = value.abs();
        if magnitude != 0.0 {
            if scale < magnitude {
                sum = 1.0 + sum * (scale / magnitude) * (scale / magnitude);
                scale = magnitude;
            } else {
                sum += (magnitude / scale) * (magnitude / scale);
            }
        }
    }
    if scale == 0.0 {
        0.0
    } else {
        scale * sum.sqrt()
    }
}

fn norm_f32(values: &[f32]) -> f32 {
    let mut scale = 0.0_f32;
    let mut sum = 1.0_f32;
    for value in values {
        let magnitude = value.abs();
        if magnitude != 0.0 {
            if scale < magnitude {
                sum = 1.0 + sum * (scale / magnitude) * (scale / magnitude);
                scale = magnitude;
            } else {
                sum += (magnitude / scale) * (magnitude / scale);
            }
        }
    }
    if scale == 0.0 {
        0.0
    } else {
        scale * sum.sqrt()
    }
}

fn run_f64<F>(
    initial: &[f64],
    residual_count: usize,
    function: F,
    options: LeastSquaresOptions<f64>,
) -> Result<LeastSquaresResult<f64>, LeastSquaresError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    validate_f64(initial, residual_count, options)?;
    let parameter_count = initial.len();
    let workspace_length = workspace_len(parameter_count, residual_count)?;
    let mut parameters = initial.to_vec();
    let mut residuals = vec![0.0; residual_count];
    let mut integer_workspace = vec![0; parameter_count];
    let mut workspace = vec![0.0; workspace_length];
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut lwa = native_integer(workspace_length, "workspace length")?;
    let mut iopt = 1;
    let mut tolerance = options.tolerance;
    let mut nprint = 0;
    let mut info = 0;
    let invocation = callback_runtime::with_least_squares_f64(
        parameter_count,
        residual_count,
        function,
        |callback: LeastSquaresF64Callback| {
            let _error_scope = crate::runtime::permit_recoverable_least_squares_statuses();
            // SAFETY: checked M, N, and LWA match the reviewed `DNLS1E`
            // IOPT=1 formula; parameter/residual/workspace pointers remain
            // valid for the call; the callback is scoped and panic-contained;
            // and the validated GNU MinGW native profile is selected.
            unsafe {
                slatec_sys::least_squares::dnls1e(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    parameters.as_mut_ptr(),
                    residuals.as_mut_ptr(),
                    &mut tolerance,
                    &mut nprint,
                    &mut info,
                    integer_workspace.as_mut_ptr(),
                    workspace.as_mut_ptr(),
                    &mut lwa,
                );
            }
        },
    )
    .map_err(callback_runtime_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    let residual_norm = norm_f64(&residuals);
    Ok(LeastSquaresResult {
        parameters,
        residuals,
        cost: 0.5 * residual_norm * residual_norm,
        residual_norm,
        status: native_status(info)?,
        function_evaluations: invocation.evaluations,
    })
}

fn run_f32<F>(
    initial: &[f32],
    residual_count: usize,
    function: F,
    options: LeastSquaresOptions<f32>,
) -> Result<LeastSquaresResult<f32>, LeastSquaresError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    validate_f32(initial, residual_count, options)?;
    let parameter_count = initial.len();
    let workspace_length = workspace_len(parameter_count, residual_count)?;
    let mut parameters = initial.to_vec();
    let mut residuals = vec![0.0; residual_count];
    let mut integer_workspace = vec![0; parameter_count];
    let mut workspace = vec![0.0; workspace_length];
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut lwa = native_integer(workspace_length, "workspace length")?;
    let mut iopt = 1;
    let mut tolerance = options.tolerance;
    let mut nprint = 0;
    let mut info = 0;
    let invocation = callback_runtime::with_least_squares_f32(
        parameter_count,
        residual_count,
        function,
        |callback: LeastSquaresF32Callback| {
            let _error_scope = crate::runtime::permit_recoverable_least_squares_statuses();
            // SAFETY: see run_f64; this calls the reviewed single-precision
            // `SNLS1E` ABI with the same checked IOPT=1 workspace formula.
            unsafe {
                slatec_sys::least_squares::snls1e(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    parameters.as_mut_ptr(),
                    residuals.as_mut_ptr(),
                    &mut tolerance,
                    &mut nprint,
                    &mut info,
                    integer_workspace.as_mut_ptr(),
                    workspace.as_mut_ptr(),
                    &mut lwa,
                );
            }
        },
    )
    .map_err(callback_runtime_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    let residual_norm = norm_f32(&residuals);
    Ok(LeastSquaresResult {
        parameters,
        residuals,
        cost: 0.5 * residual_norm * residual_norm,
        residual_norm,
        status: native_status(info)?,
        function_evaluations: invocation.evaluations,
    })
}

/// Fits f64 parameters by minimizing one half of the residual sum of squares.
///
/// Wraps the double-precision SLATEC routine `DNLS1E` (the MINPACK-style
/// Levenberg--Marquardt easy driver). `initial` is the Fortran `X` input and
/// output of length `N`; `residual_count` is `M`; and `function` is the
/// contained replacement for `FCN`, receiving `X` and writable `FVEC`. The
/// callback must write exactly `M` finite residuals. `options.tolerance` maps
/// to `TOL`; the wrapper selects `IOPT=1` and `NPRINT=0`, hiding the native
/// finite-difference Jacobian and work arrays.
///
/// `M` must be at least `N`, all inputs and the tolerance must be finite where
/// applicable, and `TOL` must be non-negative. The function allocates an
/// `INTEGER[N]` workspace and a floating workspace of `N*(M+5)+M` elements.
/// Calls serialize the process-global GNU Fortran runtime. A panic or
/// non-finite callback result is contained and returned as
/// [`LeastSquaresError`]; a nested callback-bearing SLATEC call is rejected.
/// While holding that serialization lock, the wrapper temporarily selects the
/// validated nonfatal legacy-error control so SLATEC's level-one numerical
/// completion messages return as [`LeastSquaresResult::status`], then restores
/// the application's prior process-global setting before it returns.
///
/// The method is locally convergent and sensitive to the initial estimate. It
/// forms forward-difference Jacobians internally, so it can need substantially
/// more residual evaluations than an analytic-Jacobian solver. `cost` and
/// `residual_norm` are recomputed from the final residual vector.
///
/// # Errors
///
/// Returns [`LeastSquaresError::Underdetermined`] when `M < N`, validation
/// errors for empty/non-finite inputs or an invalid tolerance, callback errors
/// for panics or non-finite residuals, and a native-contract error if the
/// reviewed ABI invariants are broken.
///
/// # Example
///
/// ```no_run
/// # fn example() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{LeastSquaresOptions, least_squares};
/// let xs = [0.0, 1.0, 2.0, 3.0];
/// let ys = [1.0, 3.0, 5.0, 7.0];
/// let fit = least_squares(&[0.0, 0.0], xs.len(), |p, residuals| {
///     for ((&x, &y), r) in xs.iter().zip(ys.iter()).zip(residuals) {
///         *r = p[0] + p[1] * x - y;
///     }
/// }, LeastSquaresOptions::default())?;
/// assert!((fit.parameters[0] - 1.0).abs() < 1.0e-8);
/// assert!((fit.parameters[1] - 2.0).abs() < 1.0e-8);
/// # Ok::<(), slatec::least_squares::LeastSquaresError>(())
/// # }
/// ```
pub fn least_squares<F>(
    initial: &[f64],
    residual_count: usize,
    residuals: F,
    options: LeastSquaresOptions<f64>,
) -> Result<LeastSquaresResult<f64>, LeastSquaresError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    run_f64(initial, residual_count, residuals, options)
}

/// Single-precision counterpart of [`least_squares`].
///
/// Wraps the single-precision SLATEC routine `SNLS1E`. Arguments, the `M >= N`
/// restriction, finite-difference policy, allocation formula, callback
/// containment, runtime serialization, and status interpretation match the
/// f64 API. Use [`LeastSquaresOptions::single_precision`] for a practical
/// starting tolerance and account for f32 rounding when choosing assertions.
///
/// # Errors
///
/// Returns the same validation, callback-containment, nested-call, and native
/// contract errors as [`least_squares`].
///
/// # Example
///
/// ```no_run
/// # fn example() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{LeastSquaresOptions, least_squares_f32};
/// let fit = least_squares_f32(&[0.0, 0.0], 3, |p, r| {
///     r.copy_from_slice(&[p[0] - 1.0, p[0] + p[1] - 3.0, p[0] + 2.0 * p[1] - 5.0]);
/// }, LeastSquaresOptions::single_precision())?;
/// assert!((fit.parameters[1] - 2.0).abs() < 2.0e-3);
/// # Ok::<(), slatec::least_squares::LeastSquaresError>(())
/// # }
/// ```
pub fn least_squares_f32<F>(
    initial: &[f32],
    residual_count: usize,
    residuals: F,
    options: LeastSquaresOptions<f32>,
) -> Result<LeastSquaresResult<f32>, LeastSquaresError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    run_f32(initial, residual_count, residuals, options)
}

#[cfg(test)]
mod tests {
    use super::{LeastSquaresError, LeastSquaresOptions, native_status, norm_f64, workspace_len};

    #[test]
    fn workspace_formula_is_exact_and_checked() {
        assert_eq!(workspace_len(2, 4), Ok(22));
        assert_eq!(
            workspace_len(usize::MAX, 1),
            Err(LeastSquaresError::WorkspaceOverflow)
        );
    }

    #[test]
    fn validation_rejects_rectangular_contract_violations() {
        let options = LeastSquaresOptions::default();
        assert!(matches!(
            super::validate_f64(&[], 1, options),
            Err(LeastSquaresError::EmptyParameters)
        ));
        assert!(matches!(
            super::validate_f64(&[0.0, 1.0], 1, options),
            Err(LeastSquaresError::Underdetermined { .. })
        ));
    }

    #[test]
    fn statuses_and_norm_are_preserved() {
        assert!(matches!(
            native_status(5),
            Ok(super::LeastSquaresStatus::MaximumEvaluations)
        ));
        assert_eq!(norm_f64(&[3.0, 4.0]), 5.0);
    }
}
