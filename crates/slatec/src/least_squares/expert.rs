//! Safe expert `SNLS1` and `DNLS1` nonlinear least-squares drivers.

use alloc::{vec, vec::Vec};

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::callback_runtime::{
    self, CallbackRuntimeError, ExpertLeastSquaresCallbackFailure, ExpertLeastSquaresF32Callback,
    ExpertLeastSquaresF64Callback,
};
use crate::nonlinear::JacobianMut;

use super::{LeastSquaresError, LeastSquaresStatus};

/// Variable-scaling policy for expert nonlinear least squares.
///
/// This idiomatic enum replaces the raw `MODE` and `DIAG` arguments of
/// `SNLS1` and `DNLS1`. `Automatic` maps to `MODE=1`; `User` copies the
/// provided positive scaling values into `DIAG` and maps to `MODE=2`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeastSquaresScaling<'a, T> {
    /// Ask SLATEC to derive positive variable scales from the initial
    /// Jacobian. This maps to Fortran `MODE=1`.
    Automatic,
    /// Use one strictly positive, finite scale per parameter. The wrapper
    /// copies the slice because the native routine may update `DIAG`.
    User(&'a [T]),
}

/// Expert controls for `SNLS1` and `DNLS1`.
///
/// The fields map directly to the documented native controls: `FTOL`, `XTOL`,
/// `GTOL`, `MAXFEV`, `EPSFCN`, `DIAG`/`MODE`, and `FACTOR`. The wrapper fixes
/// `NPRINT=0`, so legacy print/observer callbacks cannot enter Rust. For
/// forward differences, `finite_difference_step=None` maps `EPSFCN=0`, which
/// makes SLATEC use its validated machine precision. A positive value tells
/// SLATEC the estimated relative error in residual evaluations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpertLeastSquaresOptions<'a, T = f64> {
    /// Relative reduction tolerance mapped to Fortran `FTOL`.
    pub function_tolerance: T,
    /// Relative parameter-step tolerance mapped to Fortran `XTOL`.
    pub parameter_tolerance: T,
    /// Residual/Jacobian orthogonality tolerance mapped to Fortran `GTOL`.
    /// Zero is the recommended conservative default.
    pub gradient_tolerance: T,
    /// Optional `MAXFEV` residual-callback budget. `None` selects the native
    /// recommendations: `200*(N+1)` for forward differences and `100*(N+1)`
    /// for a dense user Jacobian.
    pub maximum_function_evaluations: Option<usize>,
    /// Optional non-negative `EPSFCN` finite-difference relative-error
    /// estimate. It is ignored for a user-supplied Jacobian.
    pub finite_difference_step: Option<T>,
    /// Automatic or user-provided positive variable scaling (`MODE`/`DIAG`).
    pub scaling: LeastSquaresScaling<'a, T>,
    /// Positive `FACTOR` initial trust-region bound multiplier. SLATEC
    /// recommends a value in approximately `0.1..=100`; the safe layer accepts
    /// any finite positive value documented by the native contract.
    pub step_bound_factor: T,
}

impl Default for ExpertLeastSquaresOptions<'static, f64> {
    fn default() -> Self {
        Self {
            function_tolerance: 1.0e-10,
            parameter_tolerance: 1.0e-10,
            gradient_tolerance: 0.0,
            maximum_function_evaluations: None,
            finite_difference_step: None,
            scaling: LeastSquaresScaling::Automatic,
            step_bound_factor: 100.0,
        }
    }
}

impl ExpertLeastSquaresOptions<'static, f32> {
    /// Returns practical single-precision controls for `SNLS1`.
    pub const fn single_precision() -> Self {
        Self {
            function_tolerance: 1.0e-5,
            parameter_tolerance: 1.0e-5,
            gradient_tolerance: 0.0,
            maximum_function_evaluations: None,
            finite_difference_step: None,
            scaling: LeastSquaresScaling::Automatic,
            step_bound_factor: 100.0,
        }
    }
}

/// Result of an expert nonlinear least-squares solve.
///
/// `parameters` and `residuals` are the final Fortran `X` and `FVEC` values.
/// The wrapper computes the scaled Euclidean `residual_norm` from `FVEC` and
/// `cost = residual_norm^2 / 2` in Rust. The evaluation counts are the native
/// `NFEV` and `NJEV` outputs, checked against the contained Rust callback
/// invocation counts. In forward-difference mode `jacobian_evaluations` is
/// precisely zero, as documented by `SNLS1`/`DNLS1`.
#[derive(Clone, Debug, PartialEq)]
pub struct ExpertLeastSquaresResult<T = f64> {
    /// Final parameter estimate from Fortran argument `X`, of length `N`.
    pub parameters: Vec<T>,
    /// Final residual vector from Fortran argument `FVEC`, of length `M`.
    pub residuals: Vec<T>,
    /// One half of the recomputed residual sum of squares.
    pub cost: T,
    /// Recomputed Euclidean norm of [`Self::residuals`].
    pub residual_norm: T,
    /// Interpreted native `INFO` numerical completion state.
    pub status: LeastSquaresStatus,
    /// Exact native function-evaluation count `NFEV`.
    pub function_evaluations: usize,
    /// Exact native full-Jacobian evaluation count `NJEV`.
    pub jacobian_evaluations: usize,
}

struct Workspace<T> {
    jacobian: Vec<T>,
    scaling: Vec<T>,
    qtf: Vec<T>,
    wa1: Vec<T>,
    wa2: Vec<T>,
    wa3: Vec<T>,
    wa4: Vec<T>,
    pivots: Vec<FortranInteger>,
}

fn matrix_len(residual_count: usize, parameter_count: usize) -> Result<usize, LeastSquaresError> {
    residual_count
        .checked_mul(parameter_count)
        .ok_or(LeastSquaresError::WorkspaceOverflow)
}

fn native_integer(
    value: usize,
    argument: &'static str,
) -> Result<FortranInteger, LeastSquaresError> {
    to_fortran_integer(value).map_err(|_| LeastSquaresError::IntegerOverflow { argument })
}

fn native_count(value: FortranInteger, detail: &'static str) -> Result<usize, LeastSquaresError> {
    usize::try_from(value).map_err(|_| LeastSquaresError::NativeContractViolation { detail })
}

fn status(value: FortranInteger) -> Result<LeastSquaresStatus, LeastSquaresError> {
    match value {
        1 => Ok(LeastSquaresStatus::ConvergedResidual),
        2 => Ok(LeastSquaresStatus::ConvergedParameters),
        3 => Ok(LeastSquaresStatus::ConvergedResidualAndParameters),
        4 => Ok(LeastSquaresStatus::ConvergedOrthogonality),
        5 => Ok(LeastSquaresStatus::MaximumEvaluations),
        6 => Ok(LeastSquaresStatus::ResidualToleranceTooSmall),
        7 => Ok(LeastSquaresStatus::ParameterToleranceTooSmall),
        8 => Ok(LeastSquaresStatus::GradientToleranceTooSmall),
        value => Err(LeastSquaresError::NativeStatus { status: value }),
    }
}

fn callback_runtime_error(error: CallbackRuntimeError) -> LeastSquaresError {
    match error {
        CallbackRuntimeError::NestedCallback => LeastSquaresError::NestedNativeCallback,
    }
}

fn callback_failure(failure: ExpertLeastSquaresCallbackFailure) -> LeastSquaresError {
    match failure {
        ExpertLeastSquaresCallbackFailure::ResidualPanicked => LeastSquaresError::CallbackPanicked,
        ExpertLeastSquaresCallbackFailure::JacobianPanicked => LeastSquaresError::JacobianPanicked,
        ExpertLeastSquaresCallbackFailure::ResidualNonFinite { index } => {
            LeastSquaresError::CallbackReturnedNonFinite { index }
        }
        ExpertLeastSquaresCallbackFailure::JacobianNonFinite { row, column } => {
            LeastSquaresError::JacobianReturnedNonFinite { row, column }
        }
        ExpertLeastSquaresCallbackFailure::InvalidPointer => {
            LeastSquaresError::NativeContractViolation {
                detail: "expert least-squares callback pointer was null or aliased",
            }
        }
        ExpertLeastSquaresCallbackFailure::DimensionMismatch => {
            LeastSquaresError::NativeContractViolation {
                detail: "expert least-squares callback M or N differed from its registered context",
            }
        }
        ExpertLeastSquaresCallbackFailure::InvalidLeadingDimension => {
            LeastSquaresError::NativeContractViolation {
                detail: "expert least-squares callback LDFJAC was invalid",
            }
        }
        ExpertLeastSquaresCallbackFailure::UnexpectedFlag => {
            LeastSquaresError::NativeContractViolation {
                detail: "expert least-squares callback received an unsupported IFLAG",
            }
        }
    }
}

fn norm_f64(values: &[f64]) -> f64 {
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
                sum = 1.0 + sum * (scale / magnitude).powi(2);
                scale = magnitude;
            } else {
                sum += (magnitude / scale).powi(2);
            }
        }
    }
    if scale == 0.0 {
        0.0
    } else {
        scale * sum.sqrt()
    }
}

fn default_maximum(parameter_count: usize, analytic: bool) -> Result<usize, LeastSquaresError> {
    parameter_count
        .checked_add(1)
        .and_then(|value| value.checked_mul(if analytic { 100 } else { 200 }))
        .ok_or(LeastSquaresError::WorkspaceOverflow)
}

fn validate_f64(
    initial: &[f64],
    residual_count: usize,
    options: ExpertLeastSquaresOptions<'_, f64>,
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
    if !options.function_tolerance.is_finite() || options.function_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidFunctionTolerance);
    }
    if !options.parameter_tolerance.is_finite() || options.parameter_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidParameterTolerance);
    }
    if !options.gradient_tolerance.is_finite() || options.gradient_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidGradientTolerance);
    }
    if options
        .finite_difference_step
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err(LeastSquaresError::InvalidFiniteDifferenceStep);
    }
    if !options.step_bound_factor.is_finite() || options.step_bound_factor <= 0.0 {
        return Err(LeastSquaresError::InvalidStepBoundFactor);
    }
    if let LeastSquaresScaling::User(values) = options.scaling {
        if values.len() != initial.len() {
            return Err(LeastSquaresError::InvalidScalingLength {
                expected: initial.len(),
                actual: values.len(),
            });
        }
        if let Some((index, _)) = values
            .iter()
            .enumerate()
            .find(|(_, value)| !value.is_finite() || **value <= 0.0)
        {
            return Err(LeastSquaresError::InvalidScalingValue { index });
        }
    }
    Ok(())
}

fn validate_f32(
    initial: &[f32],
    residual_count: usize,
    options: ExpertLeastSquaresOptions<'_, f32>,
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
    if !options.function_tolerance.is_finite() || options.function_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidFunctionTolerance);
    }
    if !options.parameter_tolerance.is_finite() || options.parameter_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidParameterTolerance);
    }
    if !options.gradient_tolerance.is_finite() || options.gradient_tolerance < 0.0 {
        return Err(LeastSquaresError::InvalidGradientTolerance);
    }
    if options
        .finite_difference_step
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err(LeastSquaresError::InvalidFiniteDifferenceStep);
    }
    if !options.step_bound_factor.is_finite() || options.step_bound_factor <= 0.0 {
        return Err(LeastSquaresError::InvalidStepBoundFactor);
    }
    if let LeastSquaresScaling::User(values) = options.scaling {
        if values.len() != initial.len() {
            return Err(LeastSquaresError::InvalidScalingLength {
                expected: initial.len(),
                actual: values.len(),
            });
        }
        if let Some((index, _)) = values
            .iter()
            .enumerate()
            .find(|(_, value)| !value.is_finite() || **value <= 0.0)
        {
            return Err(LeastSquaresError::InvalidScalingValue { index });
        }
    }
    Ok(())
}

fn scaling_f64(
    parameter_count: usize,
    scaling: LeastSquaresScaling<'_, f64>,
) -> (Vec<f64>, FortranInteger) {
    match scaling {
        LeastSquaresScaling::Automatic => (vec![0.0; parameter_count], 1),
        LeastSquaresScaling::User(values) => (values.to_vec(), 2),
    }
}

fn scaling_f32(
    parameter_count: usize,
    scaling: LeastSquaresScaling<'_, f32>,
) -> (Vec<f32>, FortranInteger) {
    match scaling {
        LeastSquaresScaling::Automatic => (vec![0.0; parameter_count], 1),
        LeastSquaresScaling::User(values) => (values.to_vec(), 2),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_f64<F, J>(
    initial: &[f64],
    residual_count: usize,
    residual_callback: F,
    jacobian_callback: J,
    options: ExpertLeastSquaresOptions<'_, f64>,
    analytic: bool,
) -> Result<ExpertLeastSquaresResult<f64>, LeastSquaresError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], &mut [f64], usize),
{
    validate_f64(initial, residual_count, options)?;
    let parameter_count = initial.len();
    let maximum = options
        .maximum_function_evaluations
        .unwrap_or(default_maximum(parameter_count, analytic)?);
    if maximum == 0 {
        return Err(LeastSquaresError::InvalidMaximumFunctionEvaluations);
    }
    let matrix_len = matrix_len(residual_count, parameter_count)?;
    let (scaling, mut mode) = scaling_f64(parameter_count, options.scaling);
    let mut workspace = Workspace {
        jacobian: vec![0.0; matrix_len],
        scaling,
        qtf: vec![0.0; parameter_count],
        wa1: vec![0.0; parameter_count],
        wa2: vec![0.0; parameter_count],
        wa3: vec![0.0; parameter_count],
        wa4: vec![0.0; residual_count],
        pivots: vec![0; parameter_count],
    };
    let mut parameters = initial.to_vec();
    let mut residuals = vec![0.0; residual_count];
    let mut iopt = if analytic { 2 } else { 1 };
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut ldfjac = m;
    let mut ftol = options.function_tolerance;
    let mut xtol = options.parameter_tolerance;
    let mut gtol = options.gradient_tolerance;
    let mut maxfev = native_integer(maximum, "maximum function evaluations")?;
    let mut epsfcn = options.finite_difference_step.unwrap_or(0.0);
    let mut factor = options.step_bound_factor;
    let mut nprint = 0;
    let mut info = 0;
    let mut nfev = 0;
    let mut njev = 0;
    let invocation = callback_runtime::with_expert_least_squares_f64(
        parameter_count,
        residual_count,
        analytic,
        residual_callback,
        jacobian_callback,
        |callback: ExpertLeastSquaresF64Callback| {
            let _error_scope = crate::runtime::permit_recoverable_native_statuses();
            // SAFETY: M, N, LDFJAC=M and all exact native arrays are checked;
            // the scoped callback validates residual/Jacobian pointers and
            // dimensions; NPRINT=0; and the selected GNU MinGW profile owns
            // this reviewed DNLS1 ABI.
            unsafe {
                slatec_sys::least_squares::dnls1(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    parameters.as_mut_ptr(),
                    residuals.as_mut_ptr(),
                    workspace.jacobian.as_mut_ptr(),
                    &mut ldfjac,
                    &mut ftol,
                    &mut xtol,
                    &mut gtol,
                    &mut maxfev,
                    &mut epsfcn,
                    workspace.scaling.as_mut_ptr(),
                    &mut mode,
                    &mut factor,
                    &mut nprint,
                    &mut info,
                    &mut nfev,
                    &mut njev,
                    workspace.pivots.as_mut_ptr(),
                    workspace.qtf.as_mut_ptr(),
                    workspace.wa1.as_mut_ptr(),
                    workspace.wa2.as_mut_ptr(),
                    workspace.wa3.as_mut_ptr(),
                    workspace.wa4.as_mut_ptr(),
                );
            }
        },
    )
    .map_err(callback_runtime_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    let function_evaluations = native_count(nfev, "negative native NFEV")?;
    let jacobian_evaluations = native_count(njev, "negative native NJEV")?;
    if function_evaluations != invocation.residual_evaluations
        || jacobian_evaluations != invocation.jacobian_evaluations
    {
        return Err(LeastSquaresError::NativeContractViolation {
            detail: "native expert evaluation counts disagreed with the scoped callback counts",
        });
    }
    let residual_norm = norm_f64(&residuals);
    Ok(ExpertLeastSquaresResult {
        parameters,
        residuals,
        cost: 0.5 * residual_norm * residual_norm,
        residual_norm,
        status: status(info)?,
        function_evaluations,
        jacobian_evaluations,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_f32<F, J>(
    initial: &[f32],
    residual_count: usize,
    residual_callback: F,
    jacobian_callback: J,
    options: ExpertLeastSquaresOptions<'_, f32>,
    analytic: bool,
) -> Result<ExpertLeastSquaresResult<f32>, LeastSquaresError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], &mut [f32], usize),
{
    validate_f32(initial, residual_count, options)?;
    let parameter_count = initial.len();
    let maximum = options
        .maximum_function_evaluations
        .unwrap_or(default_maximum(parameter_count, analytic)?);
    if maximum == 0 {
        return Err(LeastSquaresError::InvalidMaximumFunctionEvaluations);
    }
    let matrix_len = matrix_len(residual_count, parameter_count)?;
    let (scaling, mut mode) = scaling_f32(parameter_count, options.scaling);
    let mut workspace = Workspace {
        jacobian: vec![0.0; matrix_len],
        scaling,
        qtf: vec![0.0; parameter_count],
        wa1: vec![0.0; parameter_count],
        wa2: vec![0.0; parameter_count],
        wa3: vec![0.0; parameter_count],
        wa4: vec![0.0; residual_count],
        pivots: vec![0; parameter_count],
    };
    let mut parameters = initial.to_vec();
    let mut residuals = vec![0.0; residual_count];
    let mut iopt = if analytic { 2 } else { 1 };
    let mut m = native_integer(residual_count, "residual count")?;
    let mut n = native_integer(parameter_count, "parameter count")?;
    let mut ldfjac = m;
    let mut ftol = options.function_tolerance;
    let mut xtol = options.parameter_tolerance;
    let mut gtol = options.gradient_tolerance;
    let mut maxfev = native_integer(maximum, "maximum function evaluations")?;
    let mut epsfcn = options.finite_difference_step.unwrap_or(0.0);
    let mut factor = options.step_bound_factor;
    let mut nprint = 0;
    let mut info = 0;
    let mut nfev = 0;
    let mut njev = 0;
    let invocation = callback_runtime::with_expert_least_squares_f32(
        parameter_count,
        residual_count,
        analytic,
        residual_callback,
        jacobian_callback,
        |callback: ExpertLeastSquaresF32Callback| {
            let _error_scope = crate::runtime::permit_recoverable_native_statuses();
            // SAFETY: f32 equivalent of the fully checked DNLS1 call above,
            // using the reviewed SNLS1 ABI and exact M-by-N FJAC allocation.
            unsafe {
                slatec_sys::least_squares::snls1(
                    callback.ffi(),
                    &mut iopt,
                    &mut m,
                    &mut n,
                    parameters.as_mut_ptr(),
                    residuals.as_mut_ptr(),
                    workspace.jacobian.as_mut_ptr(),
                    &mut ldfjac,
                    &mut ftol,
                    &mut xtol,
                    &mut gtol,
                    &mut maxfev,
                    &mut epsfcn,
                    workspace.scaling.as_mut_ptr(),
                    &mut mode,
                    &mut factor,
                    &mut nprint,
                    &mut info,
                    &mut nfev,
                    &mut njev,
                    workspace.pivots.as_mut_ptr(),
                    workspace.qtf.as_mut_ptr(),
                    workspace.wa1.as_mut_ptr(),
                    workspace.wa2.as_mut_ptr(),
                    workspace.wa3.as_mut_ptr(),
                    workspace.wa4.as_mut_ptr(),
                );
            }
        },
    )
    .map_err(callback_runtime_error)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    let function_evaluations = native_count(nfev, "negative native NFEV")?;
    let jacobian_evaluations = native_count(njev, "negative native NJEV")?;
    if function_evaluations != invocation.residual_evaluations
        || jacobian_evaluations != invocation.jacobian_evaluations
    {
        return Err(LeastSquaresError::NativeContractViolation {
            detail: "native expert evaluation counts disagreed with the scoped callback counts",
        });
    }
    let residual_norm = norm_f32(&residuals);
    Ok(ExpertLeastSquaresResult {
        parameters,
        residuals,
        cost: 0.5 * residual_norm * residual_norm,
        residual_norm,
        status: status(info)?,
        function_evaluations,
        jacobian_evaluations,
    })
}

/// Minimizes a double-precision residual sum of squares with `DNLS1` and a
/// native forward-difference Jacobian.
///
/// This solves `minimize 1/2 Σ r_i(x)^2` for `M = residual_count` and
/// `N = initial.len()` with `M >= N`. It wraps original SLATEC `DNLS1`, unlike
/// [`super::least_squares`] (`DNLS1E`), and exposes independent `FTOL`, `XTOL`,
/// `GTOL`, `MAXFEV`, `EPSFCN`, scaling, and `FACTOR` controls through
/// [`ExpertLeastSquaresOptions`]. The residual closure receives immutable
/// parameters and exactly `M` mutable residuals. The wrapper selects `IOPT=1`
/// and `NPRINT=0`, allocates `FJAC[M*N]`, `IPVT[N]`, `QTF[N]`, `DIAG[N]`, three
/// `N` work vectors, and one `M` work vector internally.
///
/// Calls serialize the process-global GNU Fortran runtime. Callback panics and
/// non-finite residuals are contained; nested callback-based SLATEC calls are
/// rejected. A scoped `XGETF`/`XSETF` guard permits documented level-one
/// numerical statuses to return as [`LeastSquaresStatus`] and restores the
/// caller's legacy-error setting before returning.
///
/// # Errors
///
/// Returns [`LeastSquaresError`] for invalid dimensions/options, callback
/// failure, checked allocation/integer overflow, nesting, or an ABI-contract
/// violation. Numerical completion remains in the returned `status`.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{ExpertLeastSquaresOptions, least_squares_expert};
/// let fit = least_squares_expert(&[0.0, 0.0], 3, |p, r| {
///     r.copy_from_slice(&[p[0] - 1.0, p[0] + p[1] - 3.0, p[0] + 2.0 * p[1] - 5.0]);
/// }, ExpertLeastSquaresOptions::default())?;
/// assert!((fit.parameters[1] - 2.0).abs() < 1.0e-8);
/// # Ok(()) }
/// ```
pub fn least_squares_expert<F>(
    initial: &[f64],
    residual_count: usize,
    residuals: F,
    options: ExpertLeastSquaresOptions<'_, f64>,
) -> Result<ExpertLeastSquaresResult<f64>, LeastSquaresError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    run_f64(
        initial,
        residual_count,
        residuals,
        |_, _, _, _| {},
        options,
        false,
    )
}

/// Single-precision `SNLS1` forward-difference counterpart of
/// [`least_squares_expert`].
///
/// The objective, `M >= N` contract, expert controls, checked workspaces,
/// serialization, callback containment, and status semantics match the f64
/// API. Use [`ExpertLeastSquaresOptions::single_precision`] as a practical
/// initial control set and allow for f32 rounding in assertions.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{ExpertLeastSquaresOptions, least_squares_expert_f32};
/// let fit = least_squares_expert_f32(&[0.0, 0.0], 3, |p, r| {
///     r.copy_from_slice(&[p[0] - 1.0, p[0] + p[1] - 3.0, p[0] + 2.0 * p[1] - 5.0]);
/// }, ExpertLeastSquaresOptions::single_precision())?;
/// assert!((fit.parameters[0] - 1.0).abs() < 2.0e-3);
/// # Ok(()) }
/// ```
pub fn least_squares_expert_f32<F>(
    initial: &[f32],
    residual_count: usize,
    residuals: F,
    options: ExpertLeastSquaresOptions<'_, f32>,
) -> Result<ExpertLeastSquaresResult<f32>, LeastSquaresError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    run_f32(
        initial,
        residual_count,
        residuals,
        |_, _, _, _| {},
        options,
        false,
    )
}

/// Minimizes double-precision residuals with `DNLS1` and a dense analytic
/// Jacobian.
///
/// The residual callback receives `N` parameters and writes `M` residuals.
/// The Jacobian callback receives the same parameters, the valid native
/// residual vector, and a checked rectangular column-major
/// [`JacobianMut`] with `M` rows, `N` columns, and `LDFJAC=M`. It must write
/// each logical entry `∂r_row/∂x_column`; padding is not exposed. The wrapper
/// selects `IOPT=2`, ignores `EPSFCN`, fixes `NPRINT=0`, and otherwise maps
/// [`ExpertLeastSquaresOptions`] directly to the documented expert controls.
///
/// # Errors
///
/// In addition to [`least_squares_expert`] errors, a Jacobian panic, an
/// unwritten entry, or a NaN/infinite derivative produces a contained
/// [`LeastSquaresError::JacobianPanicked`] or
/// [`LeastSquaresError::JacobianReturnedNonFinite`].
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{ExpertLeastSquaresOptions, least_squares_with_jacobian};
/// let fit = least_squares_with_jacobian(
///     &[0.0, 0.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.0, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.0]),
///     |_, _, mut j| { j.set(0, 0, 1.0).unwrap(); j.set(0, 1, 0.0).unwrap();
///                      j.set(1, 0, 1.0).unwrap(); j.set(1, 1, 1.0).unwrap();
///                      j.set(2, 0, 1.0).unwrap(); j.set(2, 1, 2.0).unwrap(); },
///     ExpertLeastSquaresOptions::default(),
/// )?;
/// assert!(fit.cost < 1.0e-12);
/// # Ok(()) }
/// ```
pub fn least_squares_with_jacobian<F, J>(
    initial: &[f64],
    residual_count: usize,
    residuals: F,
    mut jacobian: J,
    options: ExpertLeastSquaresOptions<'_, f64>,
) -> Result<ExpertLeastSquaresResult<f64>, LeastSquaresError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], JacobianMut<'_, f64>),
{
    let parameter_count = initial.len();
    run_f64(
        initial,
        residual_count,
        residuals,
        move |x, f, matrix, leading_dimension| {
            if let Some(view) =
                JacobianMut::new(matrix, residual_count, parameter_count, leading_dimension)
            {
                jacobian(x, f, view);
            }
        },
        options,
        true,
    )
}

/// Single-precision `SNLS1` dense-analytic-Jacobian counterpart of
/// [`least_squares_with_jacobian`].
///
/// The checked `M × N` column-major Jacobian layout, expert controls,
/// allocation, serialization, callback containment, and error behavior match
/// the f64 API.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
/// use slatec::least_squares::{ExpertLeastSquaresOptions, least_squares_with_jacobian_f32};
/// let fit = least_squares_with_jacobian_f32(
///     &[0.0, 0.0], 3,
///     |p, r| r.copy_from_slice(&[p[0] - 1.0, p[0] + p[1] - 3.0, p[0] + 2.0*p[1] - 5.0]),
///     |_, _, mut j| { j.set(0, 0, 1.0).unwrap(); j.set(0, 1, 0.0).unwrap();
///                      j.set(1, 0, 1.0).unwrap(); j.set(1, 1, 1.0).unwrap();
///                      j.set(2, 0, 1.0).unwrap(); j.set(2, 1, 2.0).unwrap(); },
///     ExpertLeastSquaresOptions::single_precision(),
/// )?;
/// assert!(fit.cost < 1.0e-4);
/// # Ok(()) }
/// ```
pub fn least_squares_with_jacobian_f32<F, J>(
    initial: &[f32],
    residual_count: usize,
    residuals: F,
    mut jacobian: J,
    options: ExpertLeastSquaresOptions<'_, f32>,
) -> Result<ExpertLeastSquaresResult<f32>, LeastSquaresError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], JacobianMut<'_, f32>),
{
    let parameter_count = initial.len();
    run_f32(
        initial,
        residual_count,
        residuals,
        move |x, f, matrix, leading_dimension| {
            if let Some(view) =
                JacobianMut::new(matrix, residual_count, parameter_count, leading_dimension)
            {
                jacobian(x, f, view);
            }
        },
        options,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::{ExpertLeastSquaresOptions, LeastSquaresScaling, default_maximum, matrix_len};
    use crate::least_squares::LeastSquaresError;

    #[test]
    fn rectangular_workspace_and_defaults_are_checked() {
        assert_eq!(matrix_len(4, 2), Ok(8));
        assert_eq!(default_maximum(2, false), Ok(600));
        assert_eq!(default_maximum(2, true), Ok(300));
        assert!(matches!(
            matrix_len(usize::MAX, 2),
            Err(LeastSquaresError::WorkspaceOverflow)
        ));
        let scaling = [1.0, 2.0];
        let options = ExpertLeastSquaresOptions {
            scaling: LeastSquaresScaling::User(&scaling),
            ..ExpertLeastSquaresOptions::default()
        };
        assert_eq!(options.scaling, LeastSquaresScaling::User(&scaling));
    }
}
