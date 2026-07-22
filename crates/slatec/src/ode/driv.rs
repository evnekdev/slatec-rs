//! Safe continuation sessions for reviewed `*DRIV1` and `*DRIV2` drivers.

use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;
use core::ffi::c_void;
use core::marker::PhantomData;
use std::panic::{AssertUnwindSafe, catch_unwind};

use num_complex::Complex32 as SafeComplex32;
use slatec_core::to_fortran_integer;
use slatec_sys::Complex32 as NativeComplex32;
use slatec_sys::FortranInteger;

use super::{ACTIVE_CONTEXT, ContextGuard, OdeError, OdeInputError, ranges_overlap};

/// Error type for callback-only DRIV1/2 sessions.
///
/// These sessions accept infallible Rust closures; callback panic, non-finite
/// output, invalid input, nesting, and native contract failures use the
/// existing [`OdeError`] variants. The generic callback-error variant is
/// uninhabited.
pub type CallbackOdeError = OdeError<Infallible>;

/// Source-accurate method selection for `SDRIV2`, `DDRIV2`, and `CDRIV2`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrivMethod {
    /// Nonstiff Adams method (`MINT = 1`).
    Adams,
    /// Stiff Gear method (`MINT = 2`).
    Gear,
    /// Dynamic Adams/Gear selection (`MINT = 3`).
    Automatic,
}

impl DrivMethod {
    const fn native(self) -> FortranInteger {
        match self {
            Self::Adams => 1,
            Self::Gear => 2,
            Self::Automatic => 3,
        }
    }
}

/// Checked controls for an advanced `*DRIV2` continuation session.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Driv2Options<T> {
    /// Relative accuracy request (`EPS`).
    pub relative_tolerance: T,
    /// Positive constant error-weight scale (`EWT`).
    pub error_weight: T,
    /// Native Adams, Gear, or automatic selection.
    pub method: DrivMethod,
    /// Number of indexed root equations. Zero disables event handling.
    pub root_count: usize,
}

impl Default for Driv2Options<f32> {
    fn default() -> Self {
        Self {
            relative_tolerance: 1.0e-5,
            error_weight: 1.0,
            method: DrivMethod::Gear,
            root_count: 0,
        }
    }
}

impl Default for Driv2Options<f64> {
    fn default() -> Self {
        Self {
            relative_tolerance: 1.0e-10,
            error_weight: 1.0,
            method: DrivMethod::Gear,
            root_count: 0,
        }
    }
}

/// Meaningful source completion state for the safe convenience and advanced
/// SDRIVE sessions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrivStatus {
    /// The requested output point was reached.
    ReachedTarget,
    /// The internal work limit was reached; state remains available to inspect.
    ExcessWork,
    /// Native code raised an overambitious relative tolerance.
    ToleranceAdjusted,
    /// An indexed root equation changed sign.
    RootFound {
        /// Zero-based root-function index.
        index: usize,
    },
    /// A negative-direction call reached the output point by interpolation.
    Interpolated,
    /// The native driver stopped after a controlled callback request.
    CallbackStopped,
}

/// Result of one safe `*DRIV1` or `*DRIV2` continuation call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrivStepResult<T> {
    /// Current native time.
    pub time: T,
    /// Exact mapped source completion state.
    pub status: DrivStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Lifecycle {
    Ready,
    Recoverable,
    Failed,
}

/// Owned continuation state for `SDRIV1` or `DDRIV1`.
///
/// It owns the opaque native `WORK` array and preserves it across calls. New
/// sessions begin with `MSTATE = 1`; after every successful or recoverable
/// return the native continuation value is retained exactly. A callback panic,
/// non-finite derivative, or unreviewed native failure makes the session
/// terminal and prevents accidental reuse of corrupted workspace state.
pub struct Driv1Session<T> {
    time: T,
    state: Vec<T>,
    relative_tolerance: T,
    work: Vec<T>,
    mstate: FortranInteger,
    direction: Option<bool>,
    lifecycle: Lifecycle,
}

/// Owned continuation state for `SDRIV2` or `DDRIV2`.
///
/// In addition to state and `WORK`, this object owns the opaque `IWORK` root
/// and continuation storage. The root count and method are fixed at creation;
/// changing them requires a new session, as required by the source protocol.
pub struct Driv2Session<T> {
    time: T,
    state: Vec<T>,
    options: Driv2Options<T>,
    work: Vec<T>,
    iwork: Vec<FortranInteger>,
    mstate: FortranInteger,
    direction: Option<bool>,
    lifecycle: Lifecycle,
}

/// Owned continuation state for complex single-precision `CDRIV1`.
pub struct ComplexDriv1Session {
    time: f32,
    state: Vec<SafeComplex32>,
    relative_tolerance: f32,
    work: Vec<NativeComplex32>,
    mstate: FortranInteger,
    direction: Option<bool>,
    lifecycle: Lifecycle,
}

/// Owned continuation state for complex single-precision `CDRIV2`.
pub struct ComplexDriv2Session {
    time: f32,
    state: Vec<SafeComplex32>,
    options: Driv2Options<f32>,
    work: Vec<NativeComplex32>,
    iwork: Vec<FortranInteger>,
    mstate: FortranInteger,
    direction: Option<bool>,
    lifecycle: Lifecycle,
}

const _: () =
    assert!(core::mem::size_of::<SafeComplex32>() == core::mem::size_of::<NativeComplex32>());
const _: () =
    assert!(core::mem::align_of::<SafeComplex32>() == core::mem::align_of::<NativeComplex32>());

fn native_complex(value: SafeComplex32) -> NativeComplex32 {
    NativeComplex32 {
        re: value.re,
        im: value.im,
    }
}

fn safe_complex(value: NativeComplex32) -> SafeComplex32 {
    SafeComplex32::new(value.re, value.im)
}

fn finite_complex(value: SafeComplex32) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RealCallbackFailure {
    RhsPanicked,
    RhsNonFinite(usize),
    RootPanicked,
    RootNonFinite(usize),
    NativeContract,
}

struct RealContext<T, F, G> {
    rhs: *mut F,
    root: Option<*mut G>,
    dimension: usize,
    root_count: usize,
    failure: Option<RealCallbackFailure>,
    _scalar: PhantomData<T>,
}

struct ComplexContext<F, G> {
    rhs: *mut F,
    root: Option<*mut G>,
    dimension: usize,
    root_count: usize,
    failure: Option<RealCallbackFailure>,
}

fn native_integer(value: usize) -> Result<FortranInteger, CallbackOdeError> {
    to_fortran_integer(value).map_err(|_| OdeError::InvalidInput(OdeInputError::DimensionOverflow))
}

fn workspace1_len(n: usize) -> Result<usize, CallbackOdeError> {
    n.checked_mul(n)
        .and_then(|value| value.checked_add(11usize.checked_mul(n)?))
        .and_then(|value| value.checked_add(300))
        .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))
}

fn workspace2_len(n: usize, roots: usize, method: DrivMethod) -> Result<usize, CallbackOdeError> {
    let root_terms = 2usize
        .checked_mul(roots)
        .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))?;
    let base = match method {
        DrivMethod::Adams => 16usize.checked_mul(n),
        DrivMethod::Gear => n
            .checked_mul(n)
            .and_then(|value| value.checked_add(10usize.checked_mul(n)?)),
        DrivMethod::Automatic => n
            .checked_mul(n)
            .and_then(|value| value.checked_add(17usize.checked_mul(n)?)),
    }
    .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))?;
    base.checked_add(root_terms)
        .and_then(|value| value.checked_add(250))
        .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow))
}

fn iwork2_len(n: usize, method: DrivMethod) -> Result<usize, CallbackOdeError> {
    match method {
        DrivMethod::Adams => Ok(50),
        DrivMethod::Gear | DrivMethod::Automatic => n
            .checked_add(50)
            .ok_or(OdeError::InvalidInput(OdeInputError::DimensionOverflow)),
    }
}

fn validate_f32(time: f32, state: &[f32], tolerance: f32) -> Result<(), CallbackOdeError> {
    if state.is_empty() {
        return Err(OdeError::InvalidInput(OdeInputError::EmptyState));
    }
    if !time.is_finite() || state.iter().any(|value| !value.is_finite()) {
        return Err(OdeError::InvalidInput(OdeInputError::NonFiniteInput));
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
    }
    native_integer(state.len())?;
    Ok(())
}

fn validate_f64(time: f64, state: &[f64], tolerance: f64) -> Result<(), CallbackOdeError> {
    if state.is_empty() {
        return Err(OdeError::InvalidInput(OdeInputError::EmptyState));
    }
    if !time.is_finite() || state.iter().any(|value| !value.is_finite()) {
        return Err(OdeError::InvalidInput(OdeInputError::NonFiniteInput));
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
    }
    native_integer(state.len())?;
    Ok(())
}

fn validate_target_f32(
    time: f32,
    target: f32,
    direction: &mut Option<bool>,
    lifecycle: Lifecycle,
) -> Result<(), CallbackOdeError> {
    if lifecycle == Lifecycle::Failed {
        return Err(OdeError::SessionFailed);
    }
    if !target.is_finite() || target == time {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
    }
    let forward = target > time;
    if direction.is_some_and(|previous| previous != forward) {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
    }
    *direction = Some(forward);
    Ok(())
}

fn validate_target_f64(
    time: f64,
    target: f64,
    direction: &mut Option<bool>,
    lifecycle: Lifecycle,
) -> Result<(), CallbackOdeError> {
    if lifecycle == Lifecycle::Failed {
        return Err(OdeError::SessionFailed);
    }
    if !target.is_finite() || target == time {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
    }
    let forward = target > time;
    if direction.is_some_and(|previous| previous != forward) {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTarget));
    }
    *direction = Some(forward);
    Ok(())
}

fn finish_failure(failure: Option<RealCallbackFailure>) -> Result<(), CallbackOdeError> {
    match failure {
        None => Ok(()),
        Some(RealCallbackFailure::RhsPanicked) => Err(OdeError::CallbackPanicked),
        Some(RealCallbackFailure::RhsNonFinite(index)) => {
            Err(OdeError::NonFiniteDerivative { index })
        }
        Some(RealCallbackFailure::RootPanicked) => Err(OdeError::RootCallbackPanicked),
        Some(RealCallbackFailure::RootNonFinite(index)) => Err(OdeError::NonFiniteRoot { index }),
        Some(RealCallbackFailure::NativeContract) => Err(OdeError::NativeContractViolation {
            nstate: 0,
            ierflg: 0,
        }),
    }
}

unsafe extern "C" fn real_rhs_f32<F, G>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut f32,
    derivative: *mut f32,
) where
    F: FnMut(f32, &[f32], &mut [f32]),
{
    if n.is_null() || time.is_null() || state.is_null() || derivative.is_null() {
        if !n.is_null() {
            // Safety: native supplied a valid N pointer for this callback.
            unsafe { *n = 0 };
        }
        return;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        // Safety: N was checked non-null above.
        unsafe { *n = 0 };
        return;
    }
    // Safety: the scoped DRIV call installed this exact context type.
    let context = unsafe { &mut *pointer.cast::<RealContext<f32, F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    };
    if dimension != context.dimension
        || dimension == 0
        || ranges_overlap(state, derivative, dimension)
        || !unsafe { *time }.is_finite()
    {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    }
    // Safety: reviewed ABI gives exactly N readable Y and writable YDOT words.
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let output = unsafe { core::slice::from_raw_parts_mut(derivative, dimension) };
    let callback = unsafe { &mut *context.rhs };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, input, output)
    })) {
        Ok(()) => {
            if let Some(index) = output.iter().position(|value| !value.is_finite()) {
                context.failure = Some(RealCallbackFailure::RhsNonFinite(index));
                unsafe { *n = 0 };
            }
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RhsPanicked);
            unsafe { *n = 0 };
        }
    }
}

unsafe extern "C" fn real_rhs_f64<F, G>(
    n: *mut FortranInteger,
    time: *mut f64,
    state: *mut f64,
    derivative: *mut f64,
) where
    F: FnMut(f64, &[f64], &mut [f64]),
{
    if n.is_null() || time.is_null() || state.is_null() || derivative.is_null() {
        if !n.is_null() {
            unsafe { *n = 0 };
        }
        return;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        unsafe { *n = 0 };
        return;
    }
    let context = unsafe { &mut *pointer.cast::<RealContext<f64, F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    };
    if dimension != context.dimension
        || dimension == 0
        || ranges_overlap(state, derivative, dimension)
        || !unsafe { *time }.is_finite()
    {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    }
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let output = unsafe { core::slice::from_raw_parts_mut(derivative, dimension) };
    let callback = unsafe { &mut *context.rhs };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, input, output)
    })) {
        Ok(()) => {
            if let Some(index) = output.iter().position(|value| !value.is_finite()) {
                context.failure = Some(RealCallbackFailure::RhsNonFinite(index));
                unsafe { *n = 0 };
            }
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RhsPanicked);
            unsafe { *n = 0 };
        }
    }
}

unsafe extern "C" fn real_root_f32<F, G>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut f32,
    root: *mut FortranInteger,
) -> f32
where
    F: FnMut(f32, &[f32], &mut [f32]),
    G: FnMut(f32, &[f32], usize) -> f32,
{
    if n.is_null() || time.is_null() || state.is_null() || root.is_null() {
        if !n.is_null() {
            unsafe { *n = 0 };
        }
        return 0.0;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        unsafe { *n = 0 };
        return 0.0;
    }
    let context = unsafe { &mut *pointer.cast::<RealContext<f32, F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Ok(root_index) = usize::try_from(unsafe { *root }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Some(root_index) = root_index
        .checked_sub(1)
        .filter(|index| *index < context.root_count)
    else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    if dimension != context.dimension || !unsafe { *time }.is_finite() {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    }
    let Some(root_callback) = context.root else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let callback = unsafe { &mut *root_callback };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, input, root_index)
    })) {
        Ok(value) if value.is_finite() => value,
        Ok(_) => {
            context.failure = Some(RealCallbackFailure::RootNonFinite(root_index));
            unsafe { *n = 0 };
            0.0
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RootPanicked);
            unsafe { *n = 0 };
            0.0
        }
    }
}

unsafe extern "C" fn real_root_f64<F, G>(
    n: *mut FortranInteger,
    time: *mut f64,
    state: *mut f64,
    root: *mut FortranInteger,
) -> f64
where
    F: FnMut(f64, &[f64], &mut [f64]),
    G: FnMut(f64, &[f64], usize) -> f64,
{
    if n.is_null() || time.is_null() || state.is_null() || root.is_null() {
        if !n.is_null() {
            unsafe { *n = 0 };
        }
        return 0.0;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        unsafe { *n = 0 };
        return 0.0;
    }
    let context = unsafe { &mut *pointer.cast::<RealContext<f64, F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Ok(root_index) = usize::try_from(unsafe { *root }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Some(root_index) = root_index
        .checked_sub(1)
        .filter(|index| *index < context.root_count)
    else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    if dimension != context.dimension || !unsafe { *time }.is_finite() {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    }
    let Some(root_callback) = context.root else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let callback = unsafe { &mut *root_callback };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, input, root_index)
    })) {
        Ok(value) if value.is_finite() => value,
        Ok(_) => {
            context.failure = Some(RealCallbackFailure::RootNonFinite(root_index));
            unsafe { *n = 0 };
            0.0
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RootPanicked);
            unsafe { *n = 0 };
            0.0
        }
    }
}

unsafe extern "C" fn unused_real_root_f32(
    _n: *mut FortranInteger,
    _time: *mut f32,
    _state: *mut f32,
    _root: *mut FortranInteger,
) -> f32 {
    0.0
}

unsafe extern "C" fn unused_real_root_f64(
    _n: *mut FortranInteger,
    _time: *mut f64,
    _state: *mut f64,
    _root: *mut FortranInteger,
) -> f64 {
    0.0
}

fn validate_complex(
    time: f32,
    state: &[SafeComplex32],
    tolerance: f32,
) -> Result<(), CallbackOdeError> {
    if state.is_empty() {
        return Err(OdeError::InvalidInput(OdeInputError::EmptyState));
    }
    if !time.is_finite() || state.iter().any(|value| !finite_complex(*value)) {
        return Err(OdeError::InvalidInput(OdeInputError::NonFiniteInput));
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(OdeError::InvalidInput(OdeInputError::InvalidTolerance));
    }
    native_integer(state.len())?;
    Ok(())
}

unsafe extern "C" fn complex_rhs<F, G>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut NativeComplex32,
    derivative: *mut NativeComplex32,
) where
    F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
{
    if n.is_null() || time.is_null() || state.is_null() || derivative.is_null() {
        if !n.is_null() {
            unsafe { *n = 0 };
        }
        return;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        unsafe { *n = 0 };
        return;
    }
    let context = unsafe { &mut *pointer.cast::<ComplexContext<F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    };
    if dimension != context.dimension
        || dimension == 0
        || ranges_overlap(state, derivative, dimension)
        || !unsafe { *time }.is_finite()
    {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return;
    }
    let native_input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let input = native_input
        .iter()
        .copied()
        .map(safe_complex)
        .collect::<Vec<_>>();
    let mut output = vec![SafeComplex32::new(0.0, 0.0); dimension];
    let callback = unsafe { &mut *context.rhs };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, &input, &mut output)
    })) {
        Ok(()) => {
            if let Some(index) = output.iter().position(|value| !finite_complex(*value)) {
                context.failure = Some(RealCallbackFailure::RhsNonFinite(index));
                unsafe { *n = 0 };
                return;
            }
            let native_output = unsafe { core::slice::from_raw_parts_mut(derivative, dimension) };
            for (destination, value) in native_output.iter_mut().zip(output) {
                *destination = native_complex(value);
            }
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RhsPanicked);
            unsafe { *n = 0 };
        }
    }
}

unsafe extern "C" fn complex_root<F, G>(
    n: *mut FortranInteger,
    time: *mut f32,
    state: *mut NativeComplex32,
    root: *mut FortranInteger,
) -> f32
where
    F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
    G: FnMut(f32, &[SafeComplex32], usize) -> f32,
{
    if n.is_null() || time.is_null() || state.is_null() || root.is_null() {
        if !n.is_null() {
            unsafe { *n = 0 };
        }
        return 0.0;
    }
    let pointer = ACTIVE_CONTEXT.with(|slot| slot.get());
    if pointer.is_null() {
        unsafe { *n = 0 };
        return 0.0;
    }
    let context = unsafe { &mut *pointer.cast::<ComplexContext<F, G>>() };
    let Ok(dimension) = usize::try_from(unsafe { *n }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Ok(root_index) = usize::try_from(unsafe { *root }) else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Some(root_index) = root_index
        .checked_sub(1)
        .filter(|index| *index < context.root_count)
    else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    let Some(root_callback) = context.root else {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    };
    if dimension != context.dimension || !unsafe { *time }.is_finite() {
        context.failure = Some(RealCallbackFailure::NativeContract);
        unsafe { *n = 0 };
        return 0.0;
    }
    let native_input = unsafe { core::slice::from_raw_parts(state, dimension) };
    let input = native_input
        .iter()
        .copied()
        .map(safe_complex)
        .collect::<Vec<_>>();
    let callback = unsafe { &mut *root_callback };
    match catch_unwind(AssertUnwindSafe(|| {
        callback(unsafe { *time }, &input, root_index)
    })) {
        Ok(value) if value.is_finite() => value,
        Ok(_) => {
            context.failure = Some(RealCallbackFailure::RootNonFinite(root_index));
            unsafe { *n = 0 };
            0.0
        }
        Err(_) => {
            context.failure = Some(RealCallbackFailure::RootPanicked);
            unsafe { *n = 0 };
            0.0
        }
    }
}

unsafe extern "C" fn unused_complex_root(
    _n: *mut FortranInteger,
    _time: *mut f32,
    _state: *mut NativeComplex32,
    _root: *mut FortranInteger,
) -> f32 {
    0.0
}

fn driv1_status(
    mstate: FortranInteger,
    ierflg: FortranInteger,
) -> Result<DrivStatus, CallbackOdeError> {
    match mstate {
        2 => Ok(DrivStatus::ReachedTarget),
        3 => Ok(DrivStatus::ExcessWork),
        4 => Ok(DrivStatus::ToleranceAdjusted),
        5 => Ok(DrivStatus::CallbackStopped),
        6 => Ok(DrivStatus::Interpolated),
        _ => Err(OdeError::NativeContractViolation {
            nstate: mstate,
            ierflg,
        }),
    }
}

fn driv2_status(
    mstate: FortranInteger,
    ierflg: FortranInteger,
    root_index: Option<usize>,
) -> Result<DrivStatus, CallbackOdeError> {
    match mstate {
        2 => Ok(DrivStatus::ReachedTarget),
        3 => Ok(DrivStatus::ExcessWork),
        4 => Ok(DrivStatus::ToleranceAdjusted),
        5 => root_index
            .map(|index| DrivStatus::RootFound { index })
            .ok_or(OdeError::NativeContractViolation {
                nstate: mstate,
                ierflg,
            }),
        6 | 7 => Ok(DrivStatus::CallbackStopped),
        8 => Ok(DrivStatus::Interpolated),
        _ => Err(OdeError::NativeContractViolation {
            nstate: mstate,
            ierflg,
        }),
    }
}

fn finish_lifecycle<T>(
    lifecycle: &mut Lifecycle,
    result: Result<DrivStepResult<T>, CallbackOdeError>,
) -> Result<DrivStepResult<T>, CallbackOdeError> {
    match result {
        Ok(result) => {
            *lifecycle = match result.status {
                DrivStatus::ReachedTarget
                | DrivStatus::ExcessWork
                | DrivStatus::ToleranceAdjusted
                | DrivStatus::RootFound { .. }
                | DrivStatus::Interpolated => Lifecycle::Recoverable,
                DrivStatus::CallbackStopped => Lifecycle::Failed,
            };
            Ok(result)
        }
        Err(error) => {
            *lifecycle = Lifecycle::Failed;
            Err(error)
        }
    }
}

impl Driv1Session<f32> {
    /// Creates an owned single-precision `SDRIV1` continuation session.
    pub fn new(
        initial_time: f32,
        initial_state: Vec<f32>,
        relative_tolerance: f32,
    ) -> Result<Self, CallbackOdeError> {
        validate_f32(initial_time, &initial_state, relative_tolerance)?;
        if initial_state.len() > 200 {
            return Err(OdeError::InvalidInput(
                OdeInputError::ConvenienceDriverDimension,
            ));
        }
        let work_length = workspace1_len(initial_state.len())?;
        native_integer(work_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            relative_tolerance,
            work: vec![0.0; work_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances the session to `target_time` using a scoped Rust RHS closure.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f32,
        mut rhs: F,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[f32], &mut [f32]),
    {
        validate_target_f32(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_driv1_f32(self, target_time, &mut rhs);
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current native time.
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Current owned solution state.
    #[must_use]
    pub fn state(&self) -> &[f32] {
        &self.state
    }

    /// Consumes the session and returns the latest solution vector.
    #[must_use]
    pub fn into_state(self) -> Vec<f32> {
        self.state
    }
}

impl Driv1Session<f64> {
    /// Creates an owned double-precision `DDRIV1` continuation session.
    pub fn new(
        initial_time: f64,
        initial_state: Vec<f64>,
        relative_tolerance: f64,
    ) -> Result<Self, CallbackOdeError> {
        validate_f64(initial_time, &initial_state, relative_tolerance)?;
        if initial_state.len() > 200 {
            return Err(OdeError::InvalidInput(
                OdeInputError::ConvenienceDriverDimension,
            ));
        }
        let work_length = workspace1_len(initial_state.len())?;
        native_integer(work_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            relative_tolerance,
            work: vec![0.0; work_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances the session to `target_time` using a scoped Rust RHS closure.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f64,
        mut rhs: F,
    ) -> Result<DrivStepResult<f64>, CallbackOdeError>
    where
        F: FnMut(f64, &[f64], &mut [f64]),
    {
        validate_target_f64(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_driv1_f64(self, target_time, &mut rhs);
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current native time.
    #[must_use]
    pub const fn time(&self) -> f64 {
        self.time
    }

    /// Current owned solution state.
    #[must_use]
    pub fn state(&self) -> &[f64] {
        &self.state
    }

    /// Consumes the session and returns the latest solution vector.
    #[must_use]
    pub fn into_state(self) -> Vec<f64> {
        self.state
    }
}

fn run_driv1_f32<F>(
    session: &mut Driv1Session<f32>,
    target_time: f32,
    rhs: &mut F,
) -> Result<DrivStepResult<f32>, CallbackOdeError>
where
    F: FnMut(f32, &[f32], &mut [f32]),
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let mut context = RealContext::<f32, F, ()> {
        rhs,
        root: None,
        dimension: session.state.len(),
        root_count: 0,
        failure: None,
        _scalar: PhantomData,
    };
    let pointer = (&mut context as *mut RealContext<f32, F, ()>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut length = native_integer(session.work.len())?;
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    // SAFETY: the session owns state and persistent workspace; the scoped
    // context supplies the exact reviewed `F(N,T,Y,YDOT)` callback ABI.
    unsafe {
        slatec_sys::ode::sdriv1(
            &mut n,
            &mut session.time,
            session.state.as_mut_ptr(),
            real_rhs_f32::<F, ()>,
            &mut target,
            &mut session.mstate,
            &mut session.relative_tolerance,
            session.work.as_mut_ptr(),
            &mut length,
            &mut ierflg,
        );
    }
    drop(guard);
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    Ok(DrivStepResult {
        time: session.time,
        status: driv1_status(session.mstate, ierflg)?,
    })
}

fn run_driv1_f64<F>(
    session: &mut Driv1Session<f64>,
    target_time: f64,
    rhs: &mut F,
) -> Result<DrivStepResult<f64>, CallbackOdeError>
where
    F: FnMut(f64, &[f64], &mut [f64]),
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let mut context = RealContext::<f64, F, ()> {
        rhs,
        root: None,
        dimension: session.state.len(),
        root_count: 0,
        failure: None,
        _scalar: PhantomData,
    };
    let pointer = (&mut context as *mut RealContext<f64, F, ()>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut length = native_integer(session.work.len())?;
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    // SAFETY: equivalent to the reviewed single-precision call above.
    unsafe {
        slatec_sys::ode::ddriv1(
            &mut n,
            &mut session.time,
            session.state.as_mut_ptr(),
            real_rhs_f64::<F, ()>,
            &mut target,
            &mut session.mstate,
            &mut session.relative_tolerance,
            session.work.as_mut_ptr(),
            &mut length,
            &mut ierflg,
        );
    }
    drop(guard);
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    Ok(DrivStepResult {
        time: session.time,
        status: driv1_status(session.mstate, ierflg)?,
    })
}

impl Driv2Session<f32> {
    /// Creates an owned single-precision `SDRIV2` continuation session.
    pub fn new(
        initial_time: f32,
        initial_state: Vec<f32>,
        options: Driv2Options<f32>,
    ) -> Result<Self, CallbackOdeError> {
        validate_f32(initial_time, &initial_state, options.relative_tolerance)?;
        if !options.error_weight.is_finite() || options.error_weight <= 0.0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidErrorWeight));
        }
        native_integer(options.root_count)
            .map_err(|_| OdeError::InvalidInput(OdeInputError::InvalidRootCount))?;
        let work_length = workspace2_len(initial_state.len(), options.root_count, options.method)?;
        let iwork_length = iwork2_len(initial_state.len(), options.method)?;
        native_integer(work_length)?;
        native_integer(iwork_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            options,
            work: vec![0.0; work_length],
            iwork: vec![0; iwork_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances a no-event session. Use [`Self::integrate_to_with_events`]
    /// when `root_count` is nonzero.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f32,
        rhs: F,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[f32], &mut [f32]),
    {
        if self.options.root_count != 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl::<F, fn(f32, &[f32], usize) -> f32>(target_time, rhs, None)
    }

    /// Advances an event-enabled session with a zero-based indexed root callback.
    pub fn integrate_to_with_events<F, G>(
        &mut self,
        target_time: f32,
        rhs: F,
        root: G,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[f32], &mut [f32]),
        G: FnMut(f32, &[f32], usize) -> f32,
    {
        if self.options.root_count == 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl(target_time, rhs, Some(root))
    }

    fn integrate_to_impl<F, G>(
        &mut self,
        target_time: f32,
        mut rhs: F,
        mut root: Option<G>,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[f32], &mut [f32]),
        G: FnMut(f32, &[f32], usize) -> f32,
    {
        validate_target_f32(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_driv2_f32(self, target_time, &mut rhs, root.as_mut());
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current native time.
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Current owned solution state.
    #[must_use]
    pub fn state(&self) -> &[f32] {
        &self.state
    }

    /// Consumes the session and returns the latest solution state.
    #[must_use]
    pub fn into_state(self) -> Vec<f32> {
        self.state
    }
}

impl Driv2Session<f64> {
    /// Creates an owned double-precision `DDRIV2` continuation session.
    pub fn new(
        initial_time: f64,
        initial_state: Vec<f64>,
        options: Driv2Options<f64>,
    ) -> Result<Self, CallbackOdeError> {
        validate_f64(initial_time, &initial_state, options.relative_tolerance)?;
        if !options.error_weight.is_finite() || options.error_weight <= 0.0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidErrorWeight));
        }
        native_integer(options.root_count)
            .map_err(|_| OdeError::InvalidInput(OdeInputError::InvalidRootCount))?;
        let work_length = workspace2_len(initial_state.len(), options.root_count, options.method)?;
        let iwork_length = iwork2_len(initial_state.len(), options.method)?;
        native_integer(work_length)?;
        native_integer(iwork_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            options,
            work: vec![0.0; work_length],
            iwork: vec![0; iwork_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances a no-event session. Use [`Self::integrate_to_with_events`]
    /// when `root_count` is nonzero.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f64,
        rhs: F,
    ) -> Result<DrivStepResult<f64>, CallbackOdeError>
    where
        F: FnMut(f64, &[f64], &mut [f64]),
    {
        if self.options.root_count != 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl::<F, fn(f64, &[f64], usize) -> f64>(target_time, rhs, None)
    }

    /// Advances an event-enabled session with a zero-based indexed root callback.
    pub fn integrate_to_with_events<F, G>(
        &mut self,
        target_time: f64,
        rhs: F,
        root: G,
    ) -> Result<DrivStepResult<f64>, CallbackOdeError>
    where
        F: FnMut(f64, &[f64], &mut [f64]),
        G: FnMut(f64, &[f64], usize) -> f64,
    {
        if self.options.root_count == 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl(target_time, rhs, Some(root))
    }

    fn integrate_to_impl<F, G>(
        &mut self,
        target_time: f64,
        mut rhs: F,
        mut root: Option<G>,
    ) -> Result<DrivStepResult<f64>, CallbackOdeError>
    where
        F: FnMut(f64, &[f64], &mut [f64]),
        G: FnMut(f64, &[f64], usize) -> f64,
    {
        validate_target_f64(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_driv2_f64(self, target_time, &mut rhs, root.as_mut());
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current native time.
    #[must_use]
    pub const fn time(&self) -> f64 {
        self.time
    }

    /// Current owned solution state.
    #[must_use]
    pub fn state(&self) -> &[f64] {
        &self.state
    }

    /// Consumes the session and returns the latest solution state.
    #[must_use]
    pub fn into_state(self) -> Vec<f64> {
        self.state
    }
}

fn native_root_index(
    iwork: &[FortranInteger],
    root_count: usize,
) -> Result<usize, CallbackOdeError> {
    let value = iwork
        .get(5)
        .copied()
        .ok_or(OdeError::NativeContractViolation {
            nstate: 5,
            ierflg: 0,
        })?;
    let index = usize::try_from(value)
        .ok()
        .and_then(|value| value.checked_sub(1))
        .filter(|index| *index < root_count)
        .ok_or(OdeError::NativeContractViolation {
            nstate: 5,
            ierflg: 0,
        })?;
    Ok(index)
}

fn run_driv2_f32<F, G>(
    session: &mut Driv2Session<f32>,
    target_time: f32,
    rhs: &mut F,
    root: Option<&mut G>,
) -> Result<DrivStepResult<f32>, CallbackOdeError>
where
    F: FnMut(f32, &[f32], &mut [f32]),
    G: FnMut(f32, &[f32], usize) -> f32,
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let root_pointer = root.map(|callback| callback as *mut G);
    let mut context = RealContext {
        rhs,
        root: root_pointer,
        dimension: session.state.len(),
        root_count: session.options.root_count,
        failure: None,
        _scalar: PhantomData::<f32>,
    };
    let pointer = (&mut context as *mut RealContext<f32, F, G>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut roots = native_integer(session.options.root_count)?;
    let mut work_length = native_integer(session.work.len())?;
    let mut integer_work_length = native_integer(session.iwork.len())?;
    let mut method = session.options.method.native();
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let root_callback = if root_pointer.is_some() {
        real_root_f32::<F, G>
    } else {
        unused_real_root_f32
    };
    // SAFETY: all state/workspace storage is owned by the session; the scoped
    // context provides the reviewed RHS and optional indexed-root callbacks.
    unsafe {
        slatec_sys::ode::sdriv2(
            &mut n,
            &mut session.time,
            session.state.as_mut_ptr(),
            real_rhs_f32::<F, G>,
            &mut target,
            &mut session.mstate,
            &mut roots,
            &mut session.options.relative_tolerance,
            &mut session.options.error_weight,
            &mut method,
            session.work.as_mut_ptr(),
            &mut work_length,
            session.iwork.as_mut_ptr(),
            &mut integer_work_length,
            root_callback,
            &mut ierflg,
        );
    }
    drop(guard);
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    let root_index = if session.mstate == 5 {
        Some(native_root_index(
            &session.iwork,
            session.options.root_count,
        )?)
    } else {
        None
    };
    Ok(DrivStepResult {
        time: session.time,
        status: driv2_status(session.mstate, ierflg, root_index)?,
    })
}

fn run_driv2_f64<F, G>(
    session: &mut Driv2Session<f64>,
    target_time: f64,
    rhs: &mut F,
    root: Option<&mut G>,
) -> Result<DrivStepResult<f64>, CallbackOdeError>
where
    F: FnMut(f64, &[f64], &mut [f64]),
    G: FnMut(f64, &[f64], usize) -> f64,
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let root_pointer = root.map(|callback| callback as *mut G);
    let mut context = RealContext {
        rhs,
        root: root_pointer,
        dimension: session.state.len(),
        root_count: session.options.root_count,
        failure: None,
        _scalar: PhantomData::<f64>,
    };
    let pointer = (&mut context as *mut RealContext<f64, F, G>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut roots = native_integer(session.options.root_count)?;
    let mut work_length = native_integer(session.work.len())?;
    let mut integer_work_length = native_integer(session.iwork.len())?;
    let mut method = session.options.method.native();
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let root_callback = if root_pointer.is_some() {
        real_root_f64::<F, G>
    } else {
        unused_real_root_f64
    };
    // SAFETY: equivalent reviewed double-precision DRIV2 invocation.
    unsafe {
        slatec_sys::ode::ddriv2(
            &mut n,
            &mut session.time,
            session.state.as_mut_ptr(),
            real_rhs_f64::<F, G>,
            &mut target,
            &mut session.mstate,
            &mut roots,
            &mut session.options.relative_tolerance,
            &mut session.options.error_weight,
            &mut method,
            session.work.as_mut_ptr(),
            &mut work_length,
            session.iwork.as_mut_ptr(),
            &mut integer_work_length,
            root_callback,
            &mut ierflg,
        );
    }
    drop(guard);
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    let root_index = if session.mstate == 5 {
        Some(native_root_index(
            &session.iwork,
            session.options.root_count,
        )?)
    } else {
        None
    };
    Ok(DrivStepResult {
        time: session.time,
        status: driv2_status(session.mstate, ierflg, root_index)?,
    })
}

impl ComplexDriv1Session {
    /// Creates an owned complex single-precision `CDRIV1` continuation session.
    pub fn new(
        initial_time: f32,
        initial_state: Vec<SafeComplex32>,
        relative_tolerance: f32,
    ) -> Result<Self, CallbackOdeError> {
        validate_complex(initial_time, &initial_state, relative_tolerance)?;
        if initial_state.len() > 200 {
            return Err(OdeError::InvalidInput(
                OdeInputError::ConvenienceDriverDimension,
            ));
        }
        let work_length = workspace1_len(initial_state.len())?;
        native_integer(work_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            relative_tolerance,
            work: vec![NativeComplex32::default(); work_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances the complex IVP with a scoped RHS closure.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f32,
        mut rhs: F,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
    {
        validate_target_f32(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_cdriv1(self, target_time, &mut rhs);
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current real independent variable.
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Current owned complex state.
    #[must_use]
    pub fn state(&self) -> &[SafeComplex32] {
        &self.state
    }

    /// Consumes the session and returns the latest complex state.
    #[must_use]
    pub fn into_state(self) -> Vec<SafeComplex32> {
        self.state
    }
}

impl ComplexDriv2Session {
    /// Creates an owned complex single-precision `CDRIV2` continuation session.
    pub fn new(
        initial_time: f32,
        initial_state: Vec<SafeComplex32>,
        options: Driv2Options<f32>,
    ) -> Result<Self, CallbackOdeError> {
        validate_complex(initial_time, &initial_state, options.relative_tolerance)?;
        if !options.error_weight.is_finite() || options.error_weight <= 0.0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidErrorWeight));
        }
        native_integer(options.root_count)
            .map_err(|_| OdeError::InvalidInput(OdeInputError::InvalidRootCount))?;
        let work_length = workspace2_len(initial_state.len(), options.root_count, options.method)?;
        let iwork_length = iwork2_len(initial_state.len(), options.method)?;
        native_integer(work_length)?;
        native_integer(iwork_length)?;
        Ok(Self {
            time: initial_time,
            state: initial_state,
            options,
            work: vec![NativeComplex32::default(); work_length],
            iwork: vec![0; iwork_length],
            mstate: 1,
            direction: None,
            lifecycle: Lifecycle::Ready,
        })
    }

    /// Advances a complex session with no events configured.
    pub fn integrate_to<F>(
        &mut self,
        target_time: f32,
        rhs: F,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
    {
        if self.options.root_count != 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl::<F, fn(f32, &[SafeComplex32], usize) -> f32>(target_time, rhs, None)
    }

    /// Advances an event-enabled complex session with an indexed real root callback.
    pub fn integrate_to_with_events<F, G>(
        &mut self,
        target_time: f32,
        rhs: F,
        root: G,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
        G: FnMut(f32, &[SafeComplex32], usize) -> f32,
    {
        if self.options.root_count == 0 {
            return Err(OdeError::InvalidInput(OdeInputError::InvalidRootCount));
        }
        self.integrate_to_impl(target_time, rhs, Some(root))
    }

    fn integrate_to_impl<F, G>(
        &mut self,
        target_time: f32,
        mut rhs: F,
        mut root: Option<G>,
    ) -> Result<DrivStepResult<f32>, CallbackOdeError>
    where
        F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
        G: FnMut(f32, &[SafeComplex32], usize) -> f32,
    {
        validate_target_f32(self.time, target_time, &mut self.direction, self.lifecycle)?;
        let result = run_cdriv2(self, target_time, &mut rhs, root.as_mut());
        finish_lifecycle(&mut self.lifecycle, result)
    }

    /// Current real independent variable.
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Current owned complex state.
    #[must_use]
    pub fn state(&self) -> &[SafeComplex32] {
        &self.state
    }

    /// Consumes the session and returns the latest complex state.
    #[must_use]
    pub fn into_state(self) -> Vec<SafeComplex32> {
        self.state
    }
}

fn run_cdriv1<F>(
    session: &mut ComplexDriv1Session,
    target_time: f32,
    rhs: &mut F,
) -> Result<DrivStepResult<f32>, CallbackOdeError>
where
    F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let mut context = ComplexContext::<F, ()> {
        rhs,
        root: None,
        dimension: session.state.len(),
        root_count: 0,
        failure: None,
    };
    let pointer = (&mut context as *mut ComplexContext<F, ()>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut native_state = session
        .state
        .iter()
        .copied()
        .map(native_complex)
        .collect::<Vec<_>>();
    let mut length = native_integer(session.work.len())?;
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    // SAFETY: the complex native state is an explicit owned conversion from
    // num-complex values; work storage and callback lifetime are session-owned.
    unsafe {
        slatec_sys::ode::cdriv1(
            &mut n,
            &mut session.time,
            native_state.as_mut_ptr(),
            complex_rhs::<F, ()>,
            &mut target,
            &mut session.mstate,
            &mut session.relative_tolerance,
            session.work.as_mut_ptr(),
            &mut length,
            &mut ierflg,
        );
    }
    drop(guard);
    session.state = native_state.into_iter().map(safe_complex).collect();
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    Ok(DrivStepResult {
        time: session.time,
        status: driv1_status(session.mstate, ierflg)?,
    })
}

fn run_cdriv2<F, G>(
    session: &mut ComplexDriv2Session,
    target_time: f32,
    rhs: &mut F,
    root: Option<&mut G>,
) -> Result<DrivStepResult<f32>, CallbackOdeError>
where
    F: FnMut(f32, &[SafeComplex32], &mut [SafeComplex32]),
    G: FnMut(f32, &[SafeComplex32], usize) -> f32,
{
    if ACTIVE_CONTEXT.with(|slot| !slot.get().is_null()) {
        return Err(OdeError::ReentrantCall);
    }
    let _runtime = crate::runtime::lock_native();
    let _shared = crate::callback_runtime::reserve_external_callback_context()
        .map_err(|_| OdeError::ReentrantCall)?;
    let root_pointer = root.map(|callback| callback as *mut G);
    let mut context = ComplexContext {
        rhs,
        root: root_pointer,
        dimension: session.state.len(),
        root_count: session.options.root_count,
        failure: None,
    };
    let pointer = (&mut context as *mut ComplexContext<F, G>).cast::<c_void>();
    let guard = ContextGuard::install(pointer).map_err(|_| OdeError::ReentrantCall)?;
    let mut n = native_integer(session.state.len())?;
    let mut target = target_time;
    let mut roots = native_integer(session.options.root_count)?;
    let mut native_state = session
        .state
        .iter()
        .copied()
        .map(native_complex)
        .collect::<Vec<_>>();
    let mut work_length = native_integer(session.work.len())?;
    let mut integer_work_length = native_integer(session.iwork.len())?;
    let mut method = session.options.method.native();
    let mut ierflg = 0;
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let root_callback = if root_pointer.is_some() {
        complex_root::<F, G>
    } else {
        unused_complex_root
    };
    // SAFETY: see run_cdriv1; CDRIV2 additionally receives session-owned
    // IWORK and a checked indexed-root callback when events are configured.
    unsafe {
        slatec_sys::ode::cdriv2(
            &mut n,
            &mut session.time,
            native_state.as_mut_ptr(),
            complex_rhs::<F, G>,
            &mut target,
            &mut session.mstate,
            &mut roots,
            &mut session.options.relative_tolerance,
            &mut session.options.error_weight,
            &mut method,
            session.work.as_mut_ptr(),
            &mut work_length,
            session.iwork.as_mut_ptr(),
            &mut integer_work_length,
            root_callback,
            &mut ierflg,
        );
    }
    drop(guard);
    session.state = native_state.into_iter().map(safe_complex).collect();
    finish_failure(context.failure)?;
    if ierflg != 0 {
        return Err(OdeError::NativeContractViolation {
            nstate: session.mstate,
            ierflg,
        });
    }
    let root_index = if session.mstate == 5 {
        Some(native_root_index(
            &session.iwork,
            session.options.root_count,
        )?)
    } else {
        None
    };
    Ok(DrivStepResult {
        time: session.time,
        status: driv2_status(session.mstate, ierflg, root_index)?,
    })
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::{
        Driv1Session, Driv2Options, DrivMethod, DrivStatus, driv1_status, driv2_status,
        workspace1_len, workspace2_len,
    };
    use crate::ode::{OdeError, OdeInputError};

    #[test]
    fn convenience_and_event_workspace_formulas_are_checked() {
        assert!(matches!(workspace1_len(2), Ok(326)));
        assert!(matches!(workspace2_len(2, 1, DrivMethod::Adams), Ok(284)));
        assert!(matches!(
            Driv1Session::<f64>::new(0.0, Vec::new(), 1.0),
            Err(OdeError::InvalidInput(OdeInputError::EmptyState))
        ));
        assert!(matches!(
            super::Driv2Session::<f64>::new(
                0.0,
                vec![1.0],
                Driv2Options {
                    error_weight: 0.0,
                    ..Driv2Options::default()
                },
            ),
            Err(OdeError::InvalidInput(OdeInputError::InvalidErrorWeight))
        ));
    }

    #[test]
    fn convenience_and_event_status_models_remain_distinct() {
        assert!(matches!(
            driv1_status(5, 0),
            Ok(DrivStatus::CallbackStopped)
        ));
        assert!(matches!(driv1_status(6, 0), Ok(DrivStatus::Interpolated)));
        assert!(matches!(
            driv2_status(5, 0, Some(0)),
            Ok(DrivStatus::RootFound { index: 0 })
        ));
        assert!(matches!(
            driv2_status(6, 0, None),
            Ok(DrivStatus::CallbackStopped)
        ));
        assert!(matches!(
            driv2_status(8, 0, None),
            Ok(DrivStatus::Interpolated)
        ));
    }
}
