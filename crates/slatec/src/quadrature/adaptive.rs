use super::callback;
use super::validation::{
    Workspace, finite_bounds, output_count, principal_value_bounds, status, workspace_f32,
    workspace_f64,
};
use super::{InfiniteInterval, IntegrationError, IntegrationOptions, IntegrationResult};
use slatec_sys::FortranInteger;
use slatec_sys::quadrature::{IntegrandF32, IntegrandF64};

#[derive(Default)]
struct NativeOutput<T> {
    value: T,
    error: T,
    evaluations: FortranInteger,
    status: FortranInteger,
    intervals: FortranInteger,
}

fn execute_f64<F, D>(
    f: F,
    options: IntegrationOptions,
    driver: D,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
    D: FnOnce(IntegrandF64, &mut Workspace<f64>, &mut NativeOutput<f64>, &mut f64, &mut f64),
{
    let limit = options.limit;
    let (mut workspace, mut absolute_tolerance, mut relative_tolerance) = workspace_f64(options)?;
    let mut output = NativeOutput::default();

    let ((), failure) = callback::with_f64(f, |trampoline| {
        driver(
            trampoline,
            &mut workspace,
            &mut output,
            &mut absolute_tolerance,
            &mut relative_tolerance,
        );
    })?;

    if let Some(failure) = failure {
        return Err(callback::failure_error(failure));
    }
    status(output.status)?;
    finish_f64(output, limit)
}

fn execute_f32<F, D>(
    f: F,
    options: IntegrationOptions,
    driver: D,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
    D: FnOnce(IntegrandF32, &mut Workspace<f32>, &mut NativeOutput<f32>, &mut f32, &mut f32),
{
    let limit = options.limit;
    let (mut workspace, mut absolute_tolerance, mut relative_tolerance) = workspace_f32(options)?;
    let mut output = NativeOutput::default();

    let ((), failure) = callback::with_f32(f, |trampoline| {
        driver(
            trampoline,
            &mut workspace,
            &mut output,
            &mut absolute_tolerance,
            &mut relative_tolerance,
        );
    })?;

    if let Some(failure) = failure {
        return Err(callback::failure_error(failure));
    }
    status(output.status)?;
    finish_f32(output, limit)
}

fn finish_f64(
    output: NativeOutput<f64>,
    limit: usize,
) -> Result<IntegrationResult<f64>, IntegrationError> {
    if !output.value.is_finite() || !output.error.is_finite() || output.error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(IntegrationResult {
        value: output.value,
        estimated_error: output.error,
        evaluations: output_count(output.evaluations, None)?,
        intervals: output_count(output.intervals, Some(limit))?,
    })
}

fn finish_f32(
    output: NativeOutput<f32>,
    limit: usize,
) -> Result<IntegrationResult<f32>, IntegrationError> {
    if !output.value.is_finite() || !output.error.is_finite() || output.error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(IntegrationResult {
        value: output.value,
        estimated_error: output.error,
        evaluations: output_count(output.evaluations, None)?,
        intervals: output_count(output.intervals, Some(limit))?,
    })
}

/// Integrates a function on a finite interval with adaptive Gauss-Kronrod rules.
///
/// Reversed bounds are supported and negate the result. The callback is serialized
/// with other stateful SLATEC calls for the supported GNU MinGW profile.
pub fn integrate<F>(
    f: F,
    lower: f64,
    upper: f64,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    finite_bounds(lower, upper)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut key = options.rule.key();

    execute_f64(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: bounds and tolerances were validated; all scalar and workspace pointers
        // are valid for the call; callback state is installed for this thread; the symbol
        // and callback ABI were validated for ffi-profile-gnu-mingw-x86_64.
        unsafe {
            slatec_sys::quadrature::dqag(
                callback,
                &mut lower,
                &mut upper,
                epsabs,
                epsrel,
                &mut key,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Single-precision counterpart of [`integrate`].
pub fn integrate_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    finite_bounds(lower, upper)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut key = options.rule.key();

    execute_f32(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: the same invariants as `integrate` hold for the validated REAL ABI.
        unsafe {
            slatec_sys::quadrature::qag(
                callback,
                &mut lower,
                &mut upper,
                epsabs,
                epsrel,
                &mut key,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Integrates a finite interval using the QAGS extrapolation strategy.
///
/// This is suitable for integrable endpoint singularities. `options.rule` is not
/// used because QAGS selects its own quadrature rule.
pub fn integrate_singular<F>(
    f: F,
    lower: f64,
    upper: f64,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    finite_bounds(lower, upper)?;
    let mut lower = lower;
    let mut upper = upper;

    execute_f64(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: validated finite bounds, tolerances, workspaces, callback lifetime,
        // ABI profile, and observed `dqags_` symbol satisfy the raw contract.
        unsafe {
            slatec_sys::quadrature::dqags(
                callback,
                &mut lower,
                &mut upper,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Single-precision counterpart of [`integrate_singular`].
pub fn integrate_singular_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    finite_bounds(lower, upper)?;
    let mut lower = lower;
    let mut upper = upper;

    execute_f32(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: the same invariants as `integrate_singular` hold for REAL.
        unsafe {
            slatec_sys::quadrature::qags(
                callback,
                &mut lower,
                &mut upper,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Integrates over a semi-infinite or whole-real-line interval using QAGI.
pub fn integrate_infinite<F>(
    f: F,
    interval: InfiniteInterval<f64>,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    let (mut bound, mut direction) = interval.native_parameters()?;

    execute_f64(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: interval encoding, tolerances, callback, workspace, and the GNU
        // MinGW callback ABI have been validated before calling observed `dqagi_`.
        unsafe {
            slatec_sys::quadrature::dqagi(
                callback,
                &mut bound,
                &mut direction,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Single-precision counterpart of [`integrate_infinite`].
pub fn integrate_infinite_f32<F>(
    f: F,
    interval: InfiniteInterval<f32>,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    let (mut bound, mut direction) = interval.native_parameters()?;

    execute_f32(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: the same invariants as `integrate_infinite` hold for REAL.
        unsafe {
            slatec_sys::quadrature::qagi(
                callback,
                &mut bound,
                &mut direction,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Computes a Cauchy principal-value integral using QAWC.
///
/// `singularity` must lie strictly between the endpoints. The callback supplies
/// `f(x)` for the QAWC convention `f(x) / (x - singularity)`.
pub fn integrate_principal_value<F>(
    f: F,
    lower: f64,
    upper: f64,
    singularity: f64,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    principal_value_bounds(lower, upper, singularity)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut singularity = singularity;

    execute_f64(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: singularity placement, bounds, tolerances, workspaces, callback
        // lifetime, ABI profile, and observed `dqawc_` symbol are all validated.
        unsafe {
            slatec_sys::quadrature::dqawc(
                callback,
                &mut lower,
                &mut upper,
                &mut singularity,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}

/// Single-precision counterpart of [`integrate_principal_value`].
pub fn integrate_principal_value_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    singularity: f32,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    principal_value_bounds(lower, upper, singularity)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut singularity = singularity;

    execute_f32(f, options, |callback, workspace, output, epsabs, epsrel| {
        // SAFETY: the same invariants as `integrate_principal_value` hold for REAL.
        unsafe {
            slatec_sys::quadrature::qawc(
                callback,
                &mut lower,
                &mut upper,
                &mut singularity,
                epsabs,
                epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.limit,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })
}
