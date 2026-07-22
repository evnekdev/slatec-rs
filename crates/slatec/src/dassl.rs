//! Owned residual-only sessions for the reviewed SLATEC `SDASSL` and `DDASSL`
//! DAE drivers.
//!
//! The public scope is real first-order index-1 initial-value problems
//! `G(t, y, y') = 0`. Callers supply an initially sufficiently consistent
//! pair `(y, y')`; this API does not calculate consistent initial conditions.
//! DASSL internally differences either a dense or checked-banded iteration
//! matrix, so residual functions can be invoked many times per step. Native
//! entry, callback dispatch, and XERROR control are serialized process-wide.

use alloc::vec::Vec;
use core::cell::Cell;
use core::ffi::c_void;
use core::marker::PhantomData;
use std::cell::Cell as ThreadCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread_local;

use slatec_sys::FortranInteger;

use crate::runtime::{lock_native, permit_recoverable_native_statuses};

thread_local! {
    static ACTIVE_DASSL_CONTEXT: ThreadCell<*mut c_void> = const { ThreadCell::new(core::ptr::null_mut()) };
}

mod sealed {
    pub trait Scalar {}

    impl Scalar for f32 {}
    impl Scalar for f64 {}
}

/// Scalar types supported by the reviewed real DASSL session implementation.
///
/// [`f32`] dispatches to `SDASSL`; [`f64`] dispatches to `DDASSL`. The trait is
/// sealed because the raw ABI and workspace contract are verified only for
/// those two native precisions.
pub trait DaeScalar:
    sealed::Scalar + Copy + PartialOrd + core::fmt::Debug + core::ops::Sub<Output = Self> + 'static
{
    /// Returns whether this scalar is finite.
    fn is_finite(self) -> bool;
    /// Returns additive zero.
    fn zero() -> Self;
    /// Returns the absolute value.
    fn abs(self) -> Self;
    /// Adds two values.
    fn add(self, other: Self) -> Self;
    /// Multiplies two values.
    fn multiply(self, other: Self) -> Self;
    /// Divides this value by another value.
    fn divide(self, other: Self) -> Self;
    /// Returns the square root.
    fn sqrt(self) -> Self;
    /// Converts a nonnegative size used by a checked diagnostic calculation.
    fn from_usize(value: usize) -> Self;
    /// Calls the reviewed matching native DASSL precision.
    ///
    /// This is an implementation detail; safe callers use
    /// [`DaeSession::advance_to`].
    #[doc(hidden)]
    fn call_native<F: DaeResidual<Self>>(
        session: &mut DaeSession<Self, F>,
        target: Self,
    ) -> Result<DaeStatus, DaeError<F::Error>>;
}

impl DaeScalar for f32 {
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn zero() -> Self {
        0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn multiply(self, other: Self) -> Self {
        self * other
    }

    fn divide(self, other: Self) -> Self {
        self / other
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn from_usize(value: usize) -> Self {
        value as Self
    }

    fn call_native<F: DaeResidual<Self>>(
        session: &mut DaeSession<Self, F>,
        target: Self,
    ) -> Result<DaeStatus, DaeError<F::Error>> {
        call_f32(session, target)
    }
}

impl DaeScalar for f64 {
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn zero() -> Self {
        0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn multiply(self, other: Self) -> Self {
        self * other
    }

    fn divide(self, other: Self) -> Self {
        self / other
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn from_usize(value: usize) -> Self {
        value as Self
    }

    fn call_native<F: DaeResidual<Self>>(
        session: &mut DaeSession<Self, F>,
        target: Self,
    ) -> Result<DaeStatus, DaeError<F::Error>> {
        call_f64(session, target)
    }
}

/// A documented action requested by a residual callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualAction {
    /// The residual was written successfully and native integration may continue.
    Continue,
    /// The current native iterate is invalid; DASSL may retry without this
    /// condition and reports `IDID=-10` if recovery repeatedly fails.
    RecoverableFailure,
    /// Stop the native call immediately through documented `IRES=-2`.
    FatalFailure,
}

/// A residual callback for an implicit DAE `G(t, y, y') = 0`.
///
/// `state` and `state_derivative` are immutable callback inputs. `residual`
/// is an exactly dimensioned output buffer that must receive `G`. Returning a
/// [`ResidualAction`] maps to DASSL's reviewed `IRES` protocol; a Rust error or
/// panic is contained and converted to `IRES=-2` without unwinding through
/// Fortran.
pub trait DaeResidual<T> {
    /// Application-defined callback error.
    type Error;

    /// Evaluates `G(time, state, state_derivative)` into `residual`.
    fn evaluate(
        &mut self,
        time: T,
        state: &[T],
        state_derivative: &[T],
        residual: &mut [T],
    ) -> Result<ResidualAction, Self::Error>;
}

impl<T, F, E> DaeResidual<T> for F
where
    F: FnMut(T, &[T], &[T], &mut [T]) -> Result<ResidualAction, E>,
{
    type Error = E;

    fn evaluate(
        &mut self,
        time: T,
        state: &[T],
        state_derivative: &[T],
        residual: &mut [T],
    ) -> Result<ResidualAction, Self::Error> {
        self(time, state, state_derivative, residual)
    }
}

/// DASSL's supported common or component-wise error-control modes.
#[derive(Clone, Debug, PartialEq)]
pub enum DaeTolerance<T> {
    /// Common relative and absolute tolerances for every component.
    Scalar {
        /// Relative tolerance.
        relative: T,
        /// Absolute tolerance.
        absolute: T,
    },
    /// Component-wise relative and absolute tolerances.
    Vector {
        /// Relative tolerances, one per state component.
        relative: Vec<T>,
        /// Absolute tolerances, one per state component.
        absolute: Vec<T>,
    },
}

/// DASSL's permitted maximum BDF order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DaeOrder {
    /// First-order BDF.
    One,
    /// Second-order BDF.
    Two,
    /// Third-order BDF.
    Three,
    /// Fourth-order BDF.
    Four,
    /// Fifth-order BDF, DASSL's default maximum.
    Five,
}

impl DaeOrder {
    fn value(self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }
}

/// Internally generated DASSL iteration-matrix storage.
///
/// Both variants retain the existing residual-only callback model. DASSL's
/// analytic `JAC` ABI has no source-defined abort flag, so user-supplied
/// Jacobian callbacks remain outside the safe API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DaeJacobianMode {
    /// Dense finite-difference iteration matrix (`INFO(5)=0`, `INFO(6)=0`).
    FiniteDifferenceDense,
    /// Banded finite-difference iteration matrix (`INFO(5)=0`, `INFO(6)=1`).
    FiniteDifferenceBanded {
        /// Largest permitted row-minus-column index.
        lower_bandwidth: usize,
        /// Largest permitted column-minus-row index.
        upper_bandwidth: usize,
    },
}

/// Reviewed controls for the residual-only DASSL mode.
///
/// This API fixes interval output, internal finite-difference Jacobians,
/// caller-supplied consistent initial values. It does not expose raw `INFO`
/// words, user Jacobians, or nonnegativity projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DaeOptions<T> {
    /// Optional positive maximum absolute step size.
    pub maximum_step: Option<T>,
    /// Optional positive initial step size. Its sign must match the first
    /// requested integration direction.
    pub initial_step: Option<T>,
    /// Optional maximum BDF order; `None` keeps DASSL's order-five default.
    pub maximum_order: Option<DaeOrder>,
    /// Internal finite-difference iteration-matrix storage.
    pub jacobian_mode: DaeJacobianMode,
}

impl<T> Default for DaeOptions<T> {
    fn default() -> Self {
        Self {
            maximum_step: None,
            initial_step: None,
            maximum_order: None,
            jacobian_mode: DaeJacobianMode::FiniteDifferenceDense,
        }
    }
}

/// Input rejection before a native DASSL call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DaeInputError {
    /// `y` was empty.
    EmptyState,
    /// `y` and `y_prime` had different dimensions.
    DerivativeLength {
        /// State dimension.
        expected: usize,
        /// Supplied derivative dimension.
        actual: usize,
    },
    /// A scalar input was NaN or infinite.
    NonFiniteInput,
    /// A tolerance was negative, non-finite, or supplied a zero/zero error
    /// weight for one component.
    InvalidTolerance,
    /// A vector tolerance did not match the state dimension.
    ToleranceLength {
        /// State dimension.
        expected: usize,
        /// Supplied tolerance dimension.
        actual: usize,
    },
    /// A target did not establish or preserve integration direction.
    InvalidTarget,
    /// A step-size control was non-positive or non-finite.
    InvalidStepControl,
    /// A dimension, workspace, or native integer conversion overflowed.
    DimensionOverflow,
    /// A requested band width was outside DASSL's documented `0..NEQ` range.
    InvalidBandwidth,
    /// A fallible session-owned allocation could not be satisfied.
    AllocationFailed,
    /// Changing the state dimension requires constructing a new session.
    DimensionChangeRequiresNewSession,
}

/// DASSL status whose returned state remains meaningful.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DaeStatus {
    /// DASSL interpolated or stepped to the requested output time (`IDID=3`).
    ReachedTarget,
    /// DASSL reported a stop-time return (`IDID=2`).
    ///
    /// The initial requested-output API does not configure `TSTOP`; this
    /// defensive variant preserves the reviewed native status distinction.
    ReachedStopTime,
    /// About 500 internal steps were used before the target (`IDID=-1`). A
    /// continuation call is permitted.
    ExcessiveWork,
    /// DASSL increased the owned tolerances (`IDID=-2`). A continuation call
    /// is permitted with those or caller-replaced tolerances.
    ExcessiveAccuracyRequested,
    /// A zero solution component made a pure relative error weight invalid
    /// (`IDID=-3`). Replace tolerances before continuing.
    ErrorWeightFailure,
}

/// Native DASSL outcome that terminally poisons the current continuation history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DaeNativeFailure {
    /// Repeated local error-test failure (`IDID=-6`).
    RepeatedErrorTestFailure,
    /// Repeated corrector convergence failure (`IDID=-7`).
    RepeatedConvergenceFailure,
    /// Singular iteration matrix (`IDID=-8`).
    SingularIterationMatrix,
    /// Combined repeated corrector and error-test failures (`IDID=-9`).
    CombinedConvergenceAndErrorFailure,
    /// Repeated callback `IRES=-1` requests prevented convergence (`IDID=-10`).
    RecoverableResidualFailure,
    /// DASSL could not calculate an initial derivative (`IDID=-12`). This mode
    /// is not requested by the safe API but remains mapped defensively.
    InitialDerivativeFailure,
}

/// Rust-side error from creating or advancing a DASSL session.
#[derive(Debug)]
pub enum DaeError<E> {
    /// Input validation failed before native entry.
    InvalidInput(DaeInputError),
    /// The residual callback returned its application error and native code was
    /// stopped through `IRES=-2`.
    Callback(E),
    /// A residual callback panic was caught before it could cross FFI.
    CallbackPanicked,
    /// The residual callback requested [`ResidualAction::FatalFailure`].
    ResidualFatalFailure,
    /// The residual callback wrote a non-finite component.
    NonFiniteResidual {
        /// First non-finite zero-based residual index.
        index: usize,
    },
    /// Native code made a callback request outside the reviewed contract.
    CallbackContractViolation,
    /// A nested DASSL callback registration was attempted on one thread.
    ReentrantCall,
    /// Native DASSL reported a terminal documented failure.
    NativeFailure(DaeNativeFailure),
    /// Native DASSL returned an unreviewed `IDID` value or unexpectedly called
    /// the disabled analytic-Jacobian callback.
    NativeContractViolation {
        /// Native completion code.
        idid: FortranInteger,
    },
    /// The session's continuation history is terminally unusable.
    SessionFailed,
    /// Recovery from `ErrorWeightFailure` needs a validated tolerance update.
    ToleranceUpdateRequired,
}

impl<E: core::fmt::Display> core::fmt::Display for DaeError<E> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput(error) => write!(formatter, "invalid DASSL input: {error:?}"),
            Self::Callback(error) => write!(formatter, "DASSL residual callback failed: {error}"),
            Self::CallbackPanicked => formatter.write_str("DASSL residual callback panicked"),
            Self::ResidualFatalFailure => {
                formatter.write_str("DASSL residual requested fatal termination")
            }
            Self::NonFiniteResidual { index } => {
                write!(
                    formatter,
                    "DASSL residual produced a non-finite value at index {index}"
                )
            }
            Self::CallbackContractViolation => {
                formatter.write_str("DASSL made an invalid residual or Jacobian callback request")
            }
            Self::ReentrantCall => {
                formatter.write_str("nested DASSL callback-based calls are rejected")
            }
            Self::NativeFailure(failure) => write!(formatter, "DASSL terminated: {failure:?}"),
            Self::NativeContractViolation { idid } => {
                write!(formatter, "DASSL returned unreviewed IDID={idid}")
            }
            Self::SessionFailed => {
                formatter.write_str("DASSL session is unusable after a terminal failure")
            }
            Self::ToleranceUpdateRequired => {
                formatter.write_str("DASSL needs a tolerance update before continuation")
            }
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for DaeError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Callback(error) => Some(error),
            _ => None,
        }
    }
}

/// Independently recomputed and native-workspace diagnostics for one advance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DaeDiagnostics<T> {
    /// Maximum absolute residual from one post-return Rust callback evaluation.
    /// It is `None` if the diagnostic residual callback did not return
    /// [`ResidualAction::Continue`] with finite output.
    pub maximum_absolute_residual: Option<T>,
    /// Root-mean-square residual scaled by DASSL's `RTOL*abs(y)+ATOL` weights.
    pub weighted_residual_norm: Option<T>,
    /// Native cumulative internal step count (`IWORK(11)`).
    pub steps_taken: Option<usize>,
    /// Native cumulative residual-call count (`IWORK(12)`).
    pub residual_evaluations: Option<usize>,
    /// Native cumulative iteration-matrix evaluation count (`IWORK(13)`).
    pub jacobian_evaluations: Option<usize>,
    /// Native cumulative local error-test failure count (`IWORK(14)`).
    pub error_test_failures: Option<usize>,
    /// Native cumulative corrector convergence-failure count (`IWORK(15)`).
    pub convergence_failures: Option<usize>,
}

/// Result of an [`DaeSession::advance_to`] operation.
///
/// The session retains the current `y` and `y'`; use [`DaeSession::state`] and
/// [`DaeSession::state_derivative`] to borrow them without allocating copies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DaeAdvance<T> {
    /// Meaningful current time returned by DASSL.
    pub reached_time: T,
    /// Exact reviewed native status.
    pub status: DaeStatus,
    /// Native counters and an independent residual diagnostic.
    pub diagnostics: DaeDiagnostics<T>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DaeLifecycle {
    Ready,
    Recoverable,
    NeedsToleranceUpdate,
    Failed,
}

#[repr(C)]
struct CallbackHeader {
    unexpected_jacobian: bool,
}

#[repr(C)]
struct CallbackContext<T, F: DaeResidual<T>> {
    header: CallbackHeader,
    residual: *mut F,
    dimension: usize,
    error: Option<F::Error>,
    panicked: bool,
    fatal: bool,
    non_finite: Option<usize>,
    contract_violation: bool,
    _scalar: PhantomData<T>,
}

struct ContextGuard;

impl ContextGuard {
    fn install(pointer: *mut c_void) -> Result<Self, ()> {
        ACTIVE_DASSL_CONTEXT.with(|slot| {
            if slot.get().is_null() {
                slot.set(pointer);
                Ok(Self)
            } else {
                Err(())
            }
        })
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        ACTIVE_DASSL_CONTEXT.with(|slot| slot.set(core::ptr::null_mut()));
    }
}

/// Owned, non-cloneable continuation state over original `SDASSL` or `DDASSL`.
///
/// The session owns `y`, `y'`, scalar/vector tolerance storage, DASSL's real
/// and integer work arrays, and the residual callback. It is intentionally not
/// `Sync`. It can move between threads when Rust automatically permits that for
/// the residual, but every native call remains globally serialized.
pub struct DaeSession<T: DaeScalar, F: DaeResidual<T>> {
    time: T,
    state: Vec<T>,
    state_derivative: Vec<T>,
    residual: F,
    tolerance: DaeTolerance<T>,
    options: DaeOptions<T>,
    real_workspace: Vec<T>,
    integer_workspace: Vec<FortranInteger>,
    diagnostic_residual: Vec<T>,
    info: [FortranInteger; 15],
    lifecycle: DaeLifecycle,
    first_call: bool,
    direction_forward: Option<bool>,
    _not_sync: PhantomData<Cell<()>>,
}

impl<T: DaeScalar, F: DaeResidual<T>> DaeSession<T, F> {
    /// Constructs a residual-only DASSL session from an initially sufficiently
    /// consistent `G(initial_time, initial_state, initial_derivative)=0` pair.
    ///
    /// This constructor does not prove DAE consistency. It validates dimensions,
    /// finite values, tolerances, options, and checked source-specific workspace
    /// formulas, then allocates opaque continuation storage. DASSL's internally
    /// differenced dense or banded Jacobian mode is selected by [`DaeOptions`];
    /// `ResidualAction::Continue` callbacks may run multiple times per native step.
    pub fn new(
        initial_time: T,
        initial_state: Vec<T>,
        initial_derivative: Vec<T>,
        residual: F,
        tolerance: DaeTolerance<T>,
        options: DaeOptions<T>,
    ) -> Result<Self, DaeError<F::Error>> {
        validate_initial(
            initial_time,
            &initial_state,
            &initial_derivative,
            &tolerance,
            options,
        )?;
        let dimension = initial_state.len();
        let (real_length, integer_length) = workspace_lengths(dimension, options)?;
        let mut integer_workspace = allocate(integer_length, 0).map_err(DaeError::InvalidInput)?;
        initialize_static_controls(&mut integer_workspace, options)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            state_derivative: initial_derivative,
            residual,
            tolerance,
            options,
            real_workspace: allocate(real_length, T::zero()).map_err(DaeError::InvalidInput)?,
            integer_workspace,
            diagnostic_residual: allocate(dimension, T::zero()).map_err(DaeError::InvalidInput)?,
            info: [0; 15],
            lifecycle: DaeLifecycle::Ready,
            first_call: true,
            direction_forward: None,
            _not_sync: PhantomData,
        })
    }

    /// Copies slices into an owned residual-only DASSL session.
    ///
    /// This convenience constructor retains no borrows of caller storage;
    /// subsequent continuation state remains private to the session.
    pub fn from_slices(
        initial_time: T,
        initial_state: &[T],
        initial_derivative: &[T],
        residual: F,
        tolerance: DaeTolerance<T>,
        options: DaeOptions<T>,
    ) -> Result<Self, DaeError<F::Error>> {
        let mut state = allocate(initial_state.len(), T::zero()).map_err(DaeError::InvalidInput)?;
        state.copy_from_slice(initial_state);
        let mut derivative =
            allocate(initial_derivative.len(), T::zero()).map_err(DaeError::InvalidInput)?;
        derivative.copy_from_slice(initial_derivative);
        Self::new(
            initial_time,
            state,
            derivative,
            residual,
            tolerance,
            options,
        )
    }

    /// Advances in the established direction to a distinct target time.
    ///
    /// The first call establishes forward or backward direction. Successful
    /// interval-mode returns [`DaeStatus::ReachedTarget`].
    /// Recoverable statuses preserve the current state; terminal failures,
    /// callback errors, panics, and invalid callback requests poison this
    /// continuation history.
    pub fn advance_to(&mut self, target: T) -> Result<DaeAdvance<T>, DaeError<F::Error>> {
        self.validate_target(target)?;
        let status = T::call_native(self, target)?;
        let diagnostics = self.diagnostics(matches!(
            status,
            DaeStatus::ReachedTarget | DaeStatus::ReachedStopTime
        ));
        Ok(DaeAdvance {
            reached_time: self.time,
            status,
            diagnostics,
        })
    }

    /// Replaces tolerances without changing their scalar/vector mode.
    ///
    /// This is the supported recovery operation after
    /// [`DaeStatus::ErrorWeightFailure`]. DASSL's documented continuation
    /// protocol permits changing tolerance entries but not switching between
    /// scalar and vector modes.
    pub fn set_tolerance(&mut self, tolerance: DaeTolerance<T>) -> Result<(), DaeError<F::Error>> {
        if self.lifecycle == DaeLifecycle::Failed {
            return Err(DaeError::SessionFailed);
        }
        validate_tolerance(&tolerance, self.state.len())?;
        if tolerance_mode(&tolerance) != tolerance_mode(&self.tolerance) {
            return Err(DaeError::InvalidInput(DaeInputError::InvalidTolerance));
        }
        self.tolerance = tolerance;
        if self.lifecycle == DaeLifecycle::NeedsToleranceUpdate {
            self.lifecycle = DaeLifecycle::Recoverable;
        }
        Ok(())
    }

    /// Restarts the existing session with a new pair of the same dimension.
    ///
    /// Restart clears every opaque work array, direction, and continuation
    /// flag. It does not calculate consistent initial conditions; callers must
    /// again supply a sufficiently consistent `(y, y')` pair. A dimensional
    /// change requires a new session so native workspace formulas remain exact.
    pub fn restart_from(
        &mut self,
        time: T,
        state: Vec<T>,
        state_derivative: Vec<T>,
    ) -> Result<(), DaeError<F::Error>> {
        if state.len() != self.state.len() {
            return Err(DaeError::InvalidInput(
                DaeInputError::DimensionChangeRequiresNewSession,
            ));
        }
        validate_initial(
            time,
            &state,
            &state_derivative,
            &self.tolerance,
            self.options,
        )?;
        self.time = time;
        self.state = state;
        self.state_derivative = state_derivative;
        self.real_workspace.fill(T::zero());
        self.integer_workspace.fill(0);
        initialize_static_controls(&mut self.integer_workspace, self.options)?;
        self.diagnostic_residual.fill(T::zero());
        self.info = [0; 15];
        self.lifecycle = DaeLifecycle::Ready;
        self.first_call = true;
        self.direction_forward = None;
        Ok(())
    }

    /// Returns the current DASSL time.
    #[must_use]
    pub fn time(&self) -> T {
        self.time
    }

    /// Borrows the current state vector without exposing native workspace.
    #[must_use]
    pub fn state(&self) -> &[T] {
        &self.state
    }

    /// Borrows the current derivative vector without exposing native workspace.
    #[must_use]
    pub fn state_derivative(&self) -> &[T] {
        &self.state_derivative
    }

    /// Returns the fixed DAE dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.state.len()
    }

    /// Returns the validated public DASSL options.
    #[must_use]
    pub fn options(&self) -> DaeOptions<T> {
        self.options
    }

    fn validate_target(&self, target: T) -> Result<(), DaeError<F::Error>> {
        if self.lifecycle == DaeLifecycle::Failed {
            return Err(DaeError::SessionFailed);
        }
        if self.lifecycle == DaeLifecycle::NeedsToleranceUpdate {
            return Err(DaeError::ToleranceUpdateRequired);
        }
        if !target.is_finite() || target == self.time {
            return Err(DaeError::InvalidInput(DaeInputError::InvalidTarget));
        }
        let forward = target > self.time;
        if let Some(expected) = self.direction_forward {
            if forward != expected {
                return Err(DaeError::InvalidInput(DaeInputError::InvalidTarget));
            }
        }
        Ok(())
    }

    fn diagnostics(&mut self, check_residual: bool) -> DaeDiagnostics<T> {
        let (maximum_absolute_residual, weighted_residual_norm) = if check_residual {
            diagnostic_residual(self)
        } else {
            (None, None)
        };
        DaeDiagnostics {
            maximum_absolute_residual,
            weighted_residual_norm,
            steps_taken: counter(&self.integer_workspace, 10),
            residual_evaluations: counter(&self.integer_workspace, 11),
            jacobian_evaluations: counter(&self.integer_workspace, 12),
            error_test_failures: counter(&self.integer_workspace, 13),
            convergence_failures: counter(&self.integer_workspace, 14),
        }
    }
}

fn allocate<T: Copy>(length: usize, value: T) -> Result<Vec<T>, DaeInputError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| DaeInputError::AllocationFailed)?;
    values.resize(length, value);
    Ok(values)
}

fn workspace_lengths<T: DaeScalar, E>(
    dimension: usize,
    options: DaeOptions<T>,
) -> Result<(usize, usize), DaeError<E>> {
    let order = options.maximum_order.unwrap_or(DaeOrder::Five).value();
    let history = order
        .checked_add(4)
        .and_then(|value| value.checked_mul(dimension))
        .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    let matrix = match options.jacobian_mode {
        DaeJacobianMode::FiniteDifferenceDense => dimension
            .checked_mul(dimension)
            .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?,
        DaeJacobianMode::FiniteDifferenceBanded {
            lower_bandwidth,
            upper_bandwidth,
        } => {
            let matrix_band = lower_bandwidth
                .checked_add(upper_bandwidth)
                .and_then(|value| value.checked_add(1))
                .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
            let stored_band = lower_bandwidth
                .checked_mul(2)
                .and_then(|value| value.checked_add(upper_bandwidth))
                .and_then(|value| value.checked_add(1))
                .and_then(|value| value.checked_mul(dimension))
                .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
            let save = dimension
                .checked_div(matrix_band)
                .and_then(|value| value.checked_add(1))
                .and_then(|value| value.checked_mul(2))
                .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
            stored_band
                .checked_add(save)
                .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?
        }
    };
    let real = 40_usize
        .checked_add(history)
        .and_then(|value| value.checked_add(matrix))
        .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    let integer = 20_usize
        .checked_add(dimension)
        .ok_or(DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    FortranInteger::try_from(dimension)
        .and_then(|_| FortranInteger::try_from(real))
        .and_then(|_| FortranInteger::try_from(integer))
        .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    Ok((real, integer))
}

fn validate_initial<T: DaeScalar, E>(
    time: T,
    state: &[T],
    derivative: &[T],
    tolerance: &DaeTolerance<T>,
    options: DaeOptions<T>,
) -> Result<(), DaeError<E>> {
    if state.is_empty() {
        return Err(DaeError::InvalidInput(DaeInputError::EmptyState));
    }
    if state.len() != derivative.len() {
        return Err(DaeError::InvalidInput(DaeInputError::DerivativeLength {
            expected: state.len(),
            actual: derivative.len(),
        }));
    }
    if !time.is_finite()
        || state.iter().any(|value| !value.is_finite())
        || derivative.iter().any(|value| !value.is_finite())
    {
        return Err(DaeError::InvalidInput(DaeInputError::NonFiniteInput));
    }
    validate_tolerance(tolerance, state.len())?;
    validate_options(options, state.len())?;
    let _ = workspace_lengths::<T, E>(state.len(), options)?;
    Ok(())
}

fn validate_tolerance<T: DaeScalar, E>(
    tolerance: &DaeTolerance<T>,
    dimension: usize,
) -> Result<(), DaeError<E>> {
    match tolerance {
        DaeTolerance::Scalar { relative, absolute } => {
            validate_tolerance_pair(*relative, *absolute)?;
        }
        DaeTolerance::Vector { relative, absolute } => {
            if relative.len() != dimension {
                return Err(DaeError::InvalidInput(DaeInputError::ToleranceLength {
                    expected: dimension,
                    actual: relative.len(),
                }));
            }
            if absolute.len() != dimension {
                return Err(DaeError::InvalidInput(DaeInputError::ToleranceLength {
                    expected: dimension,
                    actual: absolute.len(),
                }));
            }
            for (&relative, &absolute) in relative.iter().zip(absolute) {
                validate_tolerance_pair(relative, absolute)?;
            }
        }
    }
    Ok(())
}

fn validate_tolerance_pair<T: DaeScalar, E>(relative: T, absolute: T) -> Result<(), DaeError<E>> {
    if !relative.is_finite()
        || !absolute.is_finite()
        || relative < T::zero()
        || absolute < T::zero()
        || (relative == T::zero() && absolute == T::zero())
    {
        return Err(DaeError::InvalidInput(DaeInputError::InvalidTolerance));
    }
    Ok(())
}

fn validate_options<T: DaeScalar, E>(
    options: DaeOptions<T>,
    dimension: usize,
) -> Result<(), DaeError<E>> {
    if let Some(value) = options.maximum_step {
        if !value.is_finite() || value <= T::zero() {
            return Err(DaeError::InvalidInput(DaeInputError::InvalidStepControl));
        }
    }
    if let Some(value) = options.initial_step {
        if !value.is_finite() || value == T::zero() {
            return Err(DaeError::InvalidInput(DaeInputError::InvalidStepControl));
        }
    }
    if let DaeJacobianMode::FiniteDifferenceBanded {
        lower_bandwidth,
        upper_bandwidth,
    } = options.jacobian_mode
    {
        if lower_bandwidth >= dimension || upper_bandwidth >= dimension {
            return Err(DaeError::InvalidInput(DaeInputError::InvalidBandwidth));
        }
    }
    Ok(())
}

fn initialize_static_controls<T: DaeScalar, E>(
    integer_workspace: &mut [FortranInteger],
    options: DaeOptions<T>,
) -> Result<(), DaeError<E>> {
    if let DaeJacobianMode::FiniteDifferenceBanded {
        lower_bandwidth,
        upper_bandwidth,
    } = options.jacobian_mode
    {
        integer_workspace[0] = FortranInteger::try_from(lower_bandwidth)
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
        integer_workspace[1] = FortranInteger::try_from(upper_bandwidth)
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    }
    if let Some(order) = options.maximum_order {
        integer_workspace[2] = FortranInteger::try_from(order.value())
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    }
    Ok(())
}

fn tolerance_mode<T>(tolerance: &DaeTolerance<T>) -> bool {
    matches!(tolerance, DaeTolerance::Vector { .. })
}

fn counter(values: &[FortranInteger], index: usize) -> Option<usize> {
    values
        .get(index)
        .and_then(|value| usize::try_from(*value).ok())
}

unsafe extern "C" fn residual_f32<F: DaeResidual<f32>>(
    time: *mut f32,
    state: *mut f32,
    derivative: *mut f32,
    output: *mut f32,
    ires: *mut FortranInteger,
    _: *mut f32,
    _: *mut FortranInteger,
) {
    // SAFETY: `call_f32` installs a matching context for this exact native call.
    unsafe { dispatch::<f32, F>(time, state, derivative, output, ires) };
}

unsafe extern "C" fn residual_f64<F: DaeResidual<f64>>(
    time: *mut f64,
    state: *mut f64,
    derivative: *mut f64,
    output: *mut f64,
    ires: *mut FortranInteger,
    _: *mut f64,
    _: *mut FortranInteger,
) {
    // SAFETY: `call_f64` installs a matching context for this exact native call.
    unsafe { dispatch::<f64, F>(time, state, derivative, output, ires) };
}

unsafe fn dispatch<T: DaeScalar, F: DaeResidual<T>>(
    time: *mut T,
    state: *mut T,
    derivative: *mut T,
    output: *mut T,
    ires: *mut FortranInteger,
) {
    if ires.is_null() {
        return;
    }
    let pointer = ACTIVE_DASSL_CONTEXT.with(|slot| slot.get());
    if pointer.is_null()
        || time.is_null()
        || state.is_null()
        || derivative.is_null()
        || output.is_null()
    {
        // SAFETY: `ires` was checked non-null above.
        unsafe { *ires = -2 };
        if !pointer.is_null() {
            // SAFETY: a matching call installed this context for its lexical scope.
            unsafe { (&mut *pointer.cast::<CallbackContext<T, F>>()).contract_violation = true };
        }
        return;
    }
    // SAFETY: the matching call installs `CallbackContext<T, F>` before native entry.
    let context = unsafe { &mut *pointer.cast::<CallbackContext<T, F>>() };
    let dimension = context.dimension;
    if dimension == 0
        || ranges_overlap(state, derivative, dimension)
        || ranges_overlap(state, output, dimension)
        || ranges_overlap(derivative, output, dimension)
    {
        context.contract_violation = true;
        // SAFETY: `ires` is non-null.
        unsafe { *ires = -2 };
        return;
    }
    // SAFETY: native DASSL promises each pointer has `NEQ` valid elements.
    let input_state = unsafe { core::slice::from_raw_parts(state, dimension) };
    // SAFETY: native DASSL promises each pointer has `NEQ` valid elements.
    let input_derivative = unsafe { core::slice::from_raw_parts(derivative, dimension) };
    // SAFETY: native DASSL promises output storage has `NEQ` valid elements.
    let residual = unsafe { core::slice::from_raw_parts_mut(output, dimension) };
    // SAFETY: `time` is non-null and points to DASSL's current scalar.
    let callback_time = unsafe { *time };
    if !callback_time.is_finite() {
        context.contract_violation = true;
        // SAFETY: `ires` is non-null.
        unsafe { *ires = -2 };
        return;
    }
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        (&mut *context.residual).evaluate(callback_time, input_state, input_derivative, residual)
    })) {
        Ok(Ok(ResidualAction::Continue)) => {
            if let Some(index) = residual.iter().position(|value| !value.is_finite()) {
                context.non_finite = Some(index);
                // SAFETY: `ires` is non-null.
                unsafe { *ires = -2 };
            }
        }
        Ok(Ok(ResidualAction::RecoverableFailure)) => {
            // SAFETY: DASSL documents `IRES=-1` as a recoverable rejected iterate.
            unsafe { *ires = -1 };
        }
        Ok(Ok(ResidualAction::FatalFailure)) => {
            context.fatal = true;
            // SAFETY: DASSL documents `IRES=-2` as immediate return with IDID=-11.
            unsafe { *ires = -2 };
        }
        Ok(Err(error)) => {
            context.error = Some(error);
            // SAFETY: `ires` is non-null.
            unsafe { *ires = -2 };
        }
        Err(_) => {
            context.panicked = true;
            // SAFETY: `ires` is non-null.
            unsafe { *ires = -2 };
        }
    }
}

unsafe extern "C" fn dummy_jacobian_f32(
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut FortranInteger,
) {
    mark_unexpected_jacobian::<f32>();
}

unsafe extern "C" fn dummy_jacobian_f64(
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut FortranInteger,
) {
    mark_unexpected_jacobian::<f64>();
}

fn mark_unexpected_jacobian<T: DaeScalar>() {
    let pointer = ACTIVE_DASSL_CONTEXT.with(|slot| slot.get());
    if !pointer.is_null() {
        // `CallbackHeader` is the first `repr(C)` field in every installed
        // callback context, independent of scalar and residual type.
        unsafe { (*pointer.cast::<CallbackHeader>()).unexpected_jacobian = true };
    }
    let _ = PhantomData::<T>;
}

fn ranges_overlap<T>(left: *const T, right: *const T, length: usize) -> bool {
    let Some(bytes) = core::mem::size_of::<T>().checked_mul(length) else {
        return true;
    };
    let Some(left_end) = (left as usize).checked_add(bytes) else {
        return true;
    };
    let Some(right_end) = (right as usize).checked_add(bytes) else {
        return true;
    };
    (left as usize) < right_end && (right as usize) < left_end
}

fn call_f32<F: DaeResidual<f32>>(
    session: &mut DaeSession<f32, F>,
    target: f32,
) -> Result<DaeStatus, DaeError<F::Error>> {
    call_native_f32(session, target)
}

fn call_f64<F: DaeResidual<f64>>(
    session: &mut DaeSession<f64, F>,
    target: f64,
) -> Result<DaeStatus, DaeError<F::Error>> {
    call_native_f64(session, target)
}

fn call_native_f32<F: DaeResidual<f32>>(
    session: &mut DaeSession<f32, F>,
    target: f32,
) -> Result<DaeStatus, DaeError<F::Error>> {
    let _runtime = lock_native();
    let mut context = CallbackContext {
        header: CallbackHeader {
            unexpected_jacobian: false,
        },
        residual: &mut session.residual,
        dimension: session.state.len(),
        error: None,
        panicked: false,
        fatal: false,
        non_finite: None,
        contract_violation: false,
        _scalar: PhantomData,
    };
    let pointer = (&mut context as *mut CallbackContext<f32, F>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| DaeError::ReentrantCall)?;
    let status = native_f32(session, target);
    drop(guard);
    finish_callback(session, context, status)
}

fn call_native_f64<F: DaeResidual<f64>>(
    session: &mut DaeSession<f64, F>,
    target: f64,
) -> Result<DaeStatus, DaeError<F::Error>> {
    let _runtime = lock_native();
    let mut context = CallbackContext {
        header: CallbackHeader {
            unexpected_jacobian: false,
        },
        residual: &mut session.residual,
        dimension: session.state.len(),
        error: None,
        panicked: false,
        fatal: false,
        non_finite: None,
        contract_violation: false,
        _scalar: PhantomData,
    };
    let pointer = (&mut context as *mut CallbackContext<f64, F>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| DaeError::ReentrantCall)?;
    let status = native_f64(session, target);
    drop(guard);
    finish_callback(session, context, status)
}

fn finish_callback<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    mut context: CallbackContext<T, F>,
    native: Result<DaeStatus, DaeError<F::Error>>,
) -> Result<DaeStatus, DaeError<F::Error>> {
    if let Some(error) = context.error.take() {
        session.lifecycle = DaeLifecycle::Failed;
        return Err(DaeError::Callback(error));
    }
    if context.panicked {
        session.lifecycle = DaeLifecycle::Failed;
        return Err(DaeError::CallbackPanicked);
    }
    if context.fatal {
        session.lifecycle = DaeLifecycle::Failed;
        return Err(DaeError::ResidualFatalFailure);
    }
    if let Some(index) = context.non_finite {
        session.lifecycle = DaeLifecycle::Failed;
        return Err(DaeError::NonFiniteResidual { index });
    }
    if context.contract_violation || context.header.unexpected_jacobian {
        session.lifecycle = DaeLifecycle::Failed;
        return Err(DaeError::CallbackContractViolation);
    }
    native
}

fn native_f32<F: DaeResidual<f32>>(
    session: &mut DaeSession<f32, F>,
    target: f32,
) -> Result<DaeStatus, DaeError<F::Error>> {
    let mut inputs = native_inputs(session, target)?;
    let mut rpar = 0.0_f32;
    let mut ipar = 0;
    let relative_tolerance = tolerance_relative_pointer(&mut session.tolerance);
    let absolute_tolerance = tolerance_absolute_pointer(&mut session.tolerance);
    let _xerror = permit_recoverable_native_statuses();
    // SAFETY: all arrays are session-owned and satisfy the reviewed dense,
    // internally-differenced DASSL contract; the callback context is installed.
    unsafe {
        slatec_sys::dassl::sdassl(
            residual_f32::<F>,
            &mut inputs.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            session.state_derivative.as_mut_ptr(),
            &mut inputs.target,
            session.info.as_mut_ptr(),
            relative_tolerance,
            absolute_tolerance,
            &mut inputs.idid,
            session.real_workspace.as_mut_ptr(),
            &mut inputs.real_workspace_length,
            session.integer_workspace.as_mut_ptr(),
            &mut inputs.integer_workspace_length,
            &mut rpar,
            &mut ipar,
            dummy_jacobian_f32,
        );
    }
    map_native_status(session, inputs.idid)
}

fn native_f64<F: DaeResidual<f64>>(
    session: &mut DaeSession<f64, F>,
    target: f64,
) -> Result<DaeStatus, DaeError<F::Error>> {
    let mut inputs = native_inputs(session, target)?;
    let mut rpar = 0.0_f64;
    let mut ipar = 0;
    let relative_tolerance = tolerance_relative_pointer(&mut session.tolerance);
    let absolute_tolerance = tolerance_absolute_pointer(&mut session.tolerance);
    let _xerror = permit_recoverable_native_statuses();
    // SAFETY: all arrays are session-owned and satisfy the reviewed dense,
    // internally-differenced DASSL contract; the callback context is installed.
    unsafe {
        slatec_sys::dassl::ddassl(
            residual_f64::<F>,
            &mut inputs.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            session.state_derivative.as_mut_ptr(),
            &mut inputs.target,
            session.info.as_mut_ptr(),
            relative_tolerance,
            absolute_tolerance,
            &mut inputs.idid,
            session.real_workspace.as_mut_ptr(),
            &mut inputs.real_workspace_length,
            session.integer_workspace.as_mut_ptr(),
            &mut inputs.integer_workspace_length,
            &mut rpar,
            &mut ipar,
            dummy_jacobian_f64,
        );
    }
    map_native_status(session, inputs.idid)
}

struct NativeInputs<T> {
    dimension: FortranInteger,
    target: T,
    idid: FortranInteger,
    real_workspace_length: FortranInteger,
    integer_workspace_length: FortranInteger,
}

fn native_inputs<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    target: T,
) -> Result<NativeInputs<T>, DaeError<F::Error>> {
    let forward = target > session.time;
    if session.first_call {
        session.direction_forward = Some(forward);
        configure_first_call_controls(session, forward)?;
        session.info[0] = 0;
    } else {
        configure_continuation_controls(session, forward)?;
        session.info[0] = 1;
    }
    Ok(NativeInputs {
        dimension: FortranInteger::try_from(session.state.len())
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?,
        target,
        idid: 0,
        real_workspace_length: FortranInteger::try_from(session.real_workspace.len())
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?,
        integer_workspace_length: FortranInteger::try_from(session.integer_workspace.len())
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?,
    })
}

fn configure_first_call_controls<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    forward: bool,
) -> Result<(), DaeError<F::Error>> {
    session.info[1] = if tolerance_mode(&session.tolerance) {
        1
    } else {
        0
    };
    session.info[2] = 0;
    session.info[4] = 0;
    session.info[5] = if matches!(
        session.options.jacobian_mode,
        DaeJacobianMode::FiniteDifferenceBanded { .. }
    ) {
        1
    } else {
        0
    };
    session.info[9] = 0;
    session.info[10] = 0;
    configure_mutable_controls(session, forward)?;
    if let Some(order) = session.options.maximum_order {
        session.info[8] = 1;
        session.integer_workspace[2] = FortranInteger::try_from(order.value())
            .map_err(|_| DaeError::InvalidInput(DaeInputError::DimensionOverflow))?;
    } else {
        session.info[8] = 0;
    }
    Ok(())
}

fn configure_continuation_controls<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    forward: bool,
) -> Result<(), DaeError<F::Error>> {
    configure_mutable_controls(session, forward)
}

fn configure_mutable_controls<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    forward: bool,
) -> Result<(), DaeError<F::Error>> {
    // Interval output is fixed in this safe API.  DASSL only permits TSTOP
    // at or beyond TOUT in that mode, so a public stop-time setting would not
    // add a meaningful distinct return path and is deliberately deferred.
    session.info[3] = 0;
    if let Some(maximum_step) = session.options.maximum_step {
        session.info[6] = 1;
        session.real_workspace[1] = maximum_step.abs();
    } else {
        session.info[6] = 0;
    }
    if session.first_call {
        if let Some(initial_step) = session.options.initial_step {
            if (initial_step > T::zero()) != forward {
                return Err(DaeError::InvalidInput(DaeInputError::InvalidStepControl));
            }
            session.info[7] = 1;
            session.real_workspace[2] = initial_step;
        } else {
            session.info[7] = 0;
        }
    }
    Ok(())
}

fn tolerance_relative_pointer<T>(tolerance: &mut DaeTolerance<T>) -> *mut T {
    match tolerance {
        DaeTolerance::Scalar { relative, .. } => relative,
        DaeTolerance::Vector { relative, .. } => relative.as_mut_ptr(),
    }
}

fn tolerance_absolute_pointer<T>(tolerance: &mut DaeTolerance<T>) -> *mut T {
    match tolerance {
        DaeTolerance::Scalar { absolute, .. } => absolute,
        DaeTolerance::Vector { absolute, .. } => absolute.as_mut_ptr(),
    }
}

fn map_native_status<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    idid: FortranInteger,
) -> Result<DaeStatus, DaeError<F::Error>> {
    session.first_call = false;
    match idid {
        2 => {
            session.lifecycle = DaeLifecycle::Ready;
            Ok(DaeStatus::ReachedStopTime)
        }
        3 => {
            session.lifecycle = DaeLifecycle::Ready;
            Ok(DaeStatus::ReachedTarget)
        }
        -1 => {
            session.lifecycle = DaeLifecycle::Recoverable;
            Ok(DaeStatus::ExcessiveWork)
        }
        -2 => {
            session.lifecycle = DaeLifecycle::Recoverable;
            Ok(DaeStatus::ExcessiveAccuracyRequested)
        }
        -3 => {
            session.lifecycle = DaeLifecycle::NeedsToleranceUpdate;
            Ok(DaeStatus::ErrorWeightFailure)
        }
        -6 => terminal_failure(session, DaeNativeFailure::RepeatedErrorTestFailure),
        -7 => terminal_failure(session, DaeNativeFailure::RepeatedConvergenceFailure),
        -8 => terminal_failure(session, DaeNativeFailure::SingularIterationMatrix),
        -9 => terminal_failure(
            session,
            DaeNativeFailure::CombinedConvergenceAndErrorFailure,
        ),
        -10 => terminal_failure(session, DaeNativeFailure::RecoverableResidualFailure),
        -12 => terminal_failure(session, DaeNativeFailure::InitialDerivativeFailure),
        value => {
            session.lifecycle = DaeLifecycle::Failed;
            Err(DaeError::NativeContractViolation { idid: value })
        }
    }
}

fn terminal_failure<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
    failure: DaeNativeFailure,
) -> Result<DaeStatus, DaeError<F::Error>> {
    session.lifecycle = DaeLifecycle::Failed;
    Err(DaeError::NativeFailure(failure))
}

fn diagnostic_residual<T: DaeScalar, F: DaeResidual<T>>(
    session: &mut DaeSession<T, F>,
) -> (Option<T>, Option<T>) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        session.residual.evaluate(
            session.time,
            &session.state,
            &session.state_derivative,
            &mut session.diagnostic_residual,
        )
    }));
    if !matches!(result, Ok(Ok(ResidualAction::Continue)))
        || session
            .diagnostic_residual
            .iter()
            .any(|value| !value.is_finite())
    {
        return (None, None);
    }
    let maximum = session
        .diagnostic_residual
        .iter()
        .fold(T::zero(), |current, value| {
            let value = value.abs();
            if value > current { value } else { current }
        });
    let mut sum = T::zero();
    for (index, residual) in session.diagnostic_residual.iter().enumerate() {
        let (relative, absolute) = tolerance_at(&session.tolerance, index);
        let weight = relative.multiply(session.state[index].abs()).add(absolute);
        if weight <= T::zero() {
            return (Some(maximum), None);
        }
        let scaled = residual.abs().divide(weight);
        sum = sum.add(scaled.multiply(scaled));
    }
    let count = session.state.len();
    let denominator = match count {
        0 => return (Some(maximum), None),
        _ => count,
    };
    let count_as_scalar = T::from_usize(denominator);
    (Some(maximum), Some(sum.divide(count_as_scalar).sqrt()))
}

fn tolerance_at<T: Copy>(tolerance: &DaeTolerance<T>, index: usize) -> (T, T) {
    match tolerance {
        DaeTolerance::Scalar { relative, absolute } => (*relative, *absolute),
        DaeTolerance::Vector { relative, absolute } => (relative[index], absolute[index]),
    }
}
