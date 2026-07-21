//! Safe scalar-equation nonlinear-system drivers over `SOS` and `DSOS`.

use alloc::vec;
use alloc::vec::Vec;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::callback_runtime::{
    self, CallbackRuntimeError, IndexedEquationCallbackFailure, IndexedEquationF32Callback,
    IndexedEquationF64Callback,
};

use super::NonlinearError;

/// Checked controls for the scalar-equation `SOS` and `DSOS` drivers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemOptions<T> {
    /// Relative iterate-change tolerance (`RTOLX`).
    pub relative_x_tolerance: T,
    /// Absolute iterate-change tolerance (`ATOLX`).
    pub absolute_x_tolerance: T,
    /// Per-equation residual tolerance (`TOLF`).
    pub residual_tolerance: T,
    /// Optional native iteration limit. `None` selects the source default.
    pub maximum_iterations: Option<usize>,
    /// Requests the source's optional iteration diagnostics. Library callers
    /// normally leave this disabled because it writes to the legacy error unit.
    pub iteration_output: bool,
}

impl Default for SystemOptions<f64> {
    fn default() -> Self {
        Self {
            relative_x_tolerance: 1.0e-8,
            absolute_x_tolerance: 1.0e-12,
            residual_tolerance: 1.0e-8,
            maximum_iterations: None,
            iteration_output: false,
        }
    }
}

impl Default for SystemOptions<f32> {
    fn default() -> Self {
        Self {
            relative_x_tolerance: 1.0e-5,
            absolute_x_tolerance: 1.0e-6,
            residual_tolerance: 1.0e-5,
            maximum_iterations: None,
            iteration_output: false,
        }
    }
}

/// Exact source-documented `IFLAG` completion state for `SOS` and `DSOS`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemTermination {
    /// The iterate-increment criterion passed.
    IterateConverged,
    /// The residual criterion passed.
    ResidualConverged,
    /// Both convergence criteria passed.
    BothConverged,
    /// Precision was inadequate or convergence was too slow.
    PrecisionOrSlowConvergence,
    /// The configured iteration limit was reached.
    IterationLimit,
    /// The iteration limit was reached while the sequence was not converging.
    IterationLimitNotConverging,
    /// The iteration appeared to diverge.
    Diverging,
    /// The finite-difference Jacobian solve was singular or nearly singular.
    SingularJacobian,
    /// Native input or workspace controls were rejected.
    NativeInvalidInput,
}

/// Final state from one `SOS` or `DSOS` solve.
#[derive(Clone, Debug, PartialEq)]
pub struct SystemReport<T> {
    /// Latest native iterate, including non-converged terminations.
    pub solution: Vec<T>,
    /// Exact mapped source completion state.
    pub termination: SystemTermination,
    /// Number of native nonlinear iterations when the driver reported it.
    pub iterations: usize,
    /// Native residual norm (`RW(1)`).
    pub residual_norm: T,
    /// Whether the iterate-change criterion passed.
    pub iterate_criterion_passed: bool,
    /// Whether the residual criterion passed.
    pub residual_criterion_passed: bool,
    /// Counted Rust equation callback invocations.
    pub equation_evaluations: usize,
}

fn integer(value: usize, argument: &'static str) -> Result<FortranInteger, NonlinearError> {
    to_fortran_integer(value).map_err(|_| NonlinearError::IntegerOverflow { argument })
}

fn real_workspace_len(n: usize) -> Result<usize, NonlinearError> {
    n.checked_mul(n.checked_add(1).ok_or(NonlinearError::WorkspaceOverflow)?)
        .and_then(|value| value.checked_div(2))
        .and_then(|value| value.checked_add(6usize.checked_mul(n)?))
        .and_then(|value| value.checked_add(1))
        .ok_or(NonlinearError::WorkspaceOverflow)
}

fn integer_workspace_len(n: usize) -> Result<usize, NonlinearError> {
    n.checked_add(3).ok_or(NonlinearError::WorkspaceOverflow)
}

fn status(value: FortranInteger) -> Result<SystemTermination, NonlinearError> {
    match value {
        1 => Ok(SystemTermination::IterateConverged),
        2 => Ok(SystemTermination::ResidualConverged),
        3 => Ok(SystemTermination::BothConverged),
        4 => Ok(SystemTermination::PrecisionOrSlowConvergence),
        5 => Ok(SystemTermination::IterationLimit),
        6 => Ok(SystemTermination::IterationLimitNotConverging),
        7 => Ok(SystemTermination::Diverging),
        8 => Ok(SystemTermination::SingularJacobian),
        9 => Ok(SystemTermination::NativeInvalidInput),
        status => Err(NonlinearError::NativeStatus { status }),
    }
}

fn callback_failure(failure: IndexedEquationCallbackFailure) -> NonlinearError {
    match failure {
        IndexedEquationCallbackFailure::Panicked => NonlinearError::CallbackPanicked,
        IndexedEquationCallbackFailure::NonFinite { index } => {
            NonlinearError::CallbackReturnedNonFinite { index }
        }
        IndexedEquationCallbackFailure::InvalidPointer => NonlinearError::NativeContractViolation {
            detail: "native SOS callback supplied a null iterate or equation pointer",
        },
        IndexedEquationCallbackFailure::InvalidEquationIndex => {
            NonlinearError::NativeContractViolation {
                detail: "native SOS callback supplied an invalid one-based equation index",
            }
        }
    }
}

fn nested(error: CallbackRuntimeError) -> NonlinearError {
    match error {
        CallbackRuntimeError::NestedCallback => NonlinearError::NestedNativeCallback,
    }
}

fn report<T: Copy>(
    solution: Vec<T>,
    native_status: FortranInteger,
    real_workspace: &[T],
    integer_workspace: &[FortranInteger],
    evaluations: usize,
) -> Result<SystemReport<T>, NonlinearError> {
    let termination = status(native_status)?;
    let iterations = integer_workspace
        .get(2)
        .copied()
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0);
    Ok(SystemReport {
        solution,
        iterate_criterion_passed: matches!(
            termination,
            SystemTermination::IterateConverged | SystemTermination::BothConverged
        ),
        residual_criterion_passed: matches!(
            termination,
            SystemTermination::ResidualConverged | SystemTermination::BothConverged
        ),
        termination,
        iterations,
        residual_norm: real_workspace[0],
        equation_evaluations: evaluations,
    })
}

fn validate_f64(initial: &[f64], options: SystemOptions<f64>) -> Result<(), NonlinearError> {
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
    if !options.relative_x_tolerance.is_finite()
        || !options.absolute_x_tolerance.is_finite()
        || !options.residual_tolerance.is_finite()
        || options.relative_x_tolerance < 0.0
        || options.absolute_x_tolerance < 0.0
        || options.residual_tolerance < 0.0
    {
        return Err(NonlinearError::InvalidTolerance);
    }
    if options.maximum_iterations.is_some_and(|limit| limit == 0)
        || (options.iteration_output && options.maximum_iterations.is_none())
    {
        return Err(NonlinearError::InvalidMaximumIterations);
    }
    Ok(())
}

fn validate_f32(initial: &[f32], options: SystemOptions<f32>) -> Result<(), NonlinearError> {
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
    if !options.relative_x_tolerance.is_finite()
        || !options.absolute_x_tolerance.is_finite()
        || !options.residual_tolerance.is_finite()
        || options.relative_x_tolerance < 0.0
        || options.absolute_x_tolerance < 0.0
        || options.residual_tolerance < 0.0
    {
        return Err(NonlinearError::InvalidTolerance);
    }
    if options.maximum_iterations.is_some_and(|limit| limit == 0)
        || (options.iteration_output && options.maximum_iterations.is_none())
    {
        return Err(NonlinearError::InvalidMaximumIterations);
    }
    Ok(())
}

/// Solves a real double-precision square system using scalar equation callback
/// `FNC(X, K)` over original SLATEC `DSOS`.
///
/// The callback sees a readable current iterate and a zero-based equation
/// index. The native driver perturbs the iterate while forming finite
/// differences, so it must be treated as read-only. Workspaces are allocated
/// internally and the returned report retains the latest iterate for every
/// documented native termination.
///
/// # Example
///
/// ```no_run
/// # use slatec::nonlinear::{NonlinearError, SystemOptions, solve_scalar_equations};
/// let report = solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), |x, equation| {
///     if equation == 0 { x[0] * x[0] + x[1] * x[1] - 1.0 } else { x[0] - x[1] }
/// })?;
/// assert!((report.solution[0] - 2.0_f64.sqrt() / 2.0).abs() < 1.0e-6);
/// # Ok::<(), NonlinearError>(())
/// ```
pub fn solve_scalar_equations<F>(
    initial: &[f64],
    options: SystemOptions<f64>,
    equation: F,
) -> Result<SystemReport<f64>, NonlinearError>
where
    F: FnMut(&[f64], usize) -> f64,
{
    validate_f64(initial, options)?;
    let mut solution = initial.to_vec();
    let n = integer(solution.len(), "system dimension")?;
    let real_length = real_workspace_len(solution.len())?;
    let integer_length = integer_workspace_len(solution.len())?;
    let mut real_workspace = vec![0.0; real_length];
    let mut integer_workspace = vec![0; integer_length];
    let mut lrw = integer(real_length, "real workspace length")?;
    let mut liw = integer(integer_length, "integer workspace length")?;
    let mut n = n;
    let mut rtolx = options.relative_x_tolerance;
    let mut atolx = options.absolute_x_tolerance;
    let mut tolf = options.residual_tolerance;
    let mut iflag = if options.maximum_iterations.is_some() || options.iteration_output {
        integer_workspace[0] = if options.iteration_output { -1 } else { 0 };
        integer_workspace[1] = integer(
            options.maximum_iterations.unwrap_or(0),
            "maximum iterations",
        )?;
        -1
    } else {
        0
    };
    let invocation = callback_runtime::with_indexed_equation_f64(
        solution.len(),
        equation,
        |callback: IndexedEquationF64Callback| {
            // SAFETY: all lengths use the reviewed source formula, the state
            // is owned and non-aliased, and the callback is scoped above.
            unsafe {
                slatec_sys::nonlinear::systems::dsos(
                    callback.ffi(),
                    &mut n,
                    solution.as_mut_ptr(),
                    &mut rtolx,
                    &mut atolx,
                    &mut tolf,
                    &mut iflag,
                    real_workspace.as_mut_ptr(),
                    &mut lrw,
                    integer_workspace.as_mut_ptr(),
                    &mut liw,
                );
            }
        },
    )
    .map_err(nested)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    report(
        solution,
        iflag,
        &real_workspace,
        &integer_workspace,
        invocation.evaluations,
    )
}

/// Single-precision counterpart of [`solve_scalar_equations`], using `SOS`.
pub fn solve_scalar_equations_f32<F>(
    initial: &[f32],
    options: SystemOptions<f32>,
    equation: F,
) -> Result<SystemReport<f32>, NonlinearError>
where
    F: FnMut(&[f32], usize) -> f32,
{
    validate_f32(initial, options)?;
    let mut solution = initial.to_vec();
    let n = integer(solution.len(), "system dimension")?;
    let real_length = real_workspace_len(solution.len())?;
    let integer_length = integer_workspace_len(solution.len())?;
    let mut real_workspace = vec![0.0; real_length];
    let mut integer_workspace = vec![0; integer_length];
    let mut lrw = integer(real_length, "real workspace length")?;
    let mut liw = integer(integer_length, "integer workspace length")?;
    let mut n = n;
    let mut rtolx = options.relative_x_tolerance;
    let mut atolx = options.absolute_x_tolerance;
    let mut tolf = options.residual_tolerance;
    let mut iflag = if options.maximum_iterations.is_some() || options.iteration_output {
        integer_workspace[0] = if options.iteration_output { -1 } else { 0 };
        integer_workspace[1] = integer(
            options.maximum_iterations.unwrap_or(0),
            "maximum iterations",
        )?;
        -1
    } else {
        0
    };
    let invocation = callback_runtime::with_indexed_equation_f32(
        solution.len(),
        equation,
        |callback: IndexedEquationF32Callback| {
            // SAFETY: equivalent to the reviewed DSOS call above.
            unsafe {
                slatec_sys::nonlinear::systems::sos(
                    callback.ffi(),
                    &mut n,
                    solution.as_mut_ptr(),
                    &mut rtolx,
                    &mut atolx,
                    &mut tolf,
                    &mut iflag,
                    real_workspace.as_mut_ptr(),
                    &mut lrw,
                    integer_workspace.as_mut_ptr(),
                    &mut liw,
                );
            }
        },
    )
    .map_err(nested)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    report(
        solution,
        iflag,
        &real_workspace,
        &integer_workspace,
        invocation.evaluations,
    )
}

#[cfg(test)]
mod tests {
    use super::{SystemOptions, integer_workspace_len, real_workspace_len, validate_f64};
    use crate::nonlinear::NonlinearError;

    #[test]
    fn source_workspace_formula_and_optional_control_validation_are_checked() {
        assert_eq!(real_workspace_len(2), Ok(16));
        assert_eq!(integer_workspace_len(2), Ok(5));
        assert_eq!(
            validate_f64(&[], SystemOptions::default()),
            Err(NonlinearError::EmptySystem)
        );
        assert_eq!(
            validate_f64(
                &[1.0],
                SystemOptions {
                    iteration_output: true,
                    ..SystemOptions::default()
                },
            ),
            Err(NonlinearError::InvalidMaximumIterations)
        );
    }
}
