//! Owned safe sessions for reviewed SLATEC `*DRIV1`, `*DRIV2`, and `*DRIV3`
//! drivers.
//!
//! [`OdeSession`](crate::ode::OdeSession) remains the expert real
//! `SDRIV3`/`DDRIV3` surface for explicit systems, with reviewed functional,
//! finite-difference, and analytic-Jacobian iteration modes.
//! [`Driv1Session`](crate::ode::Driv1Session) supplies the straightforward
//! real continuation drivers, [`Driv2Session`](crate::ode::Driv2Session) adds
//! indexed root events, and [`ComplexDriv1Session`](crate::ode::ComplexDriv1Session)/
//! [`ComplexDriv2Session`](crate::ode::ComplexDriv2Session) provide the reviewed
//! single-precision complex counterparts. Mass matrices, DAEs,
//! interpolation, and `CDRIV3` remain outside this module's safe scope.

use alloc::vec;
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
    pub(super) static ACTIVE_CONTEXT: ThreadCell<*mut c_void> = const { ThreadCell::new(core::ptr::null_mut()) };
}

mod sealed {
    pub trait Scalar {}

    impl Scalar for f32 {}
    impl Scalar for f64 {}
}

/// Real scalar types supported by the reviewed SDRIVE session implementation.
///
/// The only supported implementations are [`f32`] for `SDRIV3` and [`f64`]
/// for `DDRIV3`.
pub trait OdeScalar:
    sealed::Scalar + Copy + PartialOrd + core::fmt::Debug + core::ops::Sub<Output = Self> + 'static
{
    /// Returns whether the value is finite.
    fn is_finite(self) -> bool;
    /// Returns zero.
    fn zero() -> Self;
    /// Returns the absolute value.
    fn abs(self) -> Self;
    /// Dispatches a reviewed call to the matching native SDRIVE precision.
    ///
    /// This implementation detail is sealed to `f32` and `f64`; safe callers
    /// use [`OdeSession::integrate_to`] instead of calling it directly.
    #[doc(hidden)]
    fn call_native<S, E>(
        session: &mut OdeSession<Self, S, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        S: OdeSystem<Self, E>;
}

impl OdeScalar for f32 {
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn zero() -> Self {
        0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn call_native<S, E>(
        session: &mut OdeSession<Self, S, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        S: OdeSystem<Self, E>,
    {
        call_f32(session, target)
    }
}

impl OdeScalar for f64 {
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn zero() -> Self {
        0.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn call_native<S, E>(
        session: &mut OdeSession<Self, S, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        S: OdeSystem<Self, E>,
    {
        call_f64(session, target)
    }
}

/// Absolute error-weight floor for an SDRIVE session.
#[derive(Clone, Debug, PartialEq)]
pub enum OdeTolerance<T> {
    /// A common floor for every component.
    Scalar(T),
    /// A component-specific floor, whose length must equal the state dimension.
    Vector(Vec<T>),
}

/// Tolerances for `y'(t) = f(t, y)` under SDRIVE's error-weight model.
///
/// SDRIVE uses scalar relative accuracy `EPS`; `absolute` becomes `EWT`.
/// This wrapper uses `IERROR = 3` for a scalar floor, with
/// `YWT(i) = max(abs(Y(i)), EWT(1))`, and `IERROR = 4` for a vector floor,
/// with `YWT(i) = max(abs(Y(i)), EWT(i))`.
#[derive(Clone, Debug, PartialEq)]
pub struct OdeTolerances<T> {
    /// Requested scalar relative accuracy.
    pub relative: T,
    /// Scalar or per-component absolute floor.
    pub absolute: OdeTolerance<T>,
}

/// Integration method supported by the reviewed SDRIVE session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OdeMethod {
    /// Nonstiff Adams methods, with maximum order at most 12.
    Adams,
    /// Gear backward-differentiation methods, with maximum order at most 5.
    Bdf,
}

/// Linear iteration used by the explicit `SDRIV3`/`DDRIV3` system.
///
/// The finite-difference variants have no additional Rust callback. Analytic
/// variants are available only through [`OdeSession::new_with_jacobian`], so
/// the native `JACOBN` callback always has a checked Rust owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OdeIteration {
    /// Functional iteration (`MITER = 0`), normally appropriate for nonstiff
    /// systems.
    Functional,
    /// Dense chord iteration with a Jacobian computed by SDRIVE (`MITER = 2`).
    FiniteDifferenceDense,
    /// Banded chord iteration with a Jacobian computed by SDRIVE (`MITER = 5`).
    FiniteDifferenceBanded {
        /// Largest permitted row-minus-column index.
        lower_bandwidth: usize,
        /// Largest permitted column-minus-row index.
        upper_bandwidth: usize,
    },
    /// Dense chord iteration with a checked Rust Jacobian callback (`MITER = 1`).
    AnalyticDense,
    /// Banded chord iteration with a checked Rust Jacobian callback (`MITER = 4`).
    AnalyticBanded {
        /// Largest permitted row-minus-column index.
        lower_bandwidth: usize,
        /// Largest permitted column-minus-row index.
        upper_bandwidth: usize,
    },
}

/// Validated controls for the reviewed explicit SDRIVE expert mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdeOptions<T> {
    /// Adams or BDF selection; automatic switching is intentionally absent.
    pub method: OdeMethod,
    /// Maximum native method order. `None` selects 12 for Adams or 5 for BDF.
    pub maximum_order: Option<usize>,
    /// Maximum native internal steps per call. `None` selects 500.
    pub maximum_steps: Option<usize>,
    /// Optional positive maximum absolute native step size.
    pub maximum_step: Option<T>,
    /// Linear iteration and Jacobian storage model.
    pub iteration: OdeIteration,
}

impl<T> Default for OdeOptions<T> {
    fn default() -> Self {
        Self {
            method: OdeMethod::Adams,
            maximum_order: None,
            maximum_steps: None,
            maximum_step: None,
            iteration: OdeIteration::Functional,
        }
    }
}

/// Input validation failure before any native call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OdeInputError {
    /// The state vector was empty.
    EmptyState,
    /// A time, state, or step input was not finite.
    NonFiniteInput,
    /// A tolerance was negative, zero where unsupported, or non-finite.
    InvalidTolerance,
    /// A component tolerance vector did not match the state dimension.
    ToleranceLength {
        /// Expected state dimension.
        expected: usize,
        /// Supplied tolerance length.
        actual: usize,
    },
    /// The requested method order is outside SDRIVE's documented range.
    InvalidMaximumOrder,
    /// The maximum step count was zero or could not fit native `INTEGER`.
    InvalidMaximumSteps,
    /// The DRIV2 constant error-weight scale was non-finite or non-positive.
    InvalidErrorWeight,
    /// The supplied root-function count did not fit the reviewed driver.
    InvalidRootCount,
    /// A DRIV1/CDRIV1 convenience driver accepts at most 200 equations.
    ConvenienceDriverDimension,
    /// The target did not establish or retain one integration direction.
    InvalidTarget,
    /// A dimension or workspace calculation overflowed the native ABI.
    DimensionOverflow,
    /// A band width was outside SDRIVE's documented `0..N` range.
    InvalidBandwidth,
    /// An analytic iteration was requested without an owned Jacobian callback.
    AnalyticJacobianRequired,
    /// A Jacobian callback was supplied for an iteration that does not call it.
    AnalyticJacobianNotUsed,
}

/// Error returned by an ODE session call.
#[derive(Debug)]
pub enum OdeError<E> {
    /// Rust-side input validation rejected the request.
    InvalidInput(OdeInputError),
    /// The user RHS returned an error; the native callback immediately set `N = 0`.
    Callback(E),
    /// The user RHS panicked; the panic was caught before crossing FFI.
    CallbackPanicked,
    /// The RHS wrote a non-finite derivative entry.
    NonFiniteDerivative {
        /// First non-finite derivative index.
        index: usize,
    },
    /// The user Jacobian callback returned an error after setting native `N = 0`.
    JacobianCallback(E),
    /// The user Jacobian callback panicked; the panic was caught before FFI.
    JacobianCallbackPanicked,
    /// The user Jacobian callback wrote a non-finite matrix entry.
    NonFiniteJacobian {
        /// Zero-based matrix row.
        row: usize,
        /// Zero-based matrix column.
        column: usize,
    },
    /// A root/event callback panicked; the panic was caught before FFI.
    RootCallbackPanicked,
    /// A root/event callback returned a non-finite value.
    NonFiniteRoot {
        /// Zero-based root-function index.
        index: usize,
    },
    /// The session was invoked while a callback context was already active on this thread.
    ReentrantCall,
    /// Native SDRIVE returned a status outside the reviewed RHS-only contract.
    NativeContractViolation {
        /// Native `NSTATE` value.
        nstate: FortranInteger,
        /// Native diagnostic `IERFLG` value.
        ierflg: FortranInteger,
    },
    /// A prior terminal failure makes this session unusable.
    SessionFailed,
}

impl<E: core::fmt::Display> core::fmt::Display for OdeError<E> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInput(error) => write!(formatter, "invalid ODE input: {error:?}"),
            Self::Callback(error) => write!(formatter, "ODE RHS callback failed: {error}"),
            Self::CallbackPanicked => formatter.write_str("ODE RHS callback panicked"),
            Self::NonFiniteDerivative { index } => write!(
                formatter,
                "ODE RHS produced a non-finite derivative at index {index}"
            ),
            Self::JacobianCallback(error) => {
                write!(formatter, "ODE Jacobian callback failed: {error}")
            }
            Self::JacobianCallbackPanicked => formatter.write_str("ODE Jacobian callback panicked"),
            Self::NonFiniteJacobian { row, column } => write!(
                formatter,
                "ODE Jacobian produced a non-finite value at row {row}, column {column}"
            ),
            Self::RootCallbackPanicked => formatter.write_str("ODE root callback panicked"),
            Self::NonFiniteRoot { index } => write!(
                formatter,
                "ODE root callback produced a non-finite value at index {index}"
            ),
            Self::ReentrantCall => formatter.write_str("nested ODE session calls are rejected"),
            Self::NativeContractViolation { nstate, ierflg } => write!(
                formatter,
                "SDRIVE returned unreviewed NSTATE={nstate}, IERFLG={ierflg}"
            ),
            Self::SessionFailed => {
                formatter.write_str("ODE session is unusable after a terminal failure")
            }
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for OdeError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Callback(error) | Self::JacobianCallback(error) => Some(error),
            _ => None,
        }
    }
}

/// Meaningful non-terminal native completion statuses in the restricted mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OdeStatus {
    /// The requested target was reached exactly (`NTASK = 3`, `NSTATE = 2`).
    ReachedTarget,
    /// The internal-step limit was reached (`NSTATE = 3`); state and time remain reusable.
    ExcessWork,
    /// SDRIVE relaxed `EPS` (`NSTATE = 4`); state and time remain reusable.
    ToleranceAdjusted,
}

/// Result of one [`OdeSession::integrate_to`] call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OdeStepResult<T> {
    /// Current meaningful native time.
    pub time: T,
    /// Exact mapped native status.
    pub status: OdeStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SessionState {
    Ready,
    Recoverable,
    Failed,
}

/// Error returned when a Jacobian writer index is outside its reviewed shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OdeJacobianIndexError;

/// Checked dense Jacobian writer passed to an analytic SDRIVE callback.
///
/// Entries use mathematical zero-based `(row, column)` indices. Storage is
/// Fortran column-major internally, but never exposed through this API.
pub struct OdeDenseJacobianViewMut<'a, T> {
    values: &'a mut [T],
    dimension: usize,
}

impl<'a, T: Copy> OdeDenseJacobianViewMut<'a, T> {
    /// Stores `d f_row / d y_column`.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<(), OdeJacobianIndexError> {
        if row >= self.dimension || column >= self.dimension {
            return Err(OdeJacobianIndexError);
        }
        self.values[column * self.dimension + row] = value;
        Ok(())
    }
}

/// Checked banded Jacobian writer passed to an analytic SDRIVE callback.
///
/// Entries use mathematical zero-based `(row, column)` indices. Only entries
/// within the declared band may be written; all other matrix entries are
/// mathematically zero. The native Fortran leading dimension remains private.
pub struct OdeBandedJacobianViewMut<'a, T> {
    values: &'a mut [T],
    dimension: usize,
    lower_bandwidth: usize,
    upper_bandwidth: usize,
    leading_dimension: usize,
}

impl<'a, T: Copy> OdeBandedJacobianViewMut<'a, T> {
    /// Stores `d f_row / d y_column` when that entry is inside the declared band.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<(), OdeJacobianIndexError> {
        if row >= self.dimension || column >= self.dimension {
            return Err(OdeJacobianIndexError);
        }
        let lower = row.saturating_sub(column);
        let upper = column.saturating_sub(row);
        if lower > self.lower_bandwidth || upper > self.upper_bandwidth {
            return Err(OdeJacobianIndexError);
        }
        let native_row = row + self.upper_bandwidth - column;
        self.values[column * self.leading_dimension + native_row] = value;
        Ok(())
    }
}

/// Internal callback contract used by the sealed real SDRIVE dispatch.
///
/// This is public only because [`OdeScalar`] is public; callers construct the
/// reviewed owners through [`OdeSession::new`] or
/// [`OdeSession::new_with_jacobian`] rather than implementing this trait.
#[doc(hidden)]
pub trait OdeSystem<T: OdeScalar, E> {
    fn rhs(&mut self, time: T, state: &[T], derivative: &mut [T]) -> Result<(), E>;
    fn supports_analytic_jacobian(&self) -> bool;
    fn jacobian(
        &mut self,
        time: T,
        state: &[T],
        values: &mut [T],
        leading_dimension: usize,
        lower_bandwidth: usize,
        upper_bandwidth: usize,
        banded: bool,
    ) -> Result<(), E>;
}

/// Internal owner used by the RHS-only constructor.
pub struct RhsOnly<F>(F);

impl<T: OdeScalar, F, E> OdeSystem<T, E> for RhsOnly<F>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
{
    fn rhs(&mut self, time: T, state: &[T], derivative: &mut [T]) -> Result<(), E> {
        (self.0)(time, state, derivative)
    }

    fn supports_analytic_jacobian(&self) -> bool {
        false
    }

    fn jacobian(
        &mut self,
        _: T,
        _: &[T],
        _: &mut [T],
        _: usize,
        _: usize,
        _: usize,
        _: bool,
    ) -> Result<(), E> {
        unreachable!("RHS-only SDRIVE session cannot receive a Jacobian callback")
    }
}

/// Internal owner used by the analytic-Jacobian constructor.
pub struct RhsAndJacobian<F, J> {
    rhs: F,
    jacobian: J,
}

impl<T: OdeScalar, F, J, E> OdeSystem<T, E> for RhsAndJacobian<F, J>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
    J: FnMut(T, &[T], OdeJacobianWriter<'_, T>) -> Result<(), E>,
{
    fn rhs(&mut self, time: T, state: &[T], derivative: &mut [T]) -> Result<(), E> {
        (self.rhs)(time, state, derivative)
    }

    fn supports_analytic_jacobian(&self) -> bool {
        true
    }

    fn jacobian(
        &mut self,
        time: T,
        state: &[T],
        values: &mut [T],
        leading_dimension: usize,
        lower_bandwidth: usize,
        upper_bandwidth: usize,
        banded: bool,
    ) -> Result<(), E> {
        let writer = if !banded {
            OdeJacobianWriter::Dense(OdeDenseJacobianViewMut {
                values,
                dimension: state.len(),
            })
        } else {
            OdeJacobianWriter::Banded(OdeBandedJacobianViewMut {
                values,
                dimension: state.len(),
                lower_bandwidth,
                upper_bandwidth,
                leading_dimension,
            })
        };
        (self.jacobian)(time, state, writer)
    }
}

/// Jacobian storage passed to the checked analytic SDRIVE callback.
pub enum OdeJacobianWriter<'a, T> {
    /// A full dense matrix.
    Dense(OdeDenseJacobianViewMut<'a, T>),
    /// A matrix restricted to the declared lower and upper bands.
    Banded(OdeBandedJacobianViewMut<'a, T>),
}

impl<'a, T: Copy> OdeJacobianWriter<'a, T> {
    /// Stores `d f_row / d y_column` through the reviewed native layout.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<(), OdeJacobianIndexError> {
        match self {
            Self::Dense(view) => view.set(row, column, value),
            Self::Banded(view) => view.set(row, column, value),
        }
    }
}

#[derive(Clone, Copy)]
enum CallbackFailure {
    Rhs,
    Jacobian,
}

struct CallbackContext<T, S, E> {
    system: *mut S,
    dimension: usize,
    error: Option<E>,
    panicked: bool,
    non_finite: Option<usize>,
    non_finite_jacobian: Option<(usize, usize)>,
    failure: CallbackFailure,
    _scalar: PhantomData<T>,
}

type PreparedContext<T, S, E> = (CallbackContext<T, S, E>, crate::runtime::NativeRuntimeGuard);

pub(super) struct ContextGuard;

impl ContextGuard {
    pub(super) fn install(pointer: *mut c_void) -> Result<Self, ()> {
        ACTIVE_CONTEXT.with(|slot| {
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
        ACTIVE_CONTEXT.with(|slot| slot.set(core::ptr::null_mut()));
    }
}

/// Owned, non-cloneable continuation session over original `SDRIV3` or `DDRIV3`.
///
/// A session owns its current state and opaque native work arrays. All native
/// calls are serialized process-wide. A session is `Send` when its callback is
/// `Send`, but is intentionally not `Sync`; the API also rejects callback-nested
/// solves before native re-entry. The stored workspace is continuation state and
/// is neither cloneable nor exposed for mutation.
pub struct OdeSession<T: OdeScalar, S, E> {
    system: S,
    time: T,
    state: Vec<T>,
    tolerances: OdeTolerances<T>,
    options: OdeOptions<T>,
    work: Vec<T>,
    iwork: Vec<FortranInteger>,
    nstate: FortranInteger,
    direction: Option<bool>,
    lifecycle: SessionState,
    _not_sync: PhantomData<Cell<()>>,
    _error: PhantomData<fn() -> E>,
}

/// Resets the test-only count of simultaneously active SDRIVE native calls.
///
/// This diagnostic is available only with
/// `ode-sdrive-expert-native-tests`. It does not alter solver state or relax
/// the process-wide native runtime lock.
#[cfg(feature = "ode-sdrive-expert-native-tests")]
#[doc(hidden)]
pub fn reset_native_call_concurrency_for_test() {
    crate::runtime::reset_ode_native_call_audit();
}

/// Returns the test-only maximum number of overlapping SDRIVE native calls.
///
/// This diagnostic is available only with
/// `ode-sdrive-expert-native-tests`; a correct serialized run reports one.
#[cfg(feature = "ode-sdrive-expert-native-tests")]
#[doc(hidden)]
pub fn max_native_call_concurrency_for_test() -> usize {
    crate::runtime::max_ode_native_calls()
}

/// Executes a test-only observation while holding the native runtime lock.
///
/// The native XERROR controls are process-global. Integration tests that read
/// them directly use this helper so their before/after observation cannot race
/// another independently executing native test. The lock is reentrant on the
/// current thread, so a safe session call made by `operation` remains covered.
#[cfg(feature = "ode-sdrive-expert-native-tests")]
#[doc(hidden)]
pub fn with_native_runtime_lock_for_test<R>(operation: impl FnOnce() -> R) -> R {
    let _runtime = crate::runtime::lock_native();
    operation()
}

impl<T: OdeScalar, F, E> OdeSession<T, RhsOnly<F>, E>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
{
    /// Creates a new owned RHS-only `SDRIV3`/`DDRIV3` continuation session.
    ///
    /// `f32` selects `SDRIV3`; `f64` selects `DDRIV3`. The session owns its
    /// state and native workspace, uses exact-target `NTASK = 3`, and calls no
    /// root, Jacobian, or mass-matrix callback. Callback errors and panics set
    /// native `N = 0` and make this session terminal without unwinding through
    /// Fortran.
    pub fn new(
        initial_time: T,
        initial_state: Vec<T>,
        rhs: F,
        tolerances: OdeTolerances<T>,
        options: OdeOptions<T>,
    ) -> Result<Self, OdeError<E>> {
        validate_inputs(initial_time, &initial_state, &tolerances, options)?;
        let maximum_order = order(options)?;
        if matches!(
            options.iteration,
            OdeIteration::AnalyticDense | OdeIteration::AnalyticBanded { .. }
        ) {
            return Err(OdeError::InvalidInput(
                OdeInputError::AnalyticJacobianRequired,
            ));
        }
        let work_len = workspace_len(initial_state.len(), maximum_order, options.iteration)?;
        let integer_work_len = integer_workspace_len(initial_state.len(), options.iteration)?;
        if FortranInteger::try_from(work_len).is_err() {
            return Err(OdeError::InvalidInput(OdeInputError::DimensionOverflow));
        }

        Ok(Self {
            system: RhsOnly(rhs),
            time: initial_time,
            state: initial_state,
            tolerances,
            options,
            work: vec![T::zero(); work_len],
            iwork: vec![0; integer_work_len],
            nstate: 1,
            direction: None,
            lifecycle: SessionState::Ready,
            _not_sync: PhantomData,
            _error: PhantomData,
        })
    }

    /// Copies a borrowed initial state into a new owned continuation session.
    pub fn from_slice(
        initial_time: T,
        initial_state: &[T],
        rhs: F,
        tolerances: OdeTolerances<T>,
        options: OdeOptions<T>,
    ) -> Result<Self, OdeError<E>> {
        Self::new(
            initial_time,
            initial_state.to_vec(),
            rhs,
            tolerances,
            options,
        )
    }
}

impl<T: OdeScalar, F, J, E> OdeSession<T, RhsAndJacobian<F, J>, E>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
    J: FnMut(T, &[T], OdeJacobianWriter<'_, T>) -> Result<(), E>,
{
    /// Creates an owned explicit SDRIVE session with a checked analytic Jacobian.
    ///
    /// The selected [`OdeIteration`] must be [`OdeIteration::AnalyticDense`]
    /// or [`OdeIteration::AnalyticBanded`]. Both callbacks are contained: an
    /// error, panic, invalid index, or non-finite Jacobian value sets native
    /// `N = 0` before control returns from Fortran.
    pub fn new_with_jacobian(
        initial_time: T,
        initial_state: Vec<T>,
        rhs: F,
        jacobian: J,
        tolerances: OdeTolerances<T>,
        options: OdeOptions<T>,
    ) -> Result<Self, OdeError<E>> {
        validate_inputs(initial_time, &initial_state, &tolerances, options)?;
        if !matches!(
            options.iteration,
            OdeIteration::AnalyticDense | OdeIteration::AnalyticBanded { .. }
        ) {
            return Err(OdeError::InvalidInput(
                OdeInputError::AnalyticJacobianNotUsed,
            ));
        }
        let maximum_order = order(options)?;
        let work_len = workspace_len(initial_state.len(), maximum_order, options.iteration)?;
        let integer_work_len = integer_workspace_len(initial_state.len(), options.iteration)?;
        if FortranInteger::try_from(work_len).is_err() {
            return Err(OdeError::InvalidInput(OdeInputError::DimensionOverflow));
        }
        Ok(Self {
            system: RhsAndJacobian { rhs, jacobian },
            time: initial_time,
            state: initial_state,
            tolerances,
            options,
            work: vec![T::zero(); work_len],
            iwork: vec![0; integer_work_len],
            nstate: 1,
            direction: None,
            lifecycle: SessionState::Ready,
            _not_sync: PhantomData,
            _error: PhantomData,
        })
    }

    /// Copies a borrowed initial state into a new analytic-Jacobian session.
    pub fn from_slice_with_jacobian(
        initial_time: T,
        initial_state: &[T],
        rhs: F,
        jacobian: J,
        tolerances: OdeTolerances<T>,
        options: OdeOptions<T>,
    ) -> Result<Self, OdeError<E>> {
        Self::new_with_jacobian(
            initial_time,
            initial_state.to_vec(),
            rhs,
            jacobian,
            tolerances,
            options,
        )
    }
}

impl<T: OdeScalar, S, E> OdeSession<T, S, E>
where
    S: OdeSystem<T, E>,
{
    fn prepare_target(&mut self, target_time: T) -> Result<(), OdeError<E>> {
        if self.lifecycle == SessionState::Failed {
            return Err(OdeError::SessionFailed);
        }
        if !target_time.is_finite() || target_time == self.time {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
        }

        let forward = target_time > self.time;
        if let Some(direction) = self.direction {
            if direction != forward {
                return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
            }
        } else {
            self.direction = Some(forward);
        }
        Ok(())
    }

    fn finish_call(
        &mut self,
        result: Result<OdeStatus, OdeError<E>>,
    ) -> Result<OdeStepResult<T>, OdeError<E>> {
        match result {
            Ok(status) => {
                self.lifecycle = if status == OdeStatus::ReachedTarget {
                    SessionState::Ready
                } else {
                    SessionState::Recoverable
                };
                Ok(OdeStepResult {
                    time: self.time,
                    status,
                })
            }
            Err(error) => {
                self.lifecycle = SessionState::Failed;
                Err(error)
            }
        }
    }

    /// Integrates in the established direction to `target_time`.
    ///
    /// `ReachedTarget`, `ExcessWork`, and `ToleranceAdjusted` preserve the
    /// session for a later same-direction continuation. Callback failures,
    /// panics, non-finite derivatives, and unreviewed native terminal states
    /// permanently fail the session.
    pub fn integrate_to(&mut self, target_time: T) -> Result<OdeStepResult<T>, OdeError<E>> {
        self.prepare_target(target_time)?;
        let result = T::call_native(self, target_time);
        self.finish_call(result)
    }

    /// Current owned session time.
    pub fn time(&self) -> T {
        self.time
    }

    /// Current owned solution state.
    pub fn state(&self) -> &[T] {
        &self.state
    }

    /// System dimension.
    pub fn dimension(&self) -> usize {
        self.state.len()
    }

    /// Consumes the session and returns its last state vector.
    pub fn into_state(self) -> Vec<T> {
        self.state
    }
}

fn validate_inputs<T: OdeScalar, E>(
    time: T,
    state: &[T],
    tolerances: &OdeTolerances<T>,
    options: OdeOptions<T>,
) -> Result<(), OdeError<E>> {
    if state.is_empty() {
        return Err(OdeError::InvalidInput(OdeInputError::EmptyState));
    }
    if !time.is_finite() || state.iter().any(|value| !value.is_finite()) {
        return Err(OdeError::InvalidInput(OdeInputError::NonFiniteInput));
    }
    if !tolerances.relative.is_finite() || tolerances.relative < T::zero() {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
    }
    match &tolerances.absolute {
        OdeTolerance::Scalar(value) if !value.is_finite() || *value <= T::zero() => {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
        }
        OdeTolerance::Scalar(_) => {}
        OdeTolerance::Vector(values) => {
            if values.len() != state.len() {
                return Err(OdeError::InvalidInput(OdeInputError::ToleranceLength {
                    expected: state.len(),
                    actual: values.len(),
                }));
            }
            if values
                .iter()
                .any(|value| !value.is_finite() || *value <= T::zero())
            {
                return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
            }
        }
    }
    let _ = order(options)?;
    if let Some(steps) = options.maximum_steps {
        if steps == 0 || FortranInteger::try_from(steps).is_err() {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidMaximumSteps));
        }
    }
    if let Some(step) = options.maximum_step {
        if !step.is_finite() || step <= T::zero() {
            return Err(OdeError::InvalidInput(OdeInputError::NonFiniteInput));
        }
    }
    if FortranInteger::try_from(state.len()).is_err() {
        return Err(OdeError::InvalidInput(OdeInputError::DimensionOverflow));
    }
    if let Some((lower, upper)) = bandwidths(options.iteration) {
        if lower >= state.len() || upper >= state.len() {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidBandwidth));
        }
    }
    Ok(())
}

fn order<T: OdeScalar, E>(options: OdeOptions<T>) -> Result<usize, OdeError<E>> {
    let maximum = match options.method {
        OdeMethod::Adams => 12,
        OdeMethod::Bdf => 5,
    };
    let value = options.maximum_order.unwrap_or(maximum);
    if value == 0 || value > maximum {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidMaximumOrder));
    }
    Ok(value)
}

fn workspace_len<E>(
    dimension: usize,
    maximum_order: usize,
    iteration: OdeIteration,
) -> Result<usize, OdeError<E>> {
    let order_term = match iteration {
        OdeIteration::Functional => maximum_order.checked_add(4),
        _ => maximum_order.checked_add(5),
    }
    .and_then(|value| value.checked_mul(dimension));
    let matrix = match iteration {
        OdeIteration::Functional => Some(0),
        OdeIteration::FiniteDifferenceDense | OdeIteration::AnalyticDense => {
            dimension.checked_mul(dimension)
        }
        OdeIteration::FiniteDifferenceBanded {
            lower_bandwidth,
            upper_bandwidth,
        }
        | OdeIteration::AnalyticBanded {
            lower_bandwidth,
            upper_bandwidth,
        } => lower_bandwidth
            .checked_mul(2)
            .and_then(|value| value.checked_add(upper_bandwidth))
            .and_then(|value| value.checked_add(1))
            .and_then(|value| value.checked_mul(dimension)),
    };
    order_term
        .and_then(|value| matrix.and_then(|matrix| value.checked_add(matrix)))
        .and_then(|value| value.checked_add(250))
        .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))
}

fn integer_workspace_len<E>(
    dimension: usize,
    iteration: OdeIteration,
) -> Result<usize, OdeError<E>> {
    let length = match iteration {
        OdeIteration::Functional => 50,
        _ => dimension
            .checked_add(50)
            .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))?,
    };
    FortranInteger::try_from(length)
        .map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))?;
    Ok(length)
}

fn bandwidths(iteration: OdeIteration) -> Option<(usize, usize)> {
    match iteration {
        OdeIteration::FiniteDifferenceBanded {
            lower_bandwidth,
            upper_bandwidth,
        }
        | OdeIteration::AnalyticBanded {
            lower_bandwidth,
            upper_bandwidth,
        } => Some((lower_bandwidth, upper_bandwidth)),
        _ => None,
    }
}

fn native_iteration(iteration: OdeIteration) -> (FortranInteger, FortranInteger, FortranInteger) {
    match iteration {
        OdeIteration::Functional => (0, 0, 0),
        OdeIteration::FiniteDifferenceDense => (2, 0, 0),
        OdeIteration::FiniteDifferenceBanded {
            lower_bandwidth,
            upper_bandwidth,
        } => (
            5,
            FortranInteger::try_from(lower_bandwidth).expect("validated bandwidth"),
            FortranInteger::try_from(upper_bandwidth).expect("validated bandwidth"),
        ),
        OdeIteration::AnalyticDense => (1, 0, 0),
        OdeIteration::AnalyticBanded {
            lower_bandwidth,
            upper_bandwidth,
        } => (
            4,
            FortranInteger::try_from(lower_bandwidth).expect("validated bandwidth"),
            FortranInteger::try_from(upper_bandwidth).expect("validated bandwidth"),
        ),
    }
}

fn prepare_context<T: OdeScalar, S, E>(
    session: &mut OdeSession<T, S, E>,
) -> Result<PreparedContext<T, S, E>, OdeError<E>>
where
    S: OdeSystem<T, E>,
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let context = CallbackContext {
        system: &mut session.system,
        dimension: session.state.len(),
        error: None,
        panicked: false,
        non_finite: None,
        non_finite_jacobian: None,
        failure: CallbackFailure::Rhs,
        _scalar: PhantomData,
    };
    Ok((context, lock_native()))
}

fn hmax<T: OdeScalar>(session: &OdeSession<T, impl Sized, impl Sized>, target: T) -> T {
    session
        .options
        .maximum_step
        .unwrap_or_else(|| (target - session.time).abs())
}

unsafe extern "C" fn rhs_f32<S, E>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut f32,
    derivative: *mut f32,
) where
    S: OdeSystem<f32, E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f32` has registered the
    // matching context; `dispatch` checks all callback pointers and sizes.
    unsafe { dispatch::<f32, S, E>(n, time, state, derivative) }
}

unsafe extern "C" fn rhs_f64<S, E>(
    n: *mut FortranInteger,
    time: *mut f64,
    state: *mut f64,
    derivative: *mut f64,
) where
    S: OdeSystem<f64, E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f64` has registered the
    // matching context; `dispatch` checks all callback pointers and sizes.
    unsafe { dispatch::<f64, S, E>(n, time, state, derivative) }
}

unsafe fn dispatch<T: OdeScalar, S, E>(
    n: *mut FortranInteger,
    time: *mut T,
    state: *mut T,
    derivative: *mut T,
) where
    S: OdeSystem<T, E>,
{
    if n.is_null() || time.is_null() || state.is_null() || derivative.is_null() {
        if !n.is_null() {
            // SAFETY: non-null pointer was supplied by native code.
            unsafe { *n = 0 };
        }
        return;
    }
    // SAFETY: native code supplied the non-null `N` pointer above.
    let dimension = match usize::try_from(unsafe { *n }) {
        Ok(value) if value > 0 => value,
        _ => {
            // SAFETY: `n` is non-null.
            unsafe { *n = 0 };
            return;
        }
    };
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        // SAFETY: `n` is non-null.
        unsafe { *n = 0 };
        return;
    }
    // SAFETY: `call_f32`/`call_f64` install the matching type-specific
    // context before entering native code and remove it before stack teardown.
    let context = unsafe { &mut *pointer.cast::<CallbackContext<T, S, E>>() };
    context.failure = CallbackFailure::Rhs;
    if dimension != context.dimension || ranges_overlap(state, derivative, dimension) {
        // SAFETY: `n` is non-null. An unexpected length or aliasing layout
        // would make Rust's input/output slice construction unsound.
        unsafe { *n = 0 };
        return;
    }
    // SAFETY: `time` was checked non-null above and native code owns this
    // scalar for the duration of the callback.
    let callback_time = unsafe { *time };
    if !callback_time.is_finite() {
        // SAFETY: `n` is non-null.
        unsafe { *n = 0 };
        return;
    }
    // SAFETY: SDRIVE supplied arrays of exactly `N` elements by contract.
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    // SAFETY: SDRIVE supplied output storage of exactly `N` elements.
    let output = unsafe { core::slice::from_raw_parts_mut(derivative, dimension) };

    match catch_unwind(AssertUnwindSafe(|| unsafe {
        (&mut *context.system).rhs(callback_time, input, output)
    })) {
        Ok(Ok(())) => {
            if let Some(index) = output.iter().position(|value| !value.is_finite()) {
                context.non_finite = Some(index);
                // SAFETY: `n` is non-null.
                unsafe { *n = 0 };
            }
        }
        Ok(Err(error)) => {
            context.error = Some(error);
            // SAFETY: `n` is non-null.
            unsafe { *n = 0 };
        }
        Err(_) => {
            context.panicked = true;
            // SAFETY: `n` is non-null.
            unsafe { *n = 0 };
        }
    }
}

unsafe extern "C" fn jac_f32<S, E>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut f32,
    matrix: *mut f32,
    leading_dimension: *mut FortranInteger,
    lower_bandwidth: *mut FortranInteger,
    upper_bandwidth: *mut FortranInteger,
) where
    S: OdeSystem<f32, E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f32` has registered the
    // matching context and only in a reviewed analytic iteration mode.
    unsafe {
        dispatch_jacobian::<f32, S, E>(
            n,
            time,
            state,
            matrix,
            leading_dimension,
            lower_bandwidth,
            upper_bandwidth,
        )
    }
}

unsafe extern "C" fn jac_f64<S, E>(
    n: *mut FortranInteger,
    time: *mut f64,
    state: *mut f64,
    matrix: *mut f64,
    leading_dimension: *mut FortranInteger,
    lower_bandwidth: *mut FortranInteger,
    upper_bandwidth: *mut FortranInteger,
) where
    S: OdeSystem<f64, E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f64` has registered the
    // matching context and only in a reviewed analytic iteration mode.
    unsafe {
        dispatch_jacobian::<f64, S, E>(
            n,
            time,
            state,
            matrix,
            leading_dimension,
            lower_bandwidth,
            upper_bandwidth,
        )
    }
}

unsafe fn dispatch_jacobian<T: OdeScalar, S, E>(
    n: *mut FortranInteger,
    time: *mut T,
    state: *mut T,
    matrix: *mut T,
    leading_dimension: *mut FortranInteger,
    lower_bandwidth: *mut FortranInteger,
    upper_bandwidth: *mut FortranInteger,
) where
    S: OdeSystem<T, E>,
{
    if n.is_null()
        || time.is_null()
        || state.is_null()
        || matrix.is_null()
        || leading_dimension.is_null()
        || lower_bandwidth.is_null()
        || upper_bandwidth.is_null()
    {
        if !n.is_null() {
            // SAFETY: native supplied this non-null callback control pointer.
            unsafe { *n = 0 };
        }
        return;
    }
    let dimension = match usize::try_from(unsafe { *n }) {
        Ok(value) if value > 0 => value,
        _ => {
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
            return;
        }
    };
    let leading = match usize::try_from(unsafe { *leading_dimension }) {
        Ok(value) if value > 0 => value,
        _ => {
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
            return;
        }
    };
    let lower = match usize::try_from(unsafe { *lower_bandwidth }) {
        Ok(value) => value,
        Err(_) => {
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
            return;
        }
    };
    let upper = match usize::try_from(unsafe { *upper_bandwidth }) {
        Ok(value) => value,
        Err(_) => {
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
            return;
        }
    };
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        // SAFETY: `n` is non-null above.
        unsafe { *n = 0 };
        return;
    }
    // SAFETY: the exact typed context lives for the enclosing native call.
    let context = unsafe { &mut *pointer.cast::<CallbackContext<T, S, E>>() };
    context.failure = CallbackFailure::Jacobian;
    if context.system.is_null()
        || dimension != context.dimension
        || !unsafe { (&*context.system).supports_analytic_jacobian() }
    {
        // SAFETY: `n` is non-null above.
        unsafe { *n = 0 };
        return;
    }
    let banded = leading != dimension || lower != 0 || upper != 0;
    let expected_leading = if banded {
        match lower
            .checked_mul(2)
            .and_then(|value| value.checked_add(upper))
            .and_then(|value| value.checked_add(1))
        {
            Some(value) => value,
            None => {
                // SAFETY: `n` is non-null above.
                unsafe { *n = 0 };
                return;
            }
        }
    } else {
        dimension
    };
    let Some(length) = leading.checked_mul(dimension) else {
        // SAFETY: `n` is non-null above.
        unsafe { *n = 0 };
        return;
    };
    if leading != expected_leading || ranges_overlap(state, matrix, dimension) {
        // SAFETY: `n` is non-null above.
        unsafe { *n = 0 };
        return;
    }
    let callback_time = unsafe { *time };
    if !callback_time.is_finite() {
        // SAFETY: `n` is non-null above.
        unsafe { *n = 0 };
        return;
    }
    // SAFETY: source contract supplies Y(*), DFDY(MATDIM,*), with reviewed
    // `N` and `MATDIM` dimensions. The wrapper checked their multiplication.
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    // SAFETY: source contract supplies writable `MATDIM * N` matrix storage.
    let output = unsafe { core::slice::from_raw_parts_mut(matrix, length) };
    output.fill(T::zero());
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        (&mut *context.system).jacobian(callback_time, input, output, leading, lower, upper, banded)
    })) {
        Ok(Ok(())) => {
            if let Some(index) = output.iter().position(|value| !value.is_finite()) {
                let column = index / leading;
                let native_row = index % leading;
                let row = if banded {
                    native_row.saturating_add(column).saturating_sub(upper)
                } else {
                    native_row
                };
                context.non_finite_jacobian = Some((row, column));
                // SAFETY: `n` is non-null above.
                unsafe { *n = 0 };
            }
        }
        Ok(Err(error)) => {
            context.error = Some(error);
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
        }
        Err(_) => {
            context.panicked = true;
            // SAFETY: `n` is non-null above.
            unsafe { *n = 0 };
        }
    }
}

pub(super) fn ranges_overlap<T>(left: *const T, right: *const T, length: usize) -> bool {
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

#[path = "ode/driv.rs"]
mod driv;

pub use driv::{
    CallbackOdeError, ComplexDriv1Session, ComplexDriv2Session, Driv1Session, Driv2Options,
    Driv2Session, DrivMethod, DrivStatus, DrivStepResult,
};
/// Safe complex scalar type used by the reviewed complex DRIV sessions.
pub use num_complex::Complex32 as OdeComplex32;

unsafe extern "C" fn dummy_mass_f32(
    _: *mut FortranInteger,
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
) {
}

unsafe extern "C" fn dummy_mass_f64(
    _: *mut FortranInteger,
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
) {
}

unsafe extern "C" fn dummy_root_f32(
    _: *mut FortranInteger,
    _: *mut f32,
    _: *mut f32,
    _: *mut FortranInteger,
) -> f32 {
    0.0
}

unsafe extern "C" fn dummy_root_f64(
    _: *mut FortranInteger,
    _: *mut f64,
    _: *mut f64,
    _: *mut FortranInteger,
) -> f64 {
    0.0
}

macro_rules! dummy_users {
    ($name:ident, $scalar:ty) => {
        unsafe extern "C" fn $name(
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut $scalar,
            _: *mut FortranInteger,
            _: *mut FortranInteger,
            _: *mut FortranInteger,
        ) {
        }
    };
}

dummy_users!(dummy_users_f32, f32);
dummy_users!(dummy_users_f64, f64);

fn call_f32<S, E>(
    session: &mut OdeSession<f32, S, E>,
    target: f32,
) -> Result<OdeStatus, OdeError<E>>
where
    S: OdeSystem<f32, E>,
{
    let (mut context, _runtime) = prepare_context(session)?;
    let _shared_context = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let pointer = (&mut context as *mut CallbackContext<f32, S, E>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let result = call_native_f32(session, target);
    drop(guard);
    finish_callback_result(context, result)
}

fn call_f64<S, E>(
    session: &mut OdeSession<f64, S, E>,
    target: f64,
) -> Result<OdeStatus, OdeError<E>>
where
    S: OdeSystem<f64, E>,
{
    let (mut context, _runtime) = prepare_context(session)?;
    let _shared_context = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let pointer = (&mut context as *mut CallbackContext<f64, S, E>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let result = call_native_f64(session, target);
    drop(guard);
    finish_callback_result(context, result)
}

fn finish_callback_result<T: OdeScalar, S, E>(
    mut context: CallbackContext<T, S, E>,
    native: Result<OdeStatus, OdeError<E>>,
) -> Result<OdeStatus, OdeError<E>>
where
    S: OdeSystem<T, E>,
{
    if let Some(error) = context.error.take() {
        return Err(match context.failure {
            CallbackFailure::Rhs => OdeError::Callback(error),
            CallbackFailure::Jacobian => OdeError::JacobianCallback(error),
        });
    }
    if context.panicked {
        return Err(match context.failure {
            CallbackFailure::Rhs => OdeError::CallbackPanicked,
            CallbackFailure::Jacobian => OdeError::JacobianCallbackPanicked,
        });
    }
    if let Some(index) = context.non_finite {
        return Err(OdeError::NonFiniteDerivative { index });
    }
    if let Some((row, column)) = context.non_finite_jacobian {
        return Err(OdeError::NonFiniteJacobian { row, column });
    }
    native
}

fn native_inputs<T: OdeScalar, S, E>(
    session: &mut OdeSession<T, S, E>,
    target: T,
) -> Result<NativeInputs<T>, OdeError<E>>
where
    S: OdeSystem<T, E>,
{
    Ok(NativeInputs {
        dimension: FortranInteger::try_from(session.state.len())
            .map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))?,
        target,
        task: 3,
        roots: 0,
        error_mode: match session.tolerances.absolute {
            OdeTolerance::Scalar(_) => 3,
            OdeTolerance::Vector(_) => 4,
        },
        method: match session.options.method {
            OdeMethod::Adams => 1,
            OdeMethod::Bdf => 2,
        },
        maximum_order: FortranInteger::try_from(order(session.options)?)
            .map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))?,
        maximum_step: hmax(session, target),
        work_length: FortranInteger::try_from(session.work.len())
            .map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))?,
        integer_work_length: FortranInteger::try_from(session.iwork.len())
            .map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))?,
        maximum_steps: FortranInteger::try_from(session.options.maximum_steps.unwrap_or(500))
            .map_err(|_| OdeError::InvalidInput(OdeInputError::InvalidMaximumSteps))?,
    })
}

struct NativeInputs<T> {
    dimension: FortranInteger,
    target: T,
    task: FortranInteger,
    roots: FortranInteger,
    error_mode: FortranInteger,
    method: FortranInteger,
    maximum_order: FortranInteger,
    maximum_step: T,
    work_length: FortranInteger,
    integer_work_length: FortranInteger,
    maximum_steps: FortranInteger,
}

fn call_native_f32<S, E>(
    session: &mut OdeSession<f32, S, E>,
    target: f32,
) -> Result<OdeStatus, OdeError<E>>
where
    S: OdeSystem<f32, E>,
{
    let mut input = native_inputs(session, target)?;
    let mut eps = session.tolerances.relative;
    let ewt = tolerance_pointer(&mut session.tolerances.absolute);
    let (mut miter, mut lower_bandwidth, mut upper_bandwidth) =
        native_iteration(session.options.iteration);
    let mut implementation = 0;
    let mut equation_count = 0;
    let mut error_flag = 0;
    let _xerror = permit_recoverable_native_statuses();
    #[cfg(feature = "ode-sdrive-expert-native-tests")]
    let _native_call_audit = crate::runtime::OdeNativeCallAudit::enter();
    // SAFETY: every pointer points to session-owned storage for the documented
    // SDRIV3 RHS-only mode, and all ABI callback signatures are reviewed.
    unsafe {
        slatec_sys::ode::sdriv3(
            &mut input.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            rhs_f32::<S, E>,
            &mut session.nstate,
            &mut input.target,
            &mut input.task,
            &mut input.roots,
            &mut eps,
            ewt,
            &mut input.error_mode,
            &mut input.method,
            &mut miter,
            &mut implementation,
            &mut lower_bandwidth,
            &mut upper_bandwidth,
            &mut input.maximum_order,
            &mut input.maximum_step,
            session.work.as_mut_ptr(),
            &mut input.work_length,
            session.iwork.as_mut_ptr(),
            &mut input.integer_work_length,
            jac_f32::<S, E>,
            dummy_mass_f32,
            &mut equation_count,
            &mut input.maximum_steps,
            dummy_root_f32,
            dummy_users_f32,
            &mut error_flag,
        );
    }
    session.tolerances.relative = eps;
    map_native_status(session.nstate, error_flag)
        .map_err(|(nstate, ierflg)| OdeError::NativeContractViolation { nstate, ierflg })
}

fn call_native_f64<S, E>(
    session: &mut OdeSession<f64, S, E>,
    target: f64,
) -> Result<OdeStatus, OdeError<E>>
where
    S: OdeSystem<f64, E>,
{
    let mut input = native_inputs(session, target)?;
    let mut eps = session.tolerances.relative;
    let ewt = tolerance_pointer(&mut session.tolerances.absolute);
    let (mut miter, mut lower_bandwidth, mut upper_bandwidth) =
        native_iteration(session.options.iteration);
    let mut implementation = 0;
    let mut equation_count = 0;
    let mut error_flag = 0;
    let _xerror = permit_recoverable_native_statuses();
    #[cfg(feature = "ode-sdrive-expert-native-tests")]
    let _native_call_audit = crate::runtime::OdeNativeCallAudit::enter();
    // SAFETY: every pointer points to session-owned storage for the documented
    // DDRIV3 RHS-only mode, and all ABI callback signatures are reviewed.
    unsafe {
        slatec_sys::ode::ddriv3(
            &mut input.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            rhs_f64::<S, E>,
            &mut session.nstate,
            &mut input.target,
            &mut input.task,
            &mut input.roots,
            &mut eps,
            ewt,
            &mut input.error_mode,
            &mut input.method,
            &mut miter,
            &mut implementation,
            &mut lower_bandwidth,
            &mut upper_bandwidth,
            &mut input.maximum_order,
            &mut input.maximum_step,
            session.work.as_mut_ptr(),
            &mut input.work_length,
            session.iwork.as_mut_ptr(),
            &mut input.integer_work_length,
            jac_f64::<S, E>,
            dummy_mass_f64,
            &mut equation_count,
            &mut input.maximum_steps,
            dummy_root_f64,
            dummy_users_f64,
            &mut error_flag,
        );
    }
    session.tolerances.relative = eps;
    map_native_status(session.nstate, error_flag)
        .map_err(|(nstate, ierflg)| OdeError::NativeContractViolation { nstate, ierflg })
}

fn tolerance_pointer<T>(tolerance: &mut OdeTolerance<T>) -> *mut T {
    match tolerance {
        OdeTolerance::Scalar(value) => value,
        OdeTolerance::Vector(values) => values.as_mut_ptr(),
    }
}

fn map_native_status(
    nstate: FortranInteger,
    error_flag: FortranInteger,
) -> Result<OdeStatus, (FortranInteger, FortranInteger)> {
    match nstate {
        2 => Ok(OdeStatus::ReachedTarget),
        3 => Ok(OdeStatus::ExcessWork),
        4 => Ok(OdeStatus::ToleranceAdjusted),
        value => Err((value, error_flag)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shared_callback_guard_rejects_cross_family_reentry() {
        let _guard = crate::callback_runtime::reserve_external_callback_context().unwrap();
        assert!(crate::callback_runtime::with_f64(|value| value, |_| ()).is_err());
    }

    #[test]
    fn callback_ranges_reject_equal_and_partial_aliases() {
        let values = [0.0_f64; 4];
        let start = values.as_ptr();
        // SAFETY: this test compares pointer ranges only and does not dereference
        // the offset pointer.
        let offset = unsafe { start.add(1) };
        assert!(super::ranges_overlap(start, start, 2));
        assert!(super::ranges_overlap(start, offset, 2));
        // SAFETY: the resulting pointer is one past the array and used only for
        // integer-address range comparison.
        let disjoint = unsafe { start.add(4) };
        assert!(!super::ranges_overlap(start, disjoint, 4));
    }

    #[test]
    fn restricted_workspace_formula_is_exact() {
        assert_eq!(
            super::workspace_len::<()>(3, 12, super::OdeIteration::Functional).unwrap(),
            298
        );
        assert_eq!(
            super::workspace_len::<()>(3, 5, super::OdeIteration::FiniteDifferenceDense).unwrap(),
            289
        );
        assert_eq!(
            super::workspace_len::<()>(
                3,
                5,
                super::OdeIteration::FiniteDifferenceBanded {
                    lower_bandwidth: 1,
                    upper_bandwidth: 1,
                },
            )
            .unwrap(),
            292
        );
        assert!(
            super::workspace_len::<()>(usize::MAX, 12, super::OdeIteration::Functional).is_err()
        );
    }
}
