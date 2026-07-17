use alloc::vec;

use crate::callback_runtime::{
    self, CallbackRuntimeError, VectorCallbackFailure, VectorF32Callback, VectorF64Callback,
};
use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use super::{NonlinearError, NonlinearOptions, NonlinearResult, NonlinearStatus};

fn workspace_len(dimension: usize) -> Result<usize, NonlinearError> {
    let square = dimension
        .checked_mul(dimension)
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    let quadratic = square
        .checked_mul(3)
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    let linear = dimension
        .checked_mul(13)
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    quadratic
        .checked_add(linear)
        .ok_or(NonlinearError::WorkspaceOverflow)
        .map(|numerator| numerator / 2)
}

fn native_integer(value: usize, argument: &'static str) -> Result<FortranInteger, NonlinearError> {
    to_fortran_integer(value).map_err(|_| NonlinearError::IntegerOverflow { argument })
}

fn validate_f64(initial: &[f64], options: NonlinearOptions<f64>) -> Result<(), NonlinearError> {
    if initial.is_empty() {
        return Err(NonlinearError::EmptySystem);
    }
    if let Some((index, _)) = initial
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(NonlinearError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(NonlinearError::InvalidTolerance);
    }
    Ok(())
}

fn validate_f32(initial: &[f32], options: NonlinearOptions<f32>) -> Result<(), NonlinearError> {
    if initial.is_empty() {
        return Err(NonlinearError::EmptySystem);
    }
    if let Some((index, _)) = initial
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(NonlinearError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(NonlinearError::InvalidTolerance);
    }
    Ok(())
}

fn callback_failure(failure: VectorCallbackFailure) -> NonlinearError {
    match failure {
        VectorCallbackFailure::Panicked => NonlinearError::CallbackPanicked,
        VectorCallbackFailure::NonFinite { index } => {
            NonlinearError::CallbackReturnedNonFinite { index }
        }
        VectorCallbackFailure::Cancelled => NonlinearError::NativeContractViolation {
            detail: "unexpected internal nonlinear callback cancellation",
        },
        VectorCallbackFailure::InvalidPointer => NonlinearError::NativeContractViolation {
            detail: "native callback pointer was null or overlapped",
        },
        VectorCallbackFailure::DimensionMismatch => NonlinearError::NativeContractViolation {
            detail: "native callback dimension did not match the registered system",
        },
    }
}

fn callback_runtime_error(error: CallbackRuntimeError) -> NonlinearError {
    match error {
        CallbackRuntimeError::NestedCallback => NonlinearError::NestedNativeCallback,
    }
}

fn native_status(status: FortranInteger) -> Result<NonlinearStatus, NonlinearError> {
    match status {
        1 => Ok(NonlinearStatus::Converged),
        2 => Ok(NonlinearStatus::MaximumFunctionEvaluations),
        3 => Ok(NonlinearStatus::ToleranceTooSmall),
        4 => Ok(NonlinearStatus::SlowProgress),
        value => Err(NonlinearError::NativeStatus { status: value }),
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

unsafe extern "C" fn unused_jacobian_f64(
    _n: *const FortranInteger,
    _x: *const f64,
    _fvec: *const f64,
    _fjac: *mut f64,
    _ldfjac: *const FortranInteger,
    iflag: *mut FortranInteger,
) {
    if let Some(iflag) = unsafe { iflag.as_mut() } {
        *iflag = -1;
    }
}

unsafe extern "C" fn unused_jacobian_f32(
    _n: *const FortranInteger,
    _x: *const f32,
    _fvec: *const f32,
    _fjac: *mut f32,
    _ldfjac: *const FortranInteger,
    iflag: *mut FortranInteger,
) {
    if let Some(iflag) = unsafe { iflag.as_mut() } {
        *iflag = -1;
    }
}

fn run_f64<F>(
    initial: &[f64],
    mut function: F,
    options: NonlinearOptions<f64>,
) -> Result<NonlinearResult<f64>, NonlinearError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    validate_f64(initial, options)?;
    let dimension = initial.len();
    let mut x = initial.to_vec();
    let mut residual = vec![0.0; dimension];
    let workspace_length = workspace_len(dimension)?;
    let mut workspace = vec![0.0; workspace_length];
    let mut n = native_integer(dimension, "dimension")?;
    let mut lwa = native_integer(workspace_length, "workspace length")?;
    let mut iopt = 2;
    let mut tolerance = options.tolerance;
    let mut nprint = 0;
    let mut info = 0;

    let invocation = callback_runtime::with_vector_f64(
        dimension,
        move |x, residual| {
            function(x, residual);
            true
        },
        |callback: VectorF64Callback| {
            // SAFETY: IOPT is fixed to the reviewed finite-difference mode,
            // all scalar and workspace pointers are valid for the documented
            // `(3*n*n + 13*n)/2` elements, the callback is lexically scoped,
            // and the validated GNU MinGW native profile is selected.
            unsafe {
                slatec_sys::nonlinear::dnsqe(
                    callback.ffi(),
                    unused_jacobian_f64,
                    &mut iopt,
                    &mut n,
                    x.as_mut_ptr(),
                    residual.as_mut_ptr(),
                    &mut tolerance,
                    &mut nprint,
                    &mut info,
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
    Ok(NonlinearResult {
        solution: x,
        residual: residual.clone(),
        residual_norm: norm_f64(&residual),
        function_evaluations: invocation.evaluations,
        status: native_status(info)?,
    })
}

fn run_f32<F>(
    initial: &[f32],
    mut function: F,
    options: NonlinearOptions<f32>,
) -> Result<NonlinearResult<f32>, NonlinearError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    validate_f32(initial, options)?;
    let dimension = initial.len();
    let mut x = initial.to_vec();
    let mut residual = vec![0.0; dimension];
    let workspace_length = workspace_len(dimension)?;
    let mut workspace = vec![0.0; workspace_length];
    let mut n = native_integer(dimension, "dimension")?;
    let mut lwa = native_integer(workspace_length, "workspace length")?;
    let mut iopt = 2;
    let mut tolerance = options.tolerance;
    let mut nprint = 0;
    let mut info = 0;

    let invocation = callback_runtime::with_vector_f32(
        dimension,
        move |x, residual| {
            function(x, residual);
            true
        },
        |callback: VectorF32Callback| {
            // SAFETY: see run_f64; this uses the reviewed SNSQE single-
            // precision ABI with the same finite-difference workspace rule.
            unsafe {
                slatec_sys::nonlinear::snsqe(
                    callback.ffi(),
                    unused_jacobian_f32,
                    &mut iopt,
                    &mut n,
                    x.as_mut_ptr(),
                    residual.as_mut_ptr(),
                    &mut tolerance,
                    &mut nprint,
                    &mut info,
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
    Ok(NonlinearResult {
        solution: x,
        residual: residual.clone(),
        residual_norm: norm_f32(&residual),
        function_evaluations: invocation.evaluations,
        status: native_status(info)?,
    })
}

/// Solves a square nonlinear system with the finite-difference `DNSQE` driver.
///
/// Wraps the double-precision SLATEC routine `DNSQE` from the nonlinear-system
/// family. It seeks an `x` for which the callback writes `f(x) = 0`; `initial`
/// corresponds to Fortran `X`, the callback corresponds to `FCN`, and
/// `options.tolerance` corresponds to `TOL`. The wrapper fixes `IOPT = 2`, so
/// SLATEC estimates the Jacobian by finite differences and never calls a user
/// Jacobian. It allocates hidden `WA` and `LWA` according to
/// `(3*n*n + 13*n) / 2` and fixes `NPRINT = 0`.
///
/// `initial` must be a nonempty finite vector. The callback receives the
/// current iterate and a residual slice with exactly the same length; it must
/// overwrite every residual component with finite values. The historical
/// negative-`IFLAG` cancellation path reaches a fatal SLATEC error exit on the
/// validated profile, so cancellation is deliberately not exposed.
/// Calls allocate, serialize access to the selected process-global Fortran
/// runtime, and reject nested callback-based SLATEC calls. This wrapper is
/// available only with `std`, `alloc`, `nonlinear-easy`, and the validated GNU
/// MinGW x86_64 native profile.
///
/// The result retains the native final `X`, native final `FVEC`, a
/// Rust-recomputed Euclidean residual norm, counted callback calls, and the
/// precise reviewed native completion status. `MaximumFunctionEvaluations`,
/// `ToleranceTooSmall`, and `SlowProgress` are returned as statuses so callers
/// can inspect the final iterate; callback panic, cancellation, non-finite
/// residuals, and invalid Rust inputs return [`NonlinearError`].
///
/// # Example
///
/// ```no_run
/// # fn example() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{NonlinearOptions, solve_system};
///
/// let result = solve_system(&[1.0], |x, residual| {
///     residual[0] = x[0] * x[0] - 2.0;
/// }, NonlinearOptions::default())?;
/// assert!((result.solution[0] - 2.0_f64.sqrt()).abs() < 1.0e-8);
/// # Ok::<(), slatec::nonlinear::NonlinearError>(())
/// # }
/// ```
pub fn solve_system<F>(
    initial: &[f64],
    function: F,
    options: NonlinearOptions<f64>,
) -> Result<NonlinearResult<f64>, NonlinearError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    run_f64(initial, function, options)
}

/// Solves a square nonlinear system with the finite-difference `SNSQE` driver.
///
/// Wraps the single-precision SLATEC routine `SNSQE`. Its argument mapping,
/// callback containment, workspace allocation, runtime serialization, and
/// error behavior are the same as [`solve_system`], but all numerical values
/// use `f32`. Use [`NonlinearOptions::single_precision`] for a practical
/// initial tolerance.
///
/// # Example
///
/// ```no_run
/// # fn example() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{NonlinearOptions, solve_system_f32};
///
/// let result = solve_system_f32(&[1.0_f32], |x, residual| {
///     residual[0] = x[0] * x[0] - 2.0;
/// }, NonlinearOptions::single_precision())?;
/// assert!((result.solution[0] - 2.0_f32.sqrt()).abs() < 2.0e-4);
/// # Ok::<(), slatec::nonlinear::NonlinearError>(())
/// # }
/// ```
pub fn solve_system_f32<F>(
    initial: &[f32],
    function: F,
    options: NonlinearOptions<f32>,
) -> Result<NonlinearResult<f32>, NonlinearError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    run_f32(initial, function, options)
}

#[cfg(test)]
mod tests {
    use super::{NonlinearError, NonlinearOptions, native_status, validate_f64, workspace_len};
    use crate::nonlinear::NonlinearStatus;

    #[test]
    fn workspace_formula_is_checked_and_exact() {
        assert_eq!(workspace_len(1), Ok(8));
        assert_eq!(workspace_len(2), Ok(19));
        assert!(matches!(
            workspace_len(usize::MAX),
            Err(NonlinearError::WorkspaceOverflow)
        ));
    }

    #[test]
    fn validation_and_status_mapping_are_deterministic() {
        assert_eq!(
            validate_f64(&[], NonlinearOptions::default()),
            Err(NonlinearError::EmptySystem)
        );
        assert_eq!(
            validate_f64(&[f64::NAN], NonlinearOptions::default()),
            Err(NonlinearError::NonFiniteInitialValue { index: 0 })
        );
        assert_eq!(
            validate_f64(&[1.0], NonlinearOptions { tolerance: -1.0 }),
            Err(NonlinearError::InvalidTolerance)
        );
        assert_eq!(native_status(1), Ok(NonlinearStatus::Converged));
        assert_eq!(native_status(4), Ok(NonlinearStatus::SlowProgress));
        assert!(matches!(
            native_status(0),
            Err(NonlinearError::NativeStatus { .. })
        ));
    }
}
