//! Owned safe sessions for the reviewed SLATEC `SDRIV3` and `DDRIV3` drivers.
//!
//! This first ODE surface solves only real explicit initial-value problems
//! `y'(t) = f(t, y)`. It deliberately excludes roots, Jacobians, mass
//! matrices, DAEs, complex systems, and interpolation APIs.

use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;
use core::ffi::c_void;
use core::marker::PhantomData;
use std::cell::Cell as ThreadCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread_local;

use slatec_sys::FortranInteger;

use crate::runtime::{lock_native, permit_recoverable_ode_statuses};

thread_local! {
    static ACTIVE_CONTEXT: ThreadCell<*mut c_void> = const { ThreadCell::new(core::ptr::null_mut()) };
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
    fn call_native<F, E>(
        session: &mut OdeSession<Self, F, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        F: FnMut(Self, &[Self], &mut [Self]) -> Result<(), E>;
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

    fn call_native<F, E>(
        session: &mut OdeSession<Self, F, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        F: FnMut(Self, &[Self], &mut [Self]) -> Result<(), E>,
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

    fn call_native<F, E>(
        session: &mut OdeSession<Self, F, E>,
        target: Self,
    ) -> Result<OdeStatus, OdeError<E>>
    where
        F: FnMut(Self, &[Self], &mut [Self]) -> Result<(), E>,
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

/// Integration method supported by the RHS-only session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OdeMethod {
    /// Nonstiff Adams methods, with maximum order at most 12.
    Adams,
    /// Gear backward-differentiation methods, with maximum order at most 5.
    Bdf,
}

/// Validated controls for the restricted SDRIVE expert mode.
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
}

impl<T> Default for OdeOptions<T> {
    fn default() -> Self {
        Self {
            method: OdeMethod::Adams,
            maximum_order: None,
            maximum_steps: None,
            maximum_step: None,
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
    /// The target did not establish or retain one integration direction.
    InvalidTarget,
    /// A dimension or workspace calculation overflowed the native ABI.
    DimensionOverflow,
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
            Self::Callback(error) => Some(error),
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

struct CallbackContext<T, F, E> {
    rhs: *mut F,
    dimension: usize,
    error: Option<E>,
    panicked: bool,
    non_finite: Option<usize>,
    _scalar: PhantomData<T>,
}

type PreparedContext<T, F, E> = (CallbackContext<T, F, E>, crate::runtime::NativeRuntimeGuard);

struct ContextGuard;

impl ContextGuard {
    fn install(pointer: *mut c_void) -> Result<Self, ()> {
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
pub struct OdeSession<T: OdeScalar, F, E> {
    rhs: F,
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

impl<T: OdeScalar, F, E> OdeSession<T, F, E>
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
        let work_len = workspace_len(initial_state.len(), maximum_order)?;
        if FortranInteger::try_from(work_len).is_err() {
            return Err(OdeError::InvalidInput(OdeInputError::DimensionOverflow));
        }

        Ok(Self {
            rhs,
            time: initial_time,
            state: initial_state,
            tolerances,
            options,
            work: vec![T::zero(); work_len],
            iwork: vec![0; 50],
            nstate: 1,
            direction: None,
            lifecycle: SessionState::Ready,
            _not_sync: PhantomData,
            _error: PhantomData,
        })
    }

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

fn workspace_len<E>(dimension: usize, maximum_order: usize) -> Result<usize, OdeError<E>> {
    maximum_order
        .checked_add(4)
        .and_then(|value| value.checked_mul(dimension))
        .and_then(|value| value.checked_add(250))
        .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))
}

fn prepare_context<T: OdeScalar, F, E>(
    session: &mut OdeSession<T, F, E>,
) -> Result<PreparedContext<T, F, E>, OdeError<E>>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let context = CallbackContext {
        rhs: &mut session.rhs,
        dimension: session.state.len(),
        error: None,
        panicked: false,
        non_finite: None,
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

unsafe extern "C" fn rhs_f32<F, E>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut f32,
    derivative: *mut f32,
) where
    F: FnMut(f32, &[f32], &mut [f32]) -> Result<(), E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f32` has registered the
    // matching context; `dispatch` checks all callback pointers and sizes.
    unsafe { dispatch::<f32, F, E>(n, time, state, derivative) }
}

unsafe extern "C" fn rhs_f64<F, E>(
    n: *mut FortranInteger,
    time: *mut f64,
    state: *mut f64,
    derivative: *mut f64,
) where
    F: FnMut(f64, &[f64], &mut [f64]) -> Result<(), E>,
{
    // SAFETY: SDRIVE invokes this only while `call_f64` has registered the
    // matching context; `dispatch` checks all callback pointers and sizes.
    unsafe { dispatch::<f64, F, E>(n, time, state, derivative) }
}

unsafe fn dispatch<T: OdeScalar, F, E>(
    n: *mut FortranInteger,
    time: *mut T,
    state: *mut T,
    derivative: *mut T,
) where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
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
    let context = unsafe { &mut *pointer.cast::<CallbackContext<T, F, E>>() };
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
        (&mut *context.rhs)(callback_time, input, output)
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

unsafe extern "C" fn dummy_jac_f32(
    _: *mut FortranInteger,
    _: *mut f32,
    _: *mut f32,
    _: *mut f32,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
) {
}

unsafe extern "C" fn dummy_jac_f64(
    _: *mut FortranInteger,
    _: *mut f64,
    _: *mut f64,
    _: *mut f64,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
    _: *mut FortranInteger,
) {
}

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

fn call_f32<F, E>(
    session: &mut OdeSession<f32, F, E>,
    target: f32,
) -> Result<OdeStatus, OdeError<E>>
where
    F: FnMut(f32, &[f32], &mut [f32]) -> Result<(), E>,
{
    let (mut context, _runtime) = prepare_context(session)?;
    let pointer = (&mut context as *mut CallbackContext<f32, F, E>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let result = call_native_f32(session, target);
    drop(guard);
    finish_callback_result(context, result)
}

fn call_f64<F, E>(
    session: &mut OdeSession<f64, F, E>,
    target: f64,
) -> Result<OdeStatus, OdeError<E>>
where
    F: FnMut(f64, &[f64], &mut [f64]) -> Result<(), E>,
{
    let (mut context, _runtime) = prepare_context(session)?;
    let pointer = (&mut context as *mut CallbackContext<f64, F, E>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let result = call_native_f64(session, target);
    drop(guard);
    finish_callback_result(context, result)
}

fn finish_callback_result<T: OdeScalar, F, E>(
    mut context: CallbackContext<T, F, E>,
    native: Result<OdeStatus, OdeError<E>>,
) -> Result<OdeStatus, OdeError<E>>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
{
    if let Some(error) = context.error.take() {
        return Err(OdeError::Callback(error));
    }
    if context.panicked {
        return Err(OdeError::CallbackPanicked);
    }
    if let Some(index) = context.non_finite {
        return Err(OdeError::NonFiniteDerivative { index });
    }
    native
}

fn native_inputs<T: OdeScalar, F, E>(
    session: &mut OdeSession<T, F, E>,
    target: T,
) -> Result<NativeInputs<T>, OdeError<E>>
where
    F: FnMut(T, &[T], &mut [T]) -> Result<(), E>,
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

fn call_native_f32<F, E>(
    session: &mut OdeSession<f32, F, E>,
    target: f32,
) -> Result<OdeStatus, OdeError<E>>
where
    F: FnMut(f32, &[f32], &mut [f32]) -> Result<(), E>,
{
    let mut input = native_inputs(session, target)?;
    let mut eps = session.tolerances.relative;
    let ewt = tolerance_pointer(&mut session.tolerances.absolute);
    let mut miter = 0;
    let mut implementation = 0;
    let mut lower_bandwidth = 0;
    let mut upper_bandwidth = 0;
    let mut equation_count = 0;
    let mut error_flag = 0;
    let _xerror = permit_recoverable_ode_statuses();
    // SAFETY: every pointer points to session-owned storage for the documented
    // SDRIV3 RHS-only mode, and all ABI callback signatures are reviewed.
    unsafe {
        slatec_sys::ode::sdriv3(
            &mut input.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            rhs_f32::<F, E>,
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
            dummy_jac_f32,
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

fn call_native_f64<F, E>(
    session: &mut OdeSession<f64, F, E>,
    target: f64,
) -> Result<OdeStatus, OdeError<E>>
where
    F: FnMut(f64, &[f64], &mut [f64]) -> Result<(), E>,
{
    let mut input = native_inputs(session, target)?;
    let mut eps = session.tolerances.relative;
    let ewt = tolerance_pointer(&mut session.tolerances.absolute);
    let mut miter = 0;
    let mut implementation = 0;
    let mut lower_bandwidth = 0;
    let mut upper_bandwidth = 0;
    let mut equation_count = 0;
    let mut error_flag = 0;
    let _xerror = permit_recoverable_ode_statuses();
    // SAFETY: every pointer points to session-owned storage for the documented
    // DDRIV3 RHS-only mode, and all ABI callback signatures are reviewed.
    unsafe {
        slatec_sys::ode::ddriv3(
            &mut input.dimension,
            &mut session.time,
            session.state.as_mut_ptr(),
            rhs_f64::<F, E>,
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
            dummy_jac_f64,
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
        assert_eq!(super::workspace_len::<()>(3, 12).unwrap(), 298);
        assert!(super::workspace_len::<()>(usize::MAX, 12).is_err());
    }
}
