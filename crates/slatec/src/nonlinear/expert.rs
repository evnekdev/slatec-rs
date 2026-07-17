//! Safe expert Powell-hybrid nonlinear-system drivers.

use alloc::{vec, vec::Vec};

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::callback_runtime::{
    self, CallbackRuntimeError, ExpertCallbackFailure, ExpertF32Callbacks, ExpertF64Callbacks,
};

use super::{JacobianMut, NonlinearError, NonlinearStatus};

/// Structure used by SLATEC's native finite-difference Jacobian estimator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JacobianStructure {
    /// Treat every Jacobian entry as potentially nonzero.
    Dense,
    /// Restrict finite differences to a band around the diagonal.
    Banded {
        /// Number of subdiagonals, corresponding to Fortran `ML`.
        lower_bandwidth: usize,
        /// Number of superdiagonals, corresponding to Fortran `MU`.
        upper_bandwidth: usize,
    },
}

/// Variable scaling policy corresponding to the expert driver's `MODE` and `DIAG`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VariableScaling<'a, T> {
    /// Let SLATEC derive positive scale factors from the initial Jacobian.
    Automatic,
    /// Copy these strictly positive multiplicative scale factors into `DIAG`.
    User(&'a [T]),
}

/// Controls accepted by the expert `SNSQ` and `DNSQ` drivers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpertNonlinearOptions<'a, T = f64> {
    /// Relative iterate tolerance passed as Fortran `XTOL`.
    pub tolerance: T,
    /// Optional `MAXFEV`; `None` uses `200*(N+1)` for finite differences and
    /// `100*(N+1)` for a user Jacobian.
    pub maximum_function_evaluations: Option<usize>,
    /// Relative function error estimate `EPSFCN`; `None` uses zero, causing
    /// SLATEC to use the selected profile's machine precision.
    pub finite_difference_step: Option<T>,
    /// Positive initial step-bound multiplier passed as `FACTOR`.
    pub step_bound_factor: T,
    /// Automatic or user-provided variable scaling.
    pub scaling: VariableScaling<'a, T>,
    /// Dense or banded native finite differences. User Jacobians must be dense.
    pub jacobian_structure: JacobianStructure,
}

impl Default for ExpertNonlinearOptions<'static, f64> {
    fn default() -> Self {
        Self {
            tolerance: 1.0e-10,
            maximum_function_evaluations: None,
            finite_difference_step: None,
            step_bound_factor: 100.0,
            scaling: VariableScaling::Automatic,
            jacobian_structure: JacobianStructure::Dense,
        }
    }
}

impl ExpertNonlinearOptions<'static, f32> {
    /// Returns practical single-precision expert-driver defaults.
    pub const fn single_precision() -> Self {
        Self {
            tolerance: 1.0e-5,
            maximum_function_evaluations: None,
            finite_difference_step: None,
            step_bound_factor: 100.0,
            scaling: VariableScaling::Automatic,
            jacobian_structure: JacobianStructure::Dense,
        }
    }
}

/// Result of an expert `SNSQ` or `DNSQ` solve.
#[derive(Clone, Debug, PartialEq)]
pub struct ExpertNonlinearResult<T = f64> {
    /// Final native iterate returned in Fortran `X`.
    pub solution: Vec<T>,
    /// Final native residual vector returned in `FVEC`.
    pub residual: Vec<T>,
    /// Euclidean norm of `residual`, recomputed by the safe wrapper.
    pub residual_norm: T,
    /// Exact native `NFEV` count, cross-checked against the Rust trampoline.
    pub function_evaluations: usize,
    /// Exact native `NJEV` count, cross-checked against the Rust trampoline.
    pub jacobian_evaluations: usize,
    /// Reviewed interpretation of native `INFO`.
    pub status: NonlinearStatus,
}

struct Workspace<T> {
    fjac: Vec<T>,
    diag: Vec<T>,
    r: Vec<T>,
    qtf: Vec<T>,
    wa1: Vec<T>,
    wa2: Vec<T>,
    wa3: Vec<T>,
    wa4: Vec<T>,
}

fn matrix_and_packed_lengths(dimension: usize) -> Result<(usize, usize), NonlinearError> {
    let matrix = dimension
        .checked_mul(dimension)
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    let packed = dimension
        .checked_add(1)
        .and_then(|next| dimension.checked_mul(next))
        .map(|value| value / 2)
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    Ok((matrix, packed))
}

fn native_integer(value: usize, argument: &'static str) -> Result<FortranInteger, NonlinearError> {
    to_fortran_integer(value).map_err(|_| NonlinearError::IntegerOverflow { argument })
}

fn native_count(value: FortranInteger, detail: &'static str) -> Result<usize, NonlinearError> {
    usize::try_from(value).map_err(|_| NonlinearError::NativeContractViolation { detail })
}

fn status(value: FortranInteger) -> Result<NonlinearStatus, NonlinearError> {
    match value {
        1 => Ok(NonlinearStatus::Converged),
        2 => Ok(NonlinearStatus::MaximumFunctionEvaluations),
        3 => Ok(NonlinearStatus::ToleranceTooSmall),
        4 => Ok(NonlinearStatus::SlowProgressJacobianEvaluations),
        5 => Ok(NonlinearStatus::SlowProgressIterations),
        status => Err(NonlinearError::NativeStatus { status }),
    }
}

fn callback_runtime_error(error: CallbackRuntimeError) -> NonlinearError {
    match error {
        CallbackRuntimeError::NestedCallback => NonlinearError::NestedNativeCallback,
    }
}

fn callback_failure(failure: ExpertCallbackFailure) -> NonlinearError {
    match failure {
        ExpertCallbackFailure::ResidualPanicked => NonlinearError::CallbackPanicked,
        ExpertCallbackFailure::JacobianPanicked => NonlinearError::JacobianPanicked,
        ExpertCallbackFailure::ResidualNonFinite { index } => {
            NonlinearError::CallbackReturnedNonFinite { index }
        }
        ExpertCallbackFailure::JacobianNonFinite { row, column } => {
            NonlinearError::JacobianReturnedNonFinite { row, column }
        }
        ExpertCallbackFailure::InvalidPointer => NonlinearError::NativeContractViolation {
            detail: "expert callback pointer was null or aliased",
        },
        ExpertCallbackFailure::DimensionMismatch => NonlinearError::NativeContractViolation {
            detail: "expert callback dimension did not match the registered system",
        },
        ExpertCallbackFailure::InvalidLeadingDimension => NonlinearError::NativeContractViolation {
            detail: "expert Jacobian leading dimension was invalid",
        },
        ExpertCallbackFailure::UnexpectedFlag => NonlinearError::NativeContractViolation {
            detail: "expert callback received an unexpected native IFLAG",
        },
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

fn validate_band(
    dimension: usize,
    structure: JacobianStructure,
    analytic: bool,
) -> Result<(usize, usize), NonlinearError> {
    if analytic && structure != JacobianStructure::Dense {
        return Err(NonlinearError::AnalyticJacobianRequiresDenseStructure);
    }
    match structure {
        JacobianStructure::Dense => Ok((dimension - 1, dimension - 1)),
        JacobianStructure::Banded {
            lower_bandwidth,
            upper_bandwidth,
        } => {
            if lower_bandwidth >= dimension {
                return Err(NonlinearError::InvalidBandwidth {
                    argument: "lower",
                    value: lower_bandwidth,
                    dimension,
                });
            }
            if upper_bandwidth >= dimension {
                return Err(NonlinearError::InvalidBandwidth {
                    argument: "upper",
                    value: upper_bandwidth,
                    dimension,
                });
            }
            lower_bandwidth
                .checked_add(upper_bandwidth)
                .and_then(|sum| sum.checked_add(1))
                .ok_or(NonlinearError::WorkspaceOverflow)?;
            Ok((lower_bandwidth, upper_bandwidth))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_f64<F, J>(
    initial: &[f64],
    residual_callback: F,
    jacobian_callback: J,
    options: ExpertNonlinearOptions<'_, f64>,
    analytic: bool,
) -> Result<ExpertNonlinearResult<f64>, NonlinearError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], &mut [f64], usize),
{
    if initial.is_empty() {
        return Err(NonlinearError::EmptySystem);
    }
    if let Some((index, _)) = initial.iter().enumerate().find(|(_, x)| !x.is_finite()) {
        return Err(NonlinearError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(NonlinearError::InvalidTolerance);
    }
    let epsfcn = options.finite_difference_step.unwrap_or(0.0);
    if !epsfcn.is_finite() || epsfcn < 0.0 {
        return Err(NonlinearError::InvalidFiniteDifferenceStep);
    }
    if !options.step_bound_factor.is_finite() || options.step_bound_factor <= 0.0 {
        return Err(NonlinearError::InvalidStepBoundFactor);
    }
    let dimension = initial.len();
    let (ml, mu) = validate_band(dimension, options.jacobian_structure, analytic)?;
    let default_multiplier = if analytic { 100 } else { 200 };
    let default_max = dimension
        .checked_add(1)
        .and_then(|value| value.checked_mul(default_multiplier))
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    let maxfev = options.maximum_function_evaluations.unwrap_or(default_max);
    if maxfev == 0 {
        return Err(NonlinearError::InvalidMaximumFunctionEvaluations);
    }
    let mut diag = match options.scaling {
        VariableScaling::Automatic => vec![0.0; dimension],
        VariableScaling::User(values) => {
            if values.len() != dimension {
                return Err(NonlinearError::InvalidScalingLength {
                    expected: dimension,
                    actual: values.len(),
                });
            }
            if let Some((index, _)) = values
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite() || **value <= 0.0)
            {
                return Err(NonlinearError::InvalidScalingValue { index });
            }
            values.to_vec()
        }
    };
    let mut mode = if matches!(options.scaling, VariableScaling::User(_)) {
        2
    } else {
        1
    };
    let (matrix_len, packed_len) = matrix_and_packed_lengths(dimension)?;
    let mut workspace = Workspace {
        fjac: vec![0.0; matrix_len],
        diag: core::mem::take(&mut diag),
        r: vec![0.0; packed_len],
        qtf: vec![0.0; dimension],
        wa1: vec![0.0; dimension],
        wa2: vec![0.0; dimension],
        wa3: vec![0.0; dimension],
        wa4: vec![0.0; dimension],
    };
    let mut x = initial.to_vec();
    let mut fvec = vec![0.0; dimension];
    let mut iopt = if analytic { 1 } else { 2 };
    let mut n = native_integer(dimension, "dimension")?;
    let mut ldfjac = n;
    let mut xtol = options.tolerance;
    let mut native_maxfev = native_integer(maxfev, "maximum function evaluations")?;
    let mut native_ml = native_integer(ml, "lower bandwidth")?;
    let mut native_mu = native_integer(mu, "upper bandwidth")?;
    let mut native_epsfcn = epsfcn;
    let mut factor = options.step_bound_factor;
    let mut nprint = 0;
    let mut info = 0;
    let mut nfev = 0;
    let mut njev = 0;
    let mut lr = native_integer(packed_len, "packed R length")?;

    let invocation = callback_runtime::with_expert_f64(
        dimension,
        residual_callback,
        jacobian_callback,
        |callbacks: ExpertF64Callbacks| {
            // SAFETY: all dimensions and exact workspace formulas were checked;
            // callback state is scoped and panic-contained; `NPRINT=0`; the GNU
            // MinGW profile and reviewed DNSQ symbol are selected by features.
            unsafe {
                slatec_sys::nonlinear::dnsq(
                    callbacks.residual(),
                    callbacks.jacobian(),
                    &mut iopt,
                    &mut n,
                    x.as_mut_ptr(),
                    fvec.as_mut_ptr(),
                    workspace.fjac.as_mut_ptr(),
                    &mut ldfjac,
                    &mut xtol,
                    &mut native_maxfev,
                    &mut native_ml,
                    &mut native_mu,
                    &mut native_epsfcn,
                    workspace.diag.as_mut_ptr(),
                    &mut mode,
                    &mut factor,
                    &mut nprint,
                    &mut info,
                    &mut nfev,
                    &mut njev,
                    workspace.r.as_mut_ptr(),
                    &mut lr,
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
        return Err(NonlinearError::NativeContractViolation {
            detail: "native evaluation counters disagreed with callback counts",
        });
    }
    Ok(ExpertNonlinearResult {
        solution: x,
        residual: fvec.clone(),
        residual_norm: norm_f64(&fvec),
        function_evaluations,
        jacobian_evaluations,
        status: status(info)?,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_f32<F, J>(
    initial: &[f32],
    residual_callback: F,
    jacobian_callback: J,
    options: ExpertNonlinearOptions<'_, f32>,
    analytic: bool,
) -> Result<ExpertNonlinearResult<f32>, NonlinearError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], &mut [f32], usize),
{
    if initial.is_empty() {
        return Err(NonlinearError::EmptySystem);
    }
    if let Some((index, _)) = initial.iter().enumerate().find(|(_, x)| !x.is_finite()) {
        return Err(NonlinearError::NonFiniteInitialValue { index });
    }
    if !options.tolerance.is_finite() || options.tolerance < 0.0 {
        return Err(NonlinearError::InvalidTolerance);
    }
    let epsfcn = options.finite_difference_step.unwrap_or(0.0);
    if !epsfcn.is_finite() || epsfcn < 0.0 {
        return Err(NonlinearError::InvalidFiniteDifferenceStep);
    }
    if !options.step_bound_factor.is_finite() || options.step_bound_factor <= 0.0 {
        return Err(NonlinearError::InvalidStepBoundFactor);
    }
    let dimension = initial.len();
    let (ml, mu) = validate_band(dimension, options.jacobian_structure, analytic)?;
    let default_multiplier = if analytic { 100 } else { 200 };
    let default_max = dimension
        .checked_add(1)
        .and_then(|value| value.checked_mul(default_multiplier))
        .ok_or(NonlinearError::WorkspaceOverflow)?;
    let maxfev = options.maximum_function_evaluations.unwrap_or(default_max);
    if maxfev == 0 {
        return Err(NonlinearError::InvalidMaximumFunctionEvaluations);
    }
    let mut diag = match options.scaling {
        VariableScaling::Automatic => vec![0.0; dimension],
        VariableScaling::User(values) => {
            if values.len() != dimension {
                return Err(NonlinearError::InvalidScalingLength {
                    expected: dimension,
                    actual: values.len(),
                });
            }
            if let Some((index, _)) = values
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite() || **value <= 0.0)
            {
                return Err(NonlinearError::InvalidScalingValue { index });
            }
            values.to_vec()
        }
    };
    let mut mode = if matches!(options.scaling, VariableScaling::User(_)) {
        2
    } else {
        1
    };
    let (matrix_len, packed_len) = matrix_and_packed_lengths(dimension)?;
    let mut workspace = Workspace {
        fjac: vec![0.0; matrix_len],
        diag: core::mem::take(&mut diag),
        r: vec![0.0; packed_len],
        qtf: vec![0.0; dimension],
        wa1: vec![0.0; dimension],
        wa2: vec![0.0; dimension],
        wa3: vec![0.0; dimension],
        wa4: vec![0.0; dimension],
    };
    let mut x = initial.to_vec();
    let mut fvec = vec![0.0; dimension];
    let mut iopt = if analytic { 1 } else { 2 };
    let mut n = native_integer(dimension, "dimension")?;
    let mut ldfjac = n;
    let mut xtol = options.tolerance;
    let mut native_maxfev = native_integer(maxfev, "maximum function evaluations")?;
    let mut native_ml = native_integer(ml, "lower bandwidth")?;
    let mut native_mu = native_integer(mu, "upper bandwidth")?;
    let mut native_epsfcn = epsfcn;
    let mut factor = options.step_bound_factor;
    let mut nprint = 0;
    let mut info = 0;
    let mut nfev = 0;
    let mut njev = 0;
    let mut lr = native_integer(packed_len, "packed R length")?;
    let invocation = callback_runtime::with_expert_f32(
        dimension,
        residual_callback,
        jacobian_callback,
        |callbacks: ExpertF32Callbacks| {
            // SAFETY: equivalent to the reviewed DNSQ call above for SNSQ f32.
            unsafe {
                slatec_sys::nonlinear::snsq(
                    callbacks.residual(),
                    callbacks.jacobian(),
                    &mut iopt,
                    &mut n,
                    x.as_mut_ptr(),
                    fvec.as_mut_ptr(),
                    workspace.fjac.as_mut_ptr(),
                    &mut ldfjac,
                    &mut xtol,
                    &mut native_maxfev,
                    &mut native_ml,
                    &mut native_mu,
                    &mut native_epsfcn,
                    workspace.diag.as_mut_ptr(),
                    &mut mode,
                    &mut factor,
                    &mut nprint,
                    &mut info,
                    &mut nfev,
                    &mut njev,
                    workspace.r.as_mut_ptr(),
                    &mut lr,
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
        return Err(NonlinearError::NativeContractViolation {
            detail: "native evaluation counters disagreed with callback counts",
        });
    }
    Ok(ExpertNonlinearResult {
        solution: x,
        residual: fvec.clone(),
        residual_norm: norm_f32(&fvec),
        function_evaluations,
        jacobian_evaluations,
        status: status(info)?,
    })
}

/// Solves a square system using the double-precision expert `DNSQ` driver and
/// a native finite-difference Jacobian.
///
/// The callback computes `f(x)` into its output slice. `initial` maps to
/// Fortran `X`; option fields map to `XTOL`, `MAXFEV`, `ML`, `MU`, `EPSFCN`,
/// `DIAG`, `MODE`, and `FACTOR`. `NPRINT` is fixed to zero. Dense and banded
/// finite differences are selected by [`JacobianStructure`]. Hidden `FJAC`,
/// packed `R`, `QTF`, and four work vectors are allocated with checked exact
/// sizes. Calls require `std`, `alloc`, `nonlinear-expert`, and the validated
/// GNU MinGW profile; they serialize and reject nested native callbacks.
///
/// # Errors
///
/// Returns [`NonlinearError`] for invalid dimensions/options, callback panic,
/// non-finite residuals, nested calls, counter mismatch, or ABI violations.
/// Numerical terminations remain in [`ExpertNonlinearResult::status`].
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_expert};
/// let result = solve_system_expert(&[1.0], |x, f| f[0] = x[0] * x[0] - 2.0,
///     ExpertNonlinearOptions::default())?;
/// assert!((result.solution[0] - 2.0_f64.sqrt()).abs() < 1.0e-8);
/// # Ok(()) }
/// ```
pub fn solve_system_expert<F>(
    initial: &[f64],
    function: F,
    options: ExpertNonlinearOptions<'_, f64>,
) -> Result<ExpertNonlinearResult<f64>, NonlinearError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    run_f64(initial, function, |_, _, _, _| {}, options, false)
}

/// Single-precision `SNSQ` counterpart of [`solve_system_expert`].
///
/// Original SLATEC routine: `SNSQ`. Arguments and errors match the f64 API;
/// use [`ExpertNonlinearOptions::single_precision`] for practical defaults.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_expert_f32};
/// let result = solve_system_expert_f32(&[1.0], |x, f| f[0] = x[0] * x[0] - 2.0,
///     ExpertNonlinearOptions::single_precision())?;
/// assert!((result.solution[0] - 2.0_f32.sqrt()).abs() < 2.0e-4);
/// # Ok(()) }
/// ```
pub fn solve_system_expert_f32<F>(
    initial: &[f32],
    function: F,
    options: ExpertNonlinearOptions<'_, f32>,
) -> Result<ExpertNonlinearResult<f32>, NonlinearError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    run_f32(initial, function, |_, _, _, _| {}, options, false)
}

/// Solves a square system with `DNSQ` and a user-supplied dense Jacobian.
///
/// The Jacobian callback receives the current `X`, the valid current `FVEC`,
/// and a checked `N × N` column-major [`JacobianMut`] corresponding to
/// `FJAC(LDFJAC,N)`. Every logical entry must be written and finite. Native
/// finite-difference band controls are rejected for this dense callback mode.
/// Allocation, status, serialization, and error behavior match
/// [`solve_system_expert`].
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_with_jacobian};
/// let result = solve_system_with_jacobian(
///     &[1.0],
///     |x, f| f[0] = x[0] * x[0] - 2.0,
///     |x, _, mut j| j.set(0, 0, 2.0 * x[0]).unwrap(),
///     ExpertNonlinearOptions::default(),
/// )?;
/// assert!(result.residual_norm < 1.0e-10);
/// # Ok(()) }
/// ```
pub fn solve_system_with_jacobian<F, J>(
    initial: &[f64],
    function: F,
    mut jacobian: J,
    options: ExpertNonlinearOptions<'_, f64>,
) -> Result<ExpertNonlinearResult<f64>, NonlinearError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], JacobianMut<'_, f64>),
{
    let dimension = initial.len();
    run_f64(
        initial,
        function,
        move |x, f, storage, leading_dimension| {
            if let Some(view) = JacobianMut::new(storage, dimension, dimension, leading_dimension) {
                jacobian(x, f, view);
            }
        },
        options,
        true,
    )
}

/// Single-precision `SNSQ` counterpart of [`solve_system_with_jacobian`].
///
/// Original SLATEC routine: `SNSQ`. The callback fills a checked f32
/// column-major Jacobian; every logical entry must be finite.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
/// use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_with_jacobian_f32};
/// let result = solve_system_with_jacobian_f32(
///     &[1.0], |x, f| f[0] = x[0] * x[0] - 2.0,
///     |x, _, mut j| j.set(0, 0, 2.0 * x[0]).unwrap(),
///     ExpertNonlinearOptions::single_precision(),
/// )?;
/// assert!(result.residual_norm < 1.0e-4);
/// # Ok(()) }
/// ```
pub fn solve_system_with_jacobian_f32<F, J>(
    initial: &[f32],
    function: F,
    mut jacobian: J,
    options: ExpertNonlinearOptions<'_, f32>,
) -> Result<ExpertNonlinearResult<f32>, NonlinearError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], JacobianMut<'_, f32>),
{
    let dimension = initial.len();
    run_f32(
        initial,
        function,
        move |x, f, storage, leading_dimension| {
            if let Some(view) = JacobianMut::new(storage, dimension, dimension, leading_dimension) {
                jacobian(x, f, view);
            }
        },
        options,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ExpertNonlinearOptions, JacobianStructure, VariableScaling, matrix_and_packed_lengths,
        validate_band,
    };
    use crate::nonlinear::NonlinearError;

    #[test]
    fn exact_workspace_lengths_and_band_validation_are_checked() {
        assert_eq!(matrix_and_packed_lengths(3), Ok((9, 6)));
        assert_eq!(
            validate_band(3, JacobianStructure::Dense, false),
            Ok((2, 2))
        );
        assert_eq!(
            validate_band(
                3,
                JacobianStructure::Banded {
                    lower_bandwidth: 1,
                    upper_bandwidth: 0
                },
                false,
            ),
            Ok((1, 0))
        );
        assert!(matches!(
            validate_band(
                2,
                JacobianStructure::Banded {
                    lower_bandwidth: 2,
                    upper_bandwidth: 0
                },
                false,
            ),
            Err(NonlinearError::InvalidBandwidth { .. })
        ));
        let scaling = [1.0, 2.0];
        let options = ExpertNonlinearOptions {
            scaling: VariableScaling::User(&scaling),
            ..ExpertNonlinearOptions::default()
        };
        assert_eq!(options.scaling, VariableScaling::User(&scaling));
    }
}
