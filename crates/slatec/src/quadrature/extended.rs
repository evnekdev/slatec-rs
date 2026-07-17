//! Safe adapters for the remaining reviewed public QUADPACK drivers.
//!
//! The workspace formulas below are the public-driver formulas from the
//! selected SLATEC sources. Expert `*E` drivers and their raw workspace views
//! deliberately remain unavailable.

#![allow(dead_code, unused_imports)] // One source module serves narrow families.

use alloc::vec;
use alloc::vec::Vec;

use super::callback;
use super::validation::{
    allocated_workspace, checked_add, checked_mul, finite_bounds, output_count,
    positive_tolerance_f32, positive_tolerance_f64, standard_tolerances_f32,
    standard_tolerances_f64, status, to_fortran, workspace_f32, workspace_f64,
};
use super::{
    EndpointWeight, FourierIntegrationResult, FourierOptions, IntegrationError, IntegrationOptions,
    IntegrationResult, Nc79IntegrationResult, Nc79Options, NonAdaptiveOptions, OscillatoryOptions,
    OscillatoryWeight,
};
use slatec_sys::FortranInteger;
use slatec_sys::quadrature::{IntegrandF32, IntegrandF64};

#[derive(Default)]
struct StandardOutput<T> {
    value: T,
    error: T,
    evaluations: FortranInteger,
    status: FortranInteger,
    intervals: FortranInteger,
}

fn invoke_f64<F>(f: F, call: impl FnOnce(IntegrandF64)) -> Result<(), IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    let ((), failure) = callback::with_f64(f, call)?;
    if let Some(failure) = failure {
        Err(callback::failure_error(failure))
    } else {
        Ok(())
    }
}

fn invoke_f32<F>(f: F, call: impl FnOnce(IntegrandF32)) -> Result<(), IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    let ((), failure) = callback::with_f32(f, call)?;
    if let Some(failure) = failure {
        Err(callback::failure_error(failure))
    } else {
        Ok(())
    }
}

fn finish_standard_f64(
    output: StandardOutput<f64>,
    limit: usize,
) -> Result<IntegrationResult<f64>, IntegrationError> {
    status(output.status)?;
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

fn finish_standard_f32(
    output: StandardOutput<f32>,
    limit: usize,
) -> Result<IntegrationResult<f32>, IntegrationError> {
    status(output.status)?;
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

#[cfg(feature = "quadrature-breakpoints")]
fn sorted_breakpoints<T: Into<f64> + Copy>(
    lower: T,
    upper: T,
    breakpoints: &[T],
) -> Result<Vec<(f64, usize)>, IntegrationError> {
    finite_bounds(lower, upper)?;
    let lower = lower.into();
    let upper = upper.into();
    let minimum = lower.min(upper);
    let maximum = lower.max(upper);
    let mut points = Vec::with_capacity(breakpoints.len());
    for (index, point) in breakpoints.iter().copied().enumerate() {
        let point = point.into();
        if !point.is_finite() || point <= minimum || point >= maximum {
            return Err(IntegrationError::InvalidBreakpoint {
                index,
                value: point,
            });
        }
        points.push((point, index));
    }
    points.sort_by(|left, right| left.0.total_cmp(&right.0));
    for pair in points.windows(2) {
        if pair[0].0 == pair[1].0 {
            return Err(IntegrationError::DuplicateBreakpoint {
                first: pair[0].1,
                second: pair[1].1,
            });
        }
    }
    Ok(points)
}

#[cfg(feature = "quadrature-breakpoints")]
fn breakpoint_lengths(
    limit: usize,
    count: usize,
) -> Result<(usize, usize, usize), IntegrationError> {
    // QAGP: NPTS2 = number of breakpoints + 2; LENIW = 2*LIMIT +
    // NPTS2; LENW = 2*LENIW - NPTS2. LIMIT must exceed NPTS2 - 2.
    let npts2 = checked_add(count, 2, false)?;
    let required_limit = checked_add(count, 1, false)?;
    if limit < required_limit {
        return Err(IntegrationError::WorkspaceTooLarge);
    }
    let leniw = checked_add(checked_mul(limit, 2, false)?, npts2, false)?;
    let lenw = checked_add(leniw, leniw, false)?
        .checked_sub(npts2)
        .ok_or(IntegrationError::WorkspaceTooLarge)?;
    Ok((npts2, leniw, lenw))
}

/// Integrates a finite interval while splitting at validated breakpoints.
///
/// Breakpoints may be given in any order. They are copied, sorted, and never
/// passed through from the caller because QAGP/DQAGP may reorder its local
/// points buffer. Each value must be finite, unique, and strictly interior.
#[cfg(feature = "quadrature-breakpoints")]
pub fn integrate_with_breakpoints<F>(
    f: F,
    lower: f64,
    upper: f64,
    breakpoints: &[f64],
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    let sorted = sorted_breakpoints(lower, upper, breakpoints)?;
    let (npts2, leniw, lenw) = breakpoint_lengths(options.limit, sorted.len())?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f64(options.absolute_tolerance, options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut npts2 = to_fortran(npts2, false)?;
    let mut leniw = to_fortran(leniw, false)?;
    let mut lenw = to_fortran(lenw, false)?;
    let mut points =
        vec![0.0; usize::try_from(npts2).map_err(|_| IntegrationError::IntegerOverflow)?];
    for (slot, (point, _)) in points.iter_mut().zip(sorted) {
        *slot = point;
    }
    let (mut iwork, mut work) = allocated_workspace(
        usize::try_from(leniw).map_err(|_| IntegrationError::IntegerOverflow)?,
        usize::try_from(lenw).map_err(|_| IntegrationError::IntegerOverflow)?,
        false,
    )?;
    let mut output = StandardOutput::default();
    invoke_f64(f, |callback| {
        // SAFETY: the copied point buffer, public QAGP workspace formula,
        // scalar values, callback scope, and GNU MinGW ABI are validated.
        unsafe {
            slatec_sys::quadrature::dqagp(
                callback,
                &mut lower,
                &mut upper,
                &mut npts2,
                points.as_mut_ptr(),
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut leniw,
                &mut lenw,
                &mut output.intervals,
                iwork.as_mut_ptr(),
                work.as_mut_ptr(),
            );
        }
    })?;
    finish_standard_f64(output, options.limit)
}

/// Single-precision counterpart of [`integrate_with_breakpoints`].
#[cfg(feature = "quadrature-breakpoints")]
pub fn integrate_with_breakpoints_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    breakpoints: &[f32],
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    let sorted = sorted_breakpoints(lower, upper, breakpoints)?;
    let (npts2, leniw, lenw) = breakpoint_lengths(options.limit, sorted.len())?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f32(options.absolute_tolerance, options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut npts2 = to_fortran(npts2, false)?;
    let mut leniw = to_fortran(leniw, false)?;
    let mut lenw = to_fortran(lenw, false)?;
    let mut points =
        vec![0.0_f32; usize::try_from(npts2).map_err(|_| IntegrationError::IntegerOverflow)?];
    for (slot, (point, _)) in points.iter_mut().zip(sorted) {
        *slot = point as f32;
    }
    let (mut iwork, mut work) = allocated_workspace(
        usize::try_from(leniw).map_err(|_| IntegrationError::IntegerOverflow)?,
        usize::try_from(lenw).map_err(|_| IntegrationError::IntegerOverflow)?,
        false,
    )?;
    let mut output = StandardOutput::default();
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_with_breakpoints` for the reviewed QAGP invariants.
        unsafe {
            slatec_sys::quadrature::qagp(
                callback,
                &mut lower,
                &mut upper,
                &mut npts2,
                points.as_mut_ptr(),
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut leniw,
                &mut lenw,
                &mut output.intervals,
                iwork.as_mut_ptr(),
                work.as_mut_ptr(),
            );
        }
    })?;
    finish_standard_f32(output, options.limit)
}

#[cfg(feature = "quadrature-weighted")]
fn weighted_bounds<T: Into<f64> + Copy>(
    lower: T,
    upper: T,
    alpha: T,
    beta: T,
) -> Result<(), IntegrationError> {
    finite_bounds(lower, upper)?;
    if lower.into() >= upper.into() {
        return Err(IntegrationError::InvalidBounds);
    }
    for (name, value) in [("alpha", alpha.into()), ("beta", beta.into())] {
        if !value.is_finite() || value <= -1.0 {
            return Err(IntegrationError::InvalidWeightParameter {
                argument: name,
                value,
            });
        }
    }
    Ok(())
}

/// Integrates `f(x)` with a selected algebraic/logarithmic endpoint weight.
///
/// QAWS/DQAWS require `lower < upper` and both exponents strictly greater than
/// `-1`; those native preconditions are checked before entering Fortran.
#[cfg(feature = "quadrature-weighted")]
pub fn integrate_weighted_endpoints<F>(
    f: F,
    lower: f64,
    upper: f64,
    alpha: f64,
    beta: f64,
    weight: EndpointWeight,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    weighted_bounds(lower, upper, alpha, beta)?;
    let (mut workspace, mut epsabs, mut epsrel) = workspace_f64(options)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut alpha = alpha;
    let mut beta = beta;
    let mut weight = weight.native_selector();
    let mut output = StandardOutput::default();
    invoke_f64(f, |callback| {
        // SAFETY: QAWS endpoint restrictions, standard workspace, callback lifetime,
        // and the reviewed profile ABI have been validated.
        unsafe {
            slatec_sys::quadrature::dqaws(
                callback,
                &mut lower,
                &mut upper,
                &mut alpha,
                &mut beta,
                &mut weight,
                &mut epsabs,
                &mut epsrel,
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
    })?;
    finish_standard_f64(output, options.limit)
}

/// Single-precision counterpart of [`integrate_weighted_endpoints`].
#[cfg(feature = "quadrature-weighted")]
pub fn integrate_weighted_endpoints_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    alpha: f32,
    beta: f32,
    weight: EndpointWeight,
    options: IntegrationOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    weighted_bounds(lower, upper, alpha, beta)?;
    let (mut workspace, mut epsabs, mut epsrel) = workspace_f32(options)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut alpha = alpha;
    let mut beta = beta;
    let mut weight = weight.native_selector();
    let mut output = StandardOutput::default();
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_weighted_endpoints` for reviewed QAWS invariants.
        unsafe {
            slatec_sys::quadrature::qaws(
                callback,
                &mut lower,
                &mut upper,
                &mut alpha,
                &mut beta,
                &mut weight,
                &mut epsabs,
                &mut epsrel,
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
    })?;
    finish_standard_f32(output, options.limit)
}

struct OscillatoryWorkspace<T> {
    leniw: FortranInteger,
    maxp1: FortranInteger,
    lenw: FortranInteger,
    iwork: Vec<FortranInteger>,
    work: Vec<T>,
}

#[cfg(feature = "quadrature-oscillatory")]
fn oscillatory_workspace<T: Default + Clone>(
    limit: usize,
    maximum_moments: usize,
) -> Result<OscillatoryWorkspace<T>, IntegrationError> {
    if limit == 0 || maximum_moments == 0 {
        return Err(IntegrationError::MomentWorkspaceTooLarge);
    }
    // QAWO: LENIW = 2*LIMIT; LENW = 2*LENIW + 25*MAXP1.
    let leniw = checked_mul(limit, 2, true)?;
    let moments = checked_mul(maximum_moments, 25, true)?;
    let lenw = checked_add(checked_mul(leniw, 2, true)?, moments, true)?;
    let (iwork, work) = allocated_workspace(leniw, lenw, true)?;
    Ok(OscillatoryWorkspace {
        leniw: to_fortran(leniw, true)?,
        maxp1: to_fortran(maximum_moments, true)?,
        lenw: to_fortran(lenw, true)?,
        iwork,
        work,
    })
}

#[cfg(feature = "quadrature-oscillatory")]
fn oscillatory_input<T: Into<f64> + Copy>(
    lower: T,
    upper: T,
    omega: T,
) -> Result<(), IntegrationError> {
    finite_bounds(lower, upper)?;
    let omega = omega.into();
    if omega.is_finite() {
        Ok(())
    } else {
        Err(IntegrationError::InvalidFrequency { value: omega })
    }
}

/// Integrates a finite interval with a sine or cosine weight.
///
/// The moment table is allocated from QAWO's documented formula and discarded
/// after the call. A zero frequency is forwarded: cosine becomes the unweighted
/// integral and sine produces the native zero-weight result.
#[cfg(feature = "quadrature-oscillatory")]
pub fn integrate_oscillatory<F>(
    f: F,
    lower: f64,
    upper: f64,
    omega: f64,
    weight: OscillatoryWeight,
    options: OscillatoryOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    oscillatory_input(lower, upper, omega)?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f64(options.absolute_tolerance, options.relative_tolerance)?;
    let mut workspace = oscillatory_workspace(options.subdivision_limit, options.maximum_moments)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut omega = omega;
    let mut weight = weight.native_selector();
    let mut output = StandardOutput::default();
    invoke_f64(f, |callback| {
        // SAFETY: QAWO's documented LENIW/LENW moment formula, scalar inputs,
        // callback lifetime, and reviewed profile ABI are all upheld.
        unsafe {
            slatec_sys::quadrature::dqawo(
                callback,
                &mut lower,
                &mut upper,
                &mut omega,
                &mut weight,
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.leniw,
                &mut workspace.maxp1,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })?;
    finish_standard_f64(output, options.subdivision_limit)
}

/// Single-precision counterpart of [`integrate_oscillatory`].
#[cfg(feature = "quadrature-oscillatory")]
pub fn integrate_oscillatory_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    omega: f32,
    weight: OscillatoryWeight,
    options: OscillatoryOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    oscillatory_input(lower, upper, omega)?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f32(options.absolute_tolerance, options.relative_tolerance)?;
    let mut workspace = oscillatory_workspace(options.subdivision_limit, options.maximum_moments)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut omega = omega;
    let mut weight = weight.native_selector();
    let mut output = StandardOutput::default();
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_oscillatory` for the reviewed QAWO invariants.
        unsafe {
            slatec_sys::quadrature::qawo(
                callback,
                &mut lower,
                &mut upper,
                &mut omega,
                &mut weight,
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
                &mut workspace.leniw,
                &mut workspace.maxp1,
                &mut workspace.lenw,
                &mut output.intervals,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })?;
    finish_standard_f32(output, options.subdivision_limit)
}

struct FourierWorkspace<T> {
    limlst: FortranInteger,
    leniw: FortranInteger,
    maxp1: FortranInteger,
    lenw: FortranInteger,
    iwork: Vec<FortranInteger>,
    work: Vec<T>,
}

#[cfg(feature = "quadrature-fourier")]
fn fourier_workspace<T: Default + Clone>(
    options: FourierOptions,
) -> Result<FourierWorkspace<T>, IntegrationError> {
    if options.subdivision_limit == 0 || options.cycle_limit < 3 || options.maximum_moments == 0 {
        return Err(IntegrationError::MomentWorkspaceTooLarge);
    }
    // QAWF: LENIW = LIMLST + 2*LIMIT; LENW = 2*LENIW + 25*MAXP1.
    let leniw = checked_add(
        options.cycle_limit,
        checked_mul(options.subdivision_limit, 2, true)?,
        true,
    )?;
    let moments = checked_mul(options.maximum_moments, 25, true)?;
    let lenw = checked_add(checked_mul(leniw, 2, true)?, moments, true)?;
    let (iwork, work) = allocated_workspace(leniw, lenw, true)?;
    Ok(FourierWorkspace {
        limlst: to_fortran(options.cycle_limit, true)?,
        leniw: to_fortran(leniw, true)?,
        maxp1: to_fortran(options.maximum_moments, true)?,
        lenw: to_fortran(lenw, true)?,
        iwork,
        work,
    })
}

#[cfg(feature = "quadrature-fourier")]
fn fourier_status(status_code: FortranInteger) -> Result<(), IntegrationError> {
    match status_code {
        0 => Ok(()),
        1 => Err(IntegrationError::MaximumCyclesReached),
        4 => Err(IntegrationError::NonConvergence),
        6 => Err(IntegrationError::NativeContractViolation),
        7 => Err(IntegrationError::BadCycleBehavior),
        value => Err(IntegrationError::NativeStatus(value)),
    }
}

#[cfg(feature = "quadrature-fourier")]
fn finish_fourier_f64(
    value: f64,
    error: f64,
    evaluations: FortranInteger,
    cycles: FortranInteger,
    cycle_limit: usize,
) -> Result<FourierIntegrationResult<f64>, IntegrationError> {
    if !value.is_finite() || !error.is_finite() || error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(FourierIntegrationResult {
        value,
        estimated_error: error,
        evaluations: output_count(evaluations, None)?,
        cycles: output_count(cycles, Some(cycle_limit))?,
    })
}

#[cfg(feature = "quadrature-fourier")]
fn finish_fourier_f32(
    value: f32,
    error: f32,
    evaluations: FortranInteger,
    cycles: FortranInteger,
    cycle_limit: usize,
) -> Result<FourierIntegrationResult<f32>, IntegrationError> {
    if !value.is_finite() || !error.is_finite() || error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(FourierIntegrationResult {
        value,
        estimated_error: error,
        evaluations: output_count(evaluations, None)?,
        cycles: output_count(cycles, Some(cycle_limit))?,
    })
}

/// Integrates a decaying function's infinite sine or cosine Fourier tail.
///
/// QAWF/DQAWF require a positive absolute tolerance and at least three cycles.
/// For a zero sine frequency, the integral is exactly zero and no callback is
/// invoked. Zero-frequency cosine uses the driver's documented QAGI path.
#[cfg(feature = "quadrature-fourier")]
pub fn integrate_fourier_tail<F>(
    f: F,
    lower_bound: f64,
    omega: f64,
    weight: OscillatoryWeight,
    options: FourierOptions,
) -> Result<FourierIntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    if !lower_bound.is_finite() {
        return Err(IntegrationError::InvalidBounds);
    }
    if !omega.is_finite() {
        return Err(IntegrationError::InvalidFrequency { value: omega });
    }
    let mut epsabs = positive_tolerance_f64(options.absolute_tolerance)?;
    let mut workspace = fourier_workspace(options)?;
    if omega == 0.0 && weight == OscillatoryWeight::Sine {
        return Ok(FourierIntegrationResult {
            value: 0.0,
            estimated_error: 0.0,
            evaluations: 0,
            cycles: 0,
        });
    }
    let mut lower_bound = lower_bound;
    let mut omega = omega;
    let mut weight = weight.native_selector();
    let mut value = 0.0;
    let mut error = 0.0;
    let mut evaluations = 0;
    let mut status_code = 0;
    let mut cycles = 0;
    invoke_f64(f, |callback| {
        // SAFETY: QAWF's cycle, subdivision, and moment-table formulas;
        // callback lifetime; and profile ABI are validated before this call.
        unsafe {
            slatec_sys::quadrature::dqawf(
                callback,
                &mut lower_bound,
                &mut omega,
                &mut weight,
                &mut epsabs,
                &mut value,
                &mut error,
                &mut evaluations,
                &mut status_code,
                &mut workspace.limlst,
                &mut cycles,
                &mut workspace.leniw,
                &mut workspace.maxp1,
                &mut workspace.lenw,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })?;
    if omega == 0.0 && weight == 1 {
        status(
            *workspace
                .iwork
                .first()
                .ok_or(IntegrationError::NativeContractViolation)?,
        )?;
    } else {
        fourier_status(status_code)?;
    }
    finish_fourier_f64(value, error, evaluations, cycles, options.cycle_limit)
}

/// Single-precision counterpart of [`integrate_fourier_tail`].
#[cfg(feature = "quadrature-fourier")]
pub fn integrate_fourier_tail_f32<F>(
    f: F,
    lower_bound: f32,
    omega: f32,
    weight: OscillatoryWeight,
    options: FourierOptions,
) -> Result<FourierIntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    if !lower_bound.is_finite() {
        return Err(IntegrationError::InvalidBounds);
    }
    if !omega.is_finite() {
        return Err(IntegrationError::InvalidFrequency {
            value: f64::from(omega),
        });
    }
    let mut epsabs = positive_tolerance_f32(options.absolute_tolerance)?;
    let mut workspace = fourier_workspace(options)?;
    if omega == 0.0 && weight == OscillatoryWeight::Sine {
        return Ok(FourierIntegrationResult {
            value: 0.0,
            estimated_error: 0.0,
            evaluations: 0,
            cycles: 0,
        });
    }
    let mut lower_bound = lower_bound;
    let mut omega = omega;
    let mut weight = weight.native_selector();
    let mut value = 0.0_f32;
    let mut error = 0.0_f32;
    let mut evaluations = 0;
    let mut status_code = 0;
    let mut cycles = 0;
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_fourier_tail` for reviewed QAWF invariants.
        unsafe {
            slatec_sys::quadrature::qawf(
                callback,
                &mut lower_bound,
                &mut omega,
                &mut weight,
                &mut epsabs,
                &mut value,
                &mut error,
                &mut evaluations,
                &mut status_code,
                &mut workspace.limlst,
                &mut cycles,
                &mut workspace.leniw,
                &mut workspace.maxp1,
                &mut workspace.lenw,
                workspace.iwork.as_mut_ptr(),
                workspace.work.as_mut_ptr(),
            );
        }
    })?;
    if omega == 0.0 && weight == 1 {
        status(
            *workspace
                .iwork
                .first()
                .ok_or(IntegrationError::NativeContractViolation)?,
        )?;
    } else {
        fourier_status(status_code)?;
    }
    finish_fourier_f32(value, error, evaluations, cycles, options.cycle_limit)
}

#[cfg(feature = "quadrature-nonadaptive")]
fn non_adaptive_status(status_code: FortranInteger) -> Result<(), IntegrationError> {
    match status_code {
        0 => Ok(()),
        1 => Err(IntegrationError::NonAdaptiveAccuracyNotReached),
        6 => Err(IntegrationError::NativeContractViolation),
        value => Err(IntegrationError::NativeStatus(value)),
    }
}

/// Integrates a smooth finite interval with QNG/DQNG's non-adaptive nested rules.
///
/// The `intervals` field in the returned result is zero because QNG does not
/// create an adaptive subdivision partition.
#[cfg(feature = "quadrature-nonadaptive")]
pub fn integrate_non_adaptive<F>(
    f: F,
    lower: f64,
    upper: f64,
    options: NonAdaptiveOptions,
) -> Result<IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    finite_bounds(lower, upper)?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f64(options.absolute_tolerance, options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut output = StandardOutput::default();
    invoke_f64(f, |callback| {
        // SAFETY: QNG has scalar-only arguments; tolerance and callback ABI are validated.
        unsafe {
            slatec_sys::quadrature::dqng(
                callback,
                &mut lower,
                &mut upper,
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
            );
        }
    })?;
    non_adaptive_status(output.status)?;
    if !output.value.is_finite() || !output.error.is_finite() || output.error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(IntegrationResult {
        value: output.value,
        estimated_error: output.error,
        evaluations: output_count(output.evaluations, None)?,
        intervals: 0,
    })
}

/// Single-precision counterpart of [`integrate_non_adaptive`].
#[cfg(feature = "quadrature-nonadaptive")]
pub fn integrate_non_adaptive_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    options: NonAdaptiveOptions,
) -> Result<IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    finite_bounds(lower, upper)?;
    let (mut epsabs, mut epsrel) =
        standard_tolerances_f32(options.absolute_tolerance, options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut output = StandardOutput::default();
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_non_adaptive` for reviewed QNG invariants.
        unsafe {
            slatec_sys::quadrature::qng(
                callback,
                &mut lower,
                &mut upper,
                &mut epsabs,
                &mut epsrel,
                &mut output.value,
                &mut output.error,
                &mut output.evaluations,
                &mut output.status,
            );
        }
    })?;
    non_adaptive_status(output.status)?;
    if !output.value.is_finite() || !output.error.is_finite() || output.error < 0.0 {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(IntegrationResult {
        value: output.value,
        estimated_error: output.error,
        evaluations: output_count(output.evaluations, None)?,
        intervals: 0,
    })
}

#[cfg(feature = "quadrature-nonadaptive")]
fn nc79_tolerance_f64(value: f64) -> Result<f64, IntegrationError> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(IntegrationError::InvalidTolerance)
    }
}

#[cfg(feature = "quadrature-nonadaptive")]
fn nc79_tolerance_f32(value: f64) -> Result<f32, IntegrationError> {
    let value = nc79_tolerance_f64(value)?;
    if value > f64::from(f32::MAX) {
        return Err(IntegrationError::InvalidTolerance);
    }
    Ok(value as f32)
}

#[cfg(feature = "quadrature-nonadaptive")]
fn nc79_status(status_code: FortranInteger) -> Result<(), IntegrationError> {
    match status_code {
        1 => Ok(()),
        -1 => Err(IntegrationError::DegenerateInterval),
        2 => Err(IntegrationError::NonAdaptiveAccuracyNotReached),
        value => Err(IntegrationError::NativeStatus(value)),
    }
}

/// Integrates with SLATEC's adaptive seven-point Newton-Cotes QNC79 routine.
///
/// QNC79 has no workspace or independent error-estimate output. Its result
/// therefore reports only the value and number of callback evaluations.
#[cfg(feature = "quadrature-nonadaptive")]
pub fn integrate_nc79<F>(
    f: F,
    lower: f64,
    upper: f64,
    options: Nc79Options,
) -> Result<Nc79IntegrationResult<f64>, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    finite_bounds(lower, upper)?;
    if lower == upper {
        return Err(IntegrationError::DegenerateInterval);
    }
    let mut tolerance = nc79_tolerance_f64(options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut value = 0.0;
    let mut status_code = 0;
    let mut evaluations = 0;
    invoke_f64(f, |callback| {
        // SAFETY: QNC79's scalar callback ABI and simple no-workspace contract
        // were directly reviewed; native process-global state is held by the bridge.
        unsafe {
            slatec_sys::quadrature::dqnc79(
                callback,
                &mut lower,
                &mut upper,
                &mut tolerance,
                &mut value,
                &mut status_code,
                &mut evaluations,
            );
        }
    })?;
    nc79_status(status_code)?;
    if !value.is_finite() {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(Nc79IntegrationResult {
        value,
        evaluations: output_count(evaluations, None)?,
    })
}

/// Single-precision counterpart of [`integrate_nc79`].
#[cfg(feature = "quadrature-nonadaptive")]
pub fn integrate_nc79_f32<F>(
    f: F,
    lower: f32,
    upper: f32,
    options: Nc79Options,
) -> Result<Nc79IntegrationResult<f32>, IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    finite_bounds(lower, upper)?;
    if lower == upper {
        return Err(IntegrationError::DegenerateInterval);
    }
    let mut tolerance = nc79_tolerance_f32(options.relative_tolerance)?;
    let mut lower = lower;
    let mut upper = upper;
    let mut value = 0.0_f32;
    let mut status_code = 0;
    let mut evaluations = 0;
    invoke_f32(f, |callback| {
        // SAFETY: see `integrate_nc79` for the reviewed QNC79 invariants.
        unsafe {
            slatec_sys::quadrature::qnc79(
                callback,
                &mut lower,
                &mut upper,
                &mut tolerance,
                &mut value,
                &mut status_code,
                &mut evaluations,
            );
        }
    })?;
    nc79_status(status_code)?;
    if !value.is_finite() {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(Nc79IntegrationResult {
        value,
        evaluations: output_count(evaluations, None)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_breakpoints_and_public_workspace_formulas() {
        let points = sorted_breakpoints(-1.0, 1.0, &[0.5, -0.5]).unwrap();
        assert_eq!(
            points.iter().map(|point| point.0).collect::<Vec<_>>(),
            [-0.5, 0.5]
        );
        assert!(matches!(
            sorted_breakpoints(-1.0, 1.0, &[0.0, 0.0]),
            Err(IntegrationError::DuplicateBreakpoint { .. })
        ));
        assert!(matches!(
            sorted_breakpoints(-1.0, 1.0, &[-1.0]),
            Err(IntegrationError::InvalidBreakpoint { .. })
        ));
        assert_eq!(breakpoint_lengths(3, 2).unwrap(), (4, 10, 16));
        assert!(breakpoint_lengths(2, 2).is_err());
    }

    #[test]
    fn validates_weight_and_moment_configuration() {
        assert!(weighted_bounds(0.0, 1.0, -0.5, 0.0).is_ok());
        assert!(weighted_bounds(1.0, 0.0, 0.0, 0.0).is_err());
        assert!(weighted_bounds(0.0, 1.0, -1.0, 0.0).is_err());
        let workspace: OscillatoryWorkspace<f64> = oscillatory_workspace(4, 3).unwrap();
        assert_eq!(workspace.iwork.len(), 8);
        assert_eq!(workspace.work.len(), 91);
        assert!(
            fourier_workspace::<f64>(FourierOptions {
                subdivision_limit: 1,
                cycle_limit: 2,
                maximum_moments: 1,
                ..FourierOptions::default()
            })
            .is_err()
        );
    }

    #[test]
    fn maps_distinct_public_statuses() {
        assert_eq!(
            fourier_status(1),
            Err(IntegrationError::MaximumCyclesReached)
        );
        assert_eq!(fourier_status(7), Err(IntegrationError::BadCycleBehavior));
        assert_eq!(
            non_adaptive_status(1),
            Err(IntegrationError::NonAdaptiveAccuracyNotReached)
        );
        assert_eq!(nc79_status(-1), Err(IntegrationError::DegenerateInterval));
    }
}
