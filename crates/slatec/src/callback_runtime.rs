//! Private scoped runtime for reviewed Fortran callbacks.
//!
//! The selected GNU Fortran scalar callback ABI has no user-data argument.
//! This module therefore owns a thread-local, lexical callback slot while the
//! process-wide native runtime lock is held. It is shared by quadrature,
//! scalar-root, and nonlinear adapters so callback-bearing families cannot
//! nest into each other accidentally.

#![allow(dead_code)] // Callback families use different halves of this bridge.

use alloc::boxed::Box;
use core::mem::size_of;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread_local;

/// Reviewed GNU Fortran callback shape for a double-precision scalar value.
pub(crate) type ScalarFnF64 = unsafe extern "C" fn(*const f64) -> f64;

/// Reviewed GNU Fortran callback shape for a single-precision scalar value.
pub(crate) type ScalarFnF32 = unsafe extern "C" fn(*const f32) -> f32;

/// Reviewed GNU Fortran callback shape used by `DNSQE`.
pub(crate) type VectorFnF64 = unsafe extern "C" fn(
    *const slatec_sys::FortranInteger,
    *const f64,
    *mut f64,
    *mut slatec_sys::FortranInteger,
);

/// Reviewed GNU Fortran callback shape used by `SNSQE`.
pub(crate) type VectorFnF32 = unsafe extern "C" fn(
    *const slatec_sys::FortranInteger,
    *const f32,
    *mut f32,
    *mut slatec_sys::FortranInteger,
);

/// Reviewed GNU Fortran Jacobian callback shape used by `DNSQ`.
pub(crate) type JacobianFnF64 = unsafe extern "C" fn(
    *const slatec_sys::FortranInteger,
    *const f64,
    *const f64,
    *mut f64,
    *const slatec_sys::FortranInteger,
    *mut slatec_sys::FortranInteger,
);

/// Reviewed GNU Fortran Jacobian callback shape used by `SNSQ`.
pub(crate) type JacobianFnF32 = unsafe extern "C" fn(
    *const slatec_sys::FortranInteger,
    *const f32,
    *const f32,
    *mut f32,
    *const slatec_sys::FortranInteger,
    *mut slatec_sys::FortranInteger,
);

/// Reviewed GNU Fortran residual callback shape used by `DNLS1E`.
pub(crate) type LeastSquaresFnF64 = unsafe extern "C" fn(
    *mut slatec_sys::FortranInteger,
    *const slatec_sys::FortranInteger,
    *const slatec_sys::FortranInteger,
    *const f64,
    *mut f64,
    *mut f64,
    *const slatec_sys::FortranInteger,
);

/// Reviewed GNU Fortran residual callback shape used by `SNLS1E`.
pub(crate) type LeastSquaresFnF32 = unsafe extern "C" fn(
    *mut slatec_sys::FortranInteger,
    *const slatec_sys::FortranInteger,
    *const slatec_sys::FortranInteger,
    *const f32,
    *mut f32,
    *mut f32,
    *const slatec_sys::FortranInteger,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallbackFailure {
    Panicked,
    NonFinite,
    InvalidPointer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallbackRuntimeError {
    NestedCallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VectorCallbackFailure {
    Panicked,
    NonFinite { index: usize },
    Cancelled,
    InvalidPointer,
    DimensionMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExpertCallbackFailure {
    ResidualPanicked,
    JacobianPanicked,
    ResidualNonFinite { index: usize },
    JacobianNonFinite { row: usize, column: usize },
    InvalidPointer,
    DimensionMismatch,
    InvalidLeadingDimension,
    UnexpectedFlag,
}

/// A contained failure from an `SNLS1E` or `DNLS1E` residual callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LeastSquaresCallbackFailure {
    /// The Rust residual closure panicked.
    Panicked,
    /// A residual component was NaN or infinite.
    NonFinite {
        /// Zero-based residual index.
        index: usize,
    },
    /// A required native pointer was null or input/output regions overlap.
    InvalidPointer,
    /// Native `M` or `N` did not match the registered callback dimensions.
    DimensionMismatch,
    /// The easy-driver callback was invoked with an unsupported `IFLAG`.
    UnexpectedFlag,
}

pub(crate) struct CallbackInvocation<R> {
    pub(crate) value: R,
    pub(crate) failure: Option<CallbackFailure>,
    pub(crate) evaluations: usize,
}

pub(crate) struct VectorCallbackInvocation<R> {
    pub(crate) value: R,
    pub(crate) failure: Option<VectorCallbackFailure>,
    pub(crate) evaluations: usize,
}

pub(crate) struct ExpertCallbackInvocation<R> {
    pub(crate) value: R,
    pub(crate) failure: Option<ExpertCallbackFailure>,
    pub(crate) residual_evaluations: usize,
    pub(crate) jacobian_evaluations: usize,
}

/// Result of one scoped nonlinear least-squares callback registration.
pub(crate) struct LeastSquaresCallbackInvocation<R> {
    /// Native-call return value.
    pub(crate) value: R,
    /// First contained callback failure, if any.
    pub(crate) failure: Option<LeastSquaresCallbackFailure>,
    /// Number of residual callback invocations.
    pub(crate) evaluations: usize,
}

#[derive(Clone, Copy)]
struct SlotF64 {
    data: *mut (),
    invoke: unsafe fn(*mut (), Option<f64>) -> f64,
}

#[derive(Clone, Copy)]
struct SlotF32 {
    data: *mut (),
    invoke: unsafe fn(*mut (), Option<f32>) -> f32,
}

#[derive(Clone, Copy)]
struct VectorSlotF64 {
    data: *mut (),
    invoke: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f64,
        *mut f64,
        *mut slatec_sys::FortranInteger,
    ),
}

#[derive(Clone, Copy)]
struct VectorSlotF32 {
    data: *mut (),
    invoke: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f32,
        *mut f32,
        *mut slatec_sys::FortranInteger,
    ),
}

#[derive(Clone, Copy)]
struct ExpertSlotF64 {
    data: *mut (),
    residual: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f64,
        *mut f64,
        *mut slatec_sys::FortranInteger,
    ),
    jacobian: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f64,
        *const f64,
        *mut f64,
        *const slatec_sys::FortranInteger,
        *mut slatec_sys::FortranInteger,
    ),
}

#[derive(Clone, Copy)]
struct ExpertSlotF32 {
    data: *mut (),
    residual: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f32,
        *mut f32,
        *mut slatec_sys::FortranInteger,
    ),
    jacobian: unsafe fn(
        *mut (),
        *const slatec_sys::FortranInteger,
        *const f32,
        *const f32,
        *mut f32,
        *const slatec_sys::FortranInteger,
        *mut slatec_sys::FortranInteger,
    ),
}

#[derive(Clone, Copy)]
struct LeastSquaresSlotF64 {
    data: *mut (),
    invoke: unsafe fn(
        *mut (),
        *mut slatec_sys::FortranInteger,
        *const slatec_sys::FortranInteger,
        *const slatec_sys::FortranInteger,
        *const f64,
        *mut f64,
        *mut f64,
        *const slatec_sys::FortranInteger,
    ),
}

#[derive(Clone, Copy)]
struct LeastSquaresSlotF32 {
    data: *mut (),
    invoke: unsafe fn(
        *mut (),
        *mut slatec_sys::FortranInteger,
        *const slatec_sys::FortranInteger,
        *const slatec_sys::FortranInteger,
        *const f32,
        *mut f32,
        *mut f32,
        *const slatec_sys::FortranInteger,
    ),
}

thread_local! {
    static ACTIVE_F64: Cell<Option<SlotF64>> = const { Cell::new(None) };
    static ACTIVE_F32: Cell<Option<SlotF32>> = const { Cell::new(None) };
    static ACTIVE_VECTOR_F64: Cell<Option<VectorSlotF64>> = const { Cell::new(None) };
    static ACTIVE_VECTOR_F32: Cell<Option<VectorSlotF32>> = const { Cell::new(None) };
    static ACTIVE_EXPERT_F64: Cell<Option<ExpertSlotF64>> = const { Cell::new(None) };
    static ACTIVE_EXPERT_F32: Cell<Option<ExpertSlotF32>> = const { Cell::new(None) };
    static ACTIVE_LEAST_SQUARES_F64: Cell<Option<LeastSquaresSlotF64>> = const { Cell::new(None) };
    static ACTIVE_LEAST_SQUARES_F32: Cell<Option<LeastSquaresSlotF32>> = const { Cell::new(None) };
}

struct CallbackState<F> {
    callback: F,
    failure: Option<CallbackFailure>,
    evaluations: usize,
}

struct VectorCallbackState<F> {
    callback: F,
    dimension: usize,
    failure: Option<VectorCallbackFailure>,
    evaluations: usize,
}

struct ExpertCallbackState<F, J> {
    residual: F,
    jacobian: J,
    dimension: usize,
    failure: Option<ExpertCallbackFailure>,
    residual_evaluations: usize,
    jacobian_evaluations: usize,
}

struct LeastSquaresCallbackState<F> {
    callback: F,
    residual_count: usize,
    parameter_count: usize,
    failure: Option<LeastSquaresCallbackFailure>,
    evaluations: usize,
}

struct SlotGuard {
    kind: CallbackKind,
}

enum CallbackKind {
    F32,
    F64,
    VectorF32,
    VectorF64,
    ExpertF32,
    ExpertF64,
    LeastSquaresF32,
    LeastSquaresF64,
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        match self.kind {
            CallbackKind::F32 => ACTIVE_F32.with(|slot| slot.set(None)),
            CallbackKind::F64 => ACTIVE_F64.with(|slot| slot.set(None)),
            CallbackKind::VectorF32 => ACTIVE_VECTOR_F32.with(|slot| slot.set(None)),
            CallbackKind::VectorF64 => ACTIVE_VECTOR_F64.with(|slot| slot.set(None)),
            CallbackKind::ExpertF32 => ACTIVE_EXPERT_F32.with(|slot| slot.set(None)),
            CallbackKind::ExpertF64 => ACTIVE_EXPERT_F64.with(|slot| slot.set(None)),
            CallbackKind::LeastSquaresF32 => ACTIVE_LEAST_SQUARES_F32.with(|slot| slot.set(None)),
            CallbackKind::LeastSquaresF64 => ACTIVE_LEAST_SQUARES_F64.with(|slot| slot.set(None)),
        }
    }
}

/// Returns whether any reviewed scalar callback is active on this thread.
pub(crate) fn is_active() -> bool {
    ACTIVE_F64.with(|slot| slot.get().is_some())
        || ACTIVE_F32.with(|slot| slot.get().is_some())
        || ACTIVE_VECTOR_F64.with(|slot| slot.get().is_some())
        || ACTIVE_VECTOR_F32.with(|slot| slot.get().is_some())
        || ACTIVE_EXPERT_F64.with(|slot| slot.get().is_some())
        || ACTIVE_EXPERT_F32.with(|slot| slot.get().is_some())
        || ACTIVE_LEAST_SQUARES_F64.with(|slot| slot.get().is_some())
        || ACTIVE_LEAST_SQUARES_F32.with(|slot| slot.get().is_some())
}

unsafe fn invoke_f64<F>(data: *mut (), value: Option<f64>) -> f64
where
    F: FnMut(f64) -> f64,
{
    // Safety: the scoped installer supplies a pointer to CallbackState<F> and
    // clears the slot before that boxed state is dropped.
    let state = unsafe { &mut *data.cast::<CallbackState<F>>() };
    if state.failure.is_some() {
        return 0.0;
    }
    let Some(value) = value else {
        state.failure = Some(CallbackFailure::InvalidPointer);
        return 0.0;
    };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(value))) {
        Ok(result) if result.is_finite() => result,
        Ok(_) => {
            state.failure = Some(CallbackFailure::NonFinite);
            0.0
        }
        Err(_) => {
            state.failure = Some(CallbackFailure::Panicked);
            0.0
        }
    }
}

unsafe fn invoke_f32<F>(data: *mut (), value: Option<f32>) -> f32
where
    F: FnMut(f32) -> f32,
{
    // Safety: see invoke_f64; the slot and boxed state have the same scope.
    let state = unsafe { &mut *data.cast::<CallbackState<F>>() };
    if state.failure.is_some() {
        return 0.0;
    }
    let Some(value) = value else {
        state.failure = Some(CallbackFailure::InvalidPointer);
        return 0.0;
    };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(value))) {
        Ok(result) if result.is_finite() => result,
        Ok(_) => {
            state.failure = Some(CallbackFailure::NonFinite);
            0.0
        }
        Err(_) => {
            state.failure = Some(CallbackFailure::Panicked);
            0.0
        }
    }
}

unsafe fn failed_f64<F>(data: *const ()) -> bool
where
    F: FnMut(f64) -> f64,
{
    // Safety: the handle is scoped to the boxed state installed by with_f64.
    unsafe { (*data.cast::<CallbackState<F>>()).failure.is_some() }
}

unsafe fn failed_f32<F>(data: *const ()) -> bool
where
    F: FnMut(f32) -> f32,
{
    // Safety: the handle is scoped to the boxed state installed by with_f32.
    unsafe { (*data.cast::<CallbackState<F>>()).failure.is_some() }
}

unsafe extern "C" fn trampoline_f64(value: *const f64) -> f64 {
    ACTIVE_F64.with(|slot| match slot.get() {
        Some(slot) => {
            // Safety: the pointer is read only for the duration of the native
            // callback and passed to the matching scoped dispatcher.
            let value = unsafe { value.as_ref().copied() };
            unsafe { (slot.invoke)(slot.data, value) }
        }
        None => 0.0,
    })
}

unsafe extern "C" fn trampoline_f32(value: *const f32) -> f32 {
    ACTIVE_F32.with(|slot| match slot.get() {
        Some(slot) => {
            // Safety: see trampoline_f64.
            let value = unsafe { value.as_ref().copied() };
            unsafe { (slot.invoke)(slot.data, value) }
        }
        None => 0.0,
    })
}

fn ranges_overlap<T>(left: *const T, right: *const T, len: usize) -> bool {
    let Some(bytes) = len.checked_mul(size_of::<T>()) else {
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

fn ranges_overlap_with_lengths<T>(
    left: *const T,
    left_len: usize,
    right: *const T,
    right_len: usize,
) -> bool {
    let Some(left_bytes) = left_len.checked_mul(size_of::<T>()) else {
        return true;
    };
    let Some(right_bytes) = right_len.checked_mul(size_of::<T>()) else {
        return true;
    };
    let Some(left_end) = (left as usize).checked_add(left_bytes) else {
        return true;
    };
    let Some(right_end) = (right as usize).checked_add(right_bytes) else {
        return true;
    };
    (left as usize) < right_end && (right as usize) < left_end
}

unsafe fn invoke_vector_f64<F>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    _iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f64], &mut [f64]) -> bool,
{
    // Safety: the scoped installer supplies a pointer to VectorCallbackState<F>
    // and clears the TLS slot before its boxed state is dropped.
    let state = unsafe { &mut *data.cast::<VectorCallbackState<F>>() };
    if state.failure.is_some() {
        return;
    }
    // Safety: a non-null N pointer is part of the reviewed callback ABI.
    let Some(native_n) = (unsafe { n.as_ref() }).copied() else {
        state.failure = Some(VectorCallbackFailure::InvalidPointer);
        return;
    };
    let Ok(native_n) = usize::try_from(native_n) else {
        state.failure = Some(VectorCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 || x.is_null() || fvec.is_null() {
        state.failure = Some(if x.is_null() || fvec.is_null() {
            VectorCallbackFailure::InvalidPointer
        } else {
            VectorCallbackFailure::DimensionMismatch
        });
        return;
    }
    if ranges_overlap(x, fvec.cast_const(), native_n) {
        state.failure = Some(VectorCallbackFailure::InvalidPointer);
        return;
    }

    // Safety: pointer validity, equal dimensions, and non-overlap were
    // checked above. The native callback owns both regions for this call.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    // Safety: see above; FNVEC is an output slice of exactly N elements.
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_n) };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(input, output))) {
        Ok(true) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(VectorCallbackFailure::NonFinite { index });
                output.fill(0.0);
            }
        }
        Ok(false) => {
            state.failure = Some(VectorCallbackFailure::Cancelled);
            output.fill(0.0);
        }
        Err(_) => {
            state.failure = Some(VectorCallbackFailure::Panicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_vector_f32<F>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    _iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f32], &mut [f32]) -> bool,
{
    // Safety: equivalent to invoke_vector_f64 for the single-precision ABI.
    let state = unsafe { &mut *data.cast::<VectorCallbackState<F>>() };
    if state.failure.is_some() {
        return;
    }
    // Safety: a non-null N pointer is part of the reviewed callback ABI.
    let Some(native_n) = (unsafe { n.as_ref() }).copied() else {
        state.failure = Some(VectorCallbackFailure::InvalidPointer);
        return;
    };
    let Ok(native_n) = usize::try_from(native_n) else {
        state.failure = Some(VectorCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 || x.is_null() || fvec.is_null() {
        state.failure = Some(if x.is_null() || fvec.is_null() {
            VectorCallbackFailure::InvalidPointer
        } else {
            VectorCallbackFailure::DimensionMismatch
        });
        return;
    }
    if ranges_overlap(x, fvec.cast_const(), native_n) {
        state.failure = Some(VectorCallbackFailure::InvalidPointer);
        return;
    }

    // Safety: pointer validity, equal dimensions, and non-overlap were
    // checked above. The native callback owns both regions for this call.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    // Safety: see above; FNVEC is an output slice of exactly N elements.
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_n) };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(input, output))) {
        Ok(true) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(VectorCallbackFailure::NonFinite { index });
                output.fill(0.0);
            }
        }
        Ok(false) => {
            state.failure = Some(VectorCallbackFailure::Cancelled);
            output.fill(0.0);
        }
        Err(_) => {
            state.failure = Some(VectorCallbackFailure::Panicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_least_squares_f64<F>(
    data: *mut (),
    iflag: *mut slatec_sys::FortranInteger,
    m: *const slatec_sys::FortranInteger,
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    _fjac: *mut f64,
    _ldfjac: *const slatec_sys::FortranInteger,
) where
    F: FnMut(&[f64], &mut [f64]),
{
    // Safety: this dispatcher is installed only by with_least_squares_f64 and
    // the matching slot is cleared before the boxed state is dropped.
    let state = unsafe { &mut *data.cast::<LeastSquaresCallbackState<F>>() };
    if state.failure.is_some() {
        // Do not invoke Rust again after a contained failure. When the native
        // callback supplies the already-registered residual extent, preserve
        // a finite sentinel for a finite-difference workspace callback too.
        // An invalid pointer or dimension is never dereferenced here.
        if let Some(length) = unsafe { m.as_ref() }
            .copied()
            .and_then(|value| usize::try_from(value).ok())
            .filter(|length| *length == state.residual_count)
        {
            if !fvec.is_null() {
                // SAFETY: `length` equals the registered residual count and
                // the native callback supplied a non-null output pointer.
                unsafe { core::slice::from_raw_parts_mut(fvec, length) }.fill(0.0);
            }
        }
        return;
    }
    let (Some(flag), Some(native_m), Some(native_n)) = (
        (unsafe { iflag.as_ref() }).copied(),
        (unsafe { m.as_ref() }).copied(),
        (unsafe { n.as_ref() }).copied(),
    ) else {
        state.failure = Some(LeastSquaresCallbackFailure::InvalidPointer);
        return;
    };
    let (Ok(native_m), Ok(native_n)) = (usize::try_from(native_m), usize::try_from(native_n))
    else {
        state.failure = Some(LeastSquaresCallbackFailure::DimensionMismatch);
        return;
    };
    if flag != 1 {
        state.failure = Some(LeastSquaresCallbackFailure::UnexpectedFlag);
        return;
    }
    if native_m != state.residual_count
        || native_n != state.parameter_count
        || native_m == 0
        || native_n == 0
    {
        state.failure = Some(LeastSquaresCallbackFailure::DimensionMismatch);
        return;
    }
    if x.is_null()
        || fvec.is_null()
        || ranges_overlap_with_lengths(x, native_n, fvec.cast_const(), native_m)
    {
        state.failure = Some(LeastSquaresCallbackFailure::InvalidPointer);
        return;
    }
    // Safety: dimensions match the registered context, both ranges are
    // non-null, and the input and mutable residual regions do not overlap.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_m) };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(input, output))) {
        Ok(()) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(LeastSquaresCallbackFailure::NonFinite { index });
                output.fill(0.0);
            }
        }
        Err(_) => {
            state.failure = Some(LeastSquaresCallbackFailure::Panicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_least_squares_f32<F>(
    data: *mut (),
    iflag: *mut slatec_sys::FortranInteger,
    m: *const slatec_sys::FortranInteger,
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    _fjac: *mut f32,
    _ldfjac: *const slatec_sys::FortranInteger,
) where
    F: FnMut(&[f32], &mut [f32]),
{
    // Safety: equivalent to invoke_least_squares_f64 for f32 storage.
    let state = unsafe { &mut *data.cast::<LeastSquaresCallbackState<F>>() };
    if state.failure.is_some() {
        // See invoke_least_squares_f64: preserve a finite sentinel without
        // re-entering Rust when a finite-difference workspace is requested.
        if let Some(length) = unsafe { m.as_ref() }
            .copied()
            .and_then(|value| usize::try_from(value).ok())
            .filter(|length| *length == state.residual_count)
        {
            if !fvec.is_null() {
                // SAFETY: checked registered residual extent and non-null
                // Fortran output pointer; see the f64 dispatcher.
                unsafe { core::slice::from_raw_parts_mut(fvec, length) }.fill(0.0);
            }
        }
        return;
    }
    let (Some(flag), Some(native_m), Some(native_n)) = (
        (unsafe { iflag.as_ref() }).copied(),
        (unsafe { m.as_ref() }).copied(),
        (unsafe { n.as_ref() }).copied(),
    ) else {
        state.failure = Some(LeastSquaresCallbackFailure::InvalidPointer);
        return;
    };
    let (Ok(native_m), Ok(native_n)) = (usize::try_from(native_m), usize::try_from(native_n))
    else {
        state.failure = Some(LeastSquaresCallbackFailure::DimensionMismatch);
        return;
    };
    if flag != 1 {
        state.failure = Some(LeastSquaresCallbackFailure::UnexpectedFlag);
        return;
    }
    if native_m != state.residual_count
        || native_n != state.parameter_count
        || native_m == 0
        || native_n == 0
    {
        state.failure = Some(LeastSquaresCallbackFailure::DimensionMismatch);
        return;
    }
    if x.is_null()
        || fvec.is_null()
        || ranges_overlap_with_lengths(x, native_n, fvec.cast_const(), native_m)
    {
        state.failure = Some(LeastSquaresCallbackFailure::InvalidPointer);
        return;
    }
    // Safety: see invoke_least_squares_f64.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_m) };
    state.evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.callback)(input, output))) {
        Ok(()) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(LeastSquaresCallbackFailure::NonFinite { index });
                output.fill(0.0);
            }
        }
        Err(_) => {
            state.failure = Some(LeastSquaresCallbackFailure::Panicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_expert_residual_f64<F, J>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], &mut [f64], usize),
{
    // Safety: the scoped installer owns this exact state until the native call
    // returns and clears the matching thread-local slot before dropping it.
    let state = unsafe { &mut *data.cast::<ExpertCallbackState<F, J>>() };
    if state.failure.is_some() {
        return;
    }
    let (Some(native_n), Some(flag)) = (
        (unsafe { n.as_ref() }).copied(),
        (unsafe { iflag.as_ref() }).copied(),
    ) else {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    };
    let Ok(native_n) = usize::try_from(native_n) else {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    }
    if x.is_null() || fvec.is_null() || ranges_overlap(x, fvec.cast_const(), native_n) {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    }
    if !matches!(flag, 1 | 2) {
        state.failure = Some(ExpertCallbackFailure::UnexpectedFlag);
        return;
    }
    // Safety: the checked native dimension matches the registered dimension;
    // both regions are non-null and do not overlap.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_n) };
    state.residual_evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.residual)(input, output))) {
        Ok(()) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(ExpertCallbackFailure::ResidualNonFinite { index });
                output.fill(0.0);
            }
        }
        Err(_) => {
            state.failure = Some(ExpertCallbackFailure::ResidualPanicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_expert_jacobian_f64<F, J>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *const f64,
    fjac: *mut f64,
    ldfjac: *const slatec_sys::FortranInteger,
    iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], &mut [f64], usize),
{
    // Safety: see invoke_expert_residual_f64.
    let state = unsafe { &mut *data.cast::<ExpertCallbackState<F, J>>() };
    if state.failure.is_some() {
        return;
    }
    let (Some(native_n), Some(native_ld), Some(flag)) = (
        (unsafe { n.as_ref() }).copied(),
        (unsafe { ldfjac.as_ref() }).copied(),
        (unsafe { iflag.as_ref() }).copied(),
    ) else {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    };
    let (Ok(native_n), Ok(native_ld)) = (usize::try_from(native_n), usize::try_from(native_ld))
    else {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    }
    if native_ld < native_n {
        state.failure = Some(ExpertCallbackFailure::InvalidLeadingDimension);
        return;
    }
    if flag <= 0 {
        state.failure = Some(ExpertCallbackFailure::UnexpectedFlag);
        return;
    }
    let Some(matrix_len) = native_ld.checked_mul(native_n) else {
        state.failure = Some(ExpertCallbackFailure::InvalidLeadingDimension);
        return;
    };
    if x.is_null()
        || fvec.is_null()
        || fjac.is_null()
        || ranges_overlap(x, fvec, native_n)
        || ranges_overlap_with_lengths(x, native_n, fjac.cast_const(), matrix_len)
        || ranges_overlap_with_lengths(fvec, native_n, fjac.cast_const(), matrix_len)
    {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    }
    // Safety: dimensions, physical lengths, pointer non-nullness, and pairwise
    // non-overlap have been checked above.
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let residual = unsafe { core::slice::from_raw_parts(fvec, native_n) };
    let matrix = unsafe { core::slice::from_raw_parts_mut(fjac, matrix_len) };
    for column in 0..native_n {
        for row in 0..native_n {
            matrix[row + column * native_ld] = f64::NAN;
        }
    }
    state.jacobian_evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| {
        (state.jacobian)(input, residual, matrix, native_ld);
    })) {
        Ok(()) => {
            for column in 0..native_n {
                for row in 0..native_n {
                    if !matrix[row + column * native_ld].is_finite() {
                        state.failure =
                            Some(ExpertCallbackFailure::JacobianNonFinite { row, column });
                        matrix.fill(0.0);
                        return;
                    }
                }
            }
        }
        Err(_) => {
            state.failure = Some(ExpertCallbackFailure::JacobianPanicked);
            matrix.fill(0.0);
        }
    }
}

unsafe fn invoke_expert_residual_f32<F, J>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], &mut [f32], usize),
{
    // Safety: equivalent to invoke_expert_residual_f64 for f32 storage.
    let state = unsafe { &mut *data.cast::<ExpertCallbackState<F, J>>() };
    if state.failure.is_some() {
        return;
    }
    let (Some(native_n), Some(flag)) = (
        (unsafe { n.as_ref() }).copied(),
        (unsafe { iflag.as_ref() }).copied(),
    ) else {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    };
    let Ok(native_n) = usize::try_from(native_n) else {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    }
    if x.is_null() || fvec.is_null() || ranges_overlap(x, fvec.cast_const(), native_n) {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    }
    if !matches!(flag, 1 | 2) {
        state.failure = Some(ExpertCallbackFailure::UnexpectedFlag);
        return;
    }
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let output = unsafe { core::slice::from_raw_parts_mut(fvec, native_n) };
    state.residual_evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| (state.residual)(input, output))) {
        Ok(()) => {
            if let Some((index, _)) = output
                .iter()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                state.failure = Some(ExpertCallbackFailure::ResidualNonFinite { index });
                output.fill(0.0);
            }
        }
        Err(_) => {
            state.failure = Some(ExpertCallbackFailure::ResidualPanicked);
            output.fill(0.0);
        }
    }
}

unsafe fn invoke_expert_jacobian_f32<F, J>(
    data: *mut (),
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *const f32,
    fjac: *mut f32,
    ldfjac: *const slatec_sys::FortranInteger,
    iflag: *mut slatec_sys::FortranInteger,
) where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], &mut [f32], usize),
{
    // Safety: equivalent to invoke_expert_jacobian_f64 for f32 storage.
    let state = unsafe { &mut *data.cast::<ExpertCallbackState<F, J>>() };
    if state.failure.is_some() {
        return;
    }
    let (Some(native_n), Some(native_ld), Some(flag)) = (
        (unsafe { n.as_ref() }).copied(),
        (unsafe { ldfjac.as_ref() }).copied(),
        (unsafe { iflag.as_ref() }).copied(),
    ) else {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    };
    let (Ok(native_n), Ok(native_ld)) = (usize::try_from(native_n), usize::try_from(native_ld))
    else {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    };
    if native_n != state.dimension || native_n == 0 {
        state.failure = Some(ExpertCallbackFailure::DimensionMismatch);
        return;
    }
    if native_ld < native_n {
        state.failure = Some(ExpertCallbackFailure::InvalidLeadingDimension);
        return;
    }
    if flag <= 0 {
        state.failure = Some(ExpertCallbackFailure::UnexpectedFlag);
        return;
    }
    let Some(matrix_len) = native_ld.checked_mul(native_n) else {
        state.failure = Some(ExpertCallbackFailure::InvalidLeadingDimension);
        return;
    };
    if x.is_null()
        || fvec.is_null()
        || fjac.is_null()
        || ranges_overlap(x, fvec, native_n)
        || ranges_overlap_with_lengths(x, native_n, fjac.cast_const(), matrix_len)
        || ranges_overlap_with_lengths(fvec, native_n, fjac.cast_const(), matrix_len)
    {
        state.failure = Some(ExpertCallbackFailure::InvalidPointer);
        return;
    }
    let input = unsafe { core::slice::from_raw_parts(x, native_n) };
    let residual = unsafe { core::slice::from_raw_parts(fvec, native_n) };
    let matrix = unsafe { core::slice::from_raw_parts_mut(fjac, matrix_len) };
    for column in 0..native_n {
        for row in 0..native_n {
            matrix[row + column * native_ld] = f32::NAN;
        }
    }
    state.jacobian_evaluations += 1;
    match catch_unwind(AssertUnwindSafe(|| {
        (state.jacobian)(input, residual, matrix, native_ld);
    })) {
        Ok(()) => {
            for column in 0..native_n {
                for row in 0..native_n {
                    if !matrix[row + column * native_ld].is_finite() {
                        state.failure =
                            Some(ExpertCallbackFailure::JacobianNonFinite { row, column });
                        matrix.fill(0.0);
                        return;
                    }
                }
            }
        }
        Err(_) => {
            state.failure = Some(ExpertCallbackFailure::JacobianPanicked);
            matrix.fill(0.0);
        }
    }
}

unsafe extern "C" fn trampoline_vector_f64(
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_VECTOR_F64.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: the thread-local slot is installed for this lexical
            // native call and dispatches to its matching state type.
            unsafe { (slot.invoke)(slot.data, n, x, fvec, iflag) };
        }
    });
}

unsafe extern "C" fn trampoline_vector_f32(
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_VECTOR_F32.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: see trampoline_vector_f64.
            unsafe { (slot.invoke)(slot.data, n, x, fvec, iflag) };
        }
    });
}

unsafe extern "C" fn trampoline_least_squares_f64(
    iflag: *mut slatec_sys::FortranInteger,
    m: *const slatec_sys::FortranInteger,
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    fjac: *mut f64,
    ldfjac: *const slatec_sys::FortranInteger,
) {
    ACTIVE_LEAST_SQUARES_F64.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: the scoped least-squares slot owns the matching state
            // throughout this native callback invocation.
            unsafe { (slot.invoke)(slot.data, iflag, m, n, x, fvec, fjac, ldfjac) };
        }
    });
}

unsafe extern "C" fn trampoline_least_squares_f32(
    iflag: *mut slatec_sys::FortranInteger,
    m: *const slatec_sys::FortranInteger,
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    fjac: *mut f32,
    ldfjac: *const slatec_sys::FortranInteger,
) {
    ACTIVE_LEAST_SQUARES_F32.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: equivalent to the f64 least-squares trampoline.
            unsafe { (slot.invoke)(slot.data, iflag, m, n, x, fvec, fjac, ldfjac) };
        }
    });
}

unsafe extern "C" fn trampoline_expert_residual_f64(
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *mut f64,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_EXPERT_F64.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: the matching expert slot is installed only for the
            // lexical native call that owns its state.
            unsafe { (slot.residual)(slot.data, n, x, fvec, iflag) };
        }
    });
}

unsafe extern "C" fn trampoline_expert_jacobian_f64(
    n: *const slatec_sys::FortranInteger,
    x: *const f64,
    fvec: *const f64,
    fjac: *mut f64,
    ldfjac: *const slatec_sys::FortranInteger,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_EXPERT_F64.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: see trampoline_expert_residual_f64.
            unsafe { (slot.jacobian)(slot.data, n, x, fvec, fjac, ldfjac, iflag) };
        }
    });
}

unsafe extern "C" fn trampoline_expert_residual_f32(
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *mut f32,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_EXPERT_F32.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: equivalent to the f64 expert trampoline.
            unsafe { (slot.residual)(slot.data, n, x, fvec, iflag) };
        }
    });
}

unsafe extern "C" fn trampoline_expert_jacobian_f32(
    n: *const slatec_sys::FortranInteger,
    x: *const f32,
    fvec: *const f32,
    fjac: *mut f32,
    ldfjac: *const slatec_sys::FortranInteger,
    iflag: *mut slatec_sys::FortranInteger,
) {
    ACTIVE_EXPERT_F32.with(|slot| {
        if let Some(slot) = slot.get() {
            // Safety: equivalent to the f64 expert trampoline.
            unsafe { (slot.jacobian)(slot.data, n, x, fvec, fjac, ldfjac, iflag) };
        }
    });
}

#[derive(Clone, Copy)]
pub(crate) struct F64Callback {
    callback: ScalarFnF64,
    state: *const (),
    failed: unsafe fn(*const ()) -> bool,
}

impl F64Callback {
    pub(crate) fn call(self, value: f64) -> f64 {
        // Safety: the trampoline is installed for this lexical native-call
        // scope and receives a valid pointer to the scalar value.
        unsafe { (self.callback)(&value) }
    }

    pub(crate) fn failed(self) -> bool {
        // Safety: this handle cannot outlive the state installed by with_f64.
        unsafe { (self.failed)(self.state) }
    }

    pub(crate) const fn ffi(self) -> ScalarFnF64 {
        self.callback
    }
}

#[derive(Clone, Copy)]
pub(crate) struct F32Callback {
    callback: ScalarFnF32,
    state: *const (),
    failed: unsafe fn(*const ()) -> bool,
}

impl F32Callback {
    pub(crate) fn call(self, value: f32) -> f32 {
        // Safety: see F64Callback::call.
        unsafe { (self.callback)(&value) }
    }

    pub(crate) fn failed(self) -> bool {
        // Safety: this handle cannot outlive the state installed by with_f32.
        unsafe { (self.failed)(self.state) }
    }

    pub(crate) const fn ffi(self) -> ScalarFnF32 {
        self.callback
    }
}

/// Scoped vector callback handle for the reviewed `DNSQE` ABI.
#[derive(Clone, Copy)]
pub(crate) struct VectorF64Callback {
    callback: VectorFnF64,
}

impl VectorF64Callback {
    pub(crate) const fn ffi(self) -> VectorFnF64 {
        self.callback
    }
}

/// Scoped vector callback handle for the reviewed `SNSQE` ABI.
#[derive(Clone, Copy)]
pub(crate) struct VectorF32Callback {
    callback: VectorFnF32,
}

impl VectorF32Callback {
    pub(crate) const fn ffi(self) -> VectorFnF32 {
        self.callback
    }
}

/// Scoped callback handle for the reviewed `DNLS1E` residual ABI.
#[derive(Clone, Copy)]
pub(crate) struct LeastSquaresF64Callback {
    callback: LeastSquaresFnF64,
}

impl LeastSquaresF64Callback {
    pub(crate) const fn ffi(self) -> LeastSquaresFnF64 {
        self.callback
    }
}

/// Scoped callback handle for the reviewed `SNLS1E` residual ABI.
#[derive(Clone, Copy)]
pub(crate) struct LeastSquaresF32Callback {
    callback: LeastSquaresFnF32,
}

impl LeastSquaresF32Callback {
    pub(crate) const fn ffi(self) -> LeastSquaresFnF32 {
        self.callback
    }
}

/// Scoped callback pair for the reviewed double-precision `DNSQ` ABI.
#[derive(Clone, Copy)]
pub(crate) struct ExpertF64Callbacks {
    residual: VectorFnF64,
    jacobian: JacobianFnF64,
}

impl ExpertF64Callbacks {
    pub(crate) const fn residual(self) -> VectorFnF64 {
        self.residual
    }

    pub(crate) const fn jacobian(self) -> JacobianFnF64 {
        self.jacobian
    }
}

/// Scoped callback pair for the reviewed single-precision `SNSQ` ABI.
#[derive(Clone, Copy)]
pub(crate) struct ExpertF32Callbacks {
    residual: VectorFnF32,
    jacobian: JacobianFnF32,
}

impl ExpertF32Callbacks {
    pub(crate) const fn residual(self) -> VectorFnF32 {
        self.residual
    }

    pub(crate) const fn jacobian(self) -> JacobianFnF32 {
        self.jacobian
    }
}

pub(crate) fn with_f64<F, R>(
    callback: F,
    native_call: impl FnOnce(F64Callback) -> R,
) -> Result<CallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(f64) -> f64,
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(CallbackState {
        callback,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut CallbackState<F>).cast();
    ACTIVE_F64.with(|slot| {
        slot.set(Some(SlotF64 {
            data,
            invoke: invoke_f64::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::F64,
    };
    let value = native_call(F64Callback {
        callback: trampoline_f64,
        state: data.cast_const(),
        failed: failed_f64::<F>,
    });
    drop(slot_guard);
    Ok(CallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_f32<F, R>(
    callback: F,
    native_call: impl FnOnce(F32Callback) -> R,
) -> Result<CallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(f32) -> f32,
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(CallbackState {
        callback,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut CallbackState<F>).cast();
    ACTIVE_F32.with(|slot| {
        slot.set(Some(SlotF32 {
            data,
            invoke: invoke_f32::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::F32,
    };
    let value = native_call(F32Callback {
        callback: trampoline_f32,
        state: data.cast_const(),
        failed: failed_f32::<F>,
    });
    drop(slot_guard);
    Ok(CallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_vector_f64<F, R>(
    dimension: usize,
    callback: F,
    native_call: impl FnOnce(VectorF64Callback) -> R,
) -> Result<VectorCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f64], &mut [f64]) -> bool,
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(VectorCallbackState {
        callback,
        dimension,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut VectorCallbackState<F>).cast();
    ACTIVE_VECTOR_F64.with(|slot| {
        slot.set(Some(VectorSlotF64 {
            data,
            invoke: invoke_vector_f64::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::VectorF64,
    };
    let value = native_call(VectorF64Callback {
        callback: trampoline_vector_f64,
    });
    drop(slot_guard);
    Ok(VectorCallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_vector_f32<F, R>(
    dimension: usize,
    callback: F,
    native_call: impl FnOnce(VectorF32Callback) -> R,
) -> Result<VectorCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f32], &mut [f32]) -> bool,
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(VectorCallbackState {
        callback,
        dimension,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut VectorCallbackState<F>).cast();
    ACTIVE_VECTOR_F32.with(|slot| {
        slot.set(Some(VectorSlotF32 {
            data,
            invoke: invoke_vector_f32::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::VectorF32,
    };
    let value = native_call(VectorF32Callback {
        callback: trampoline_vector_f32,
    });
    drop(slot_guard);
    Ok(VectorCallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_least_squares_f64<F, R>(
    parameter_count: usize,
    residual_count: usize,
    callback: F,
    native_call: impl FnOnce(LeastSquaresF64Callback) -> R,
) -> Result<LeastSquaresCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f64], &mut [f64]),
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(LeastSquaresCallbackState {
        callback,
        residual_count,
        parameter_count,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut LeastSquaresCallbackState<F>).cast();
    ACTIVE_LEAST_SQUARES_F64.with(|slot| {
        slot.set(Some(LeastSquaresSlotF64 {
            data,
            invoke: invoke_least_squares_f64::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::LeastSquaresF64,
    };
    let value = native_call(LeastSquaresF64Callback {
        callback: trampoline_least_squares_f64,
    });
    drop(slot_guard);
    Ok(LeastSquaresCallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_least_squares_f32<F, R>(
    parameter_count: usize,
    residual_count: usize,
    callback: F,
    native_call: impl FnOnce(LeastSquaresF32Callback) -> R,
) -> Result<LeastSquaresCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f32], &mut [f32]),
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(LeastSquaresCallbackState {
        callback,
        residual_count,
        parameter_count,
        failure: None,
        evaluations: 0,
    });
    let data = (&mut *state as *mut LeastSquaresCallbackState<F>).cast();
    ACTIVE_LEAST_SQUARES_F32.with(|slot| {
        slot.set(Some(LeastSquaresSlotF32 {
            data,
            invoke: invoke_least_squares_f32::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::LeastSquaresF32,
    };
    let value = native_call(LeastSquaresF32Callback {
        callback: trampoline_least_squares_f32,
    });
    drop(slot_guard);
    Ok(LeastSquaresCallbackInvocation {
        value,
        failure: state.failure,
        evaluations: state.evaluations,
    })
}

pub(crate) fn with_expert_f64<F, J, R>(
    dimension: usize,
    residual: F,
    jacobian: J,
    native_call: impl FnOnce(ExpertF64Callbacks) -> R,
) -> Result<ExpertCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f64], &mut [f64]),
    J: FnMut(&[f64], &[f64], &mut [f64], usize),
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(ExpertCallbackState {
        residual,
        jacobian,
        dimension,
        failure: None,
        residual_evaluations: 0,
        jacobian_evaluations: 0,
    });
    let data = (&mut *state as *mut ExpertCallbackState<F, J>).cast();
    ACTIVE_EXPERT_F64.with(|slot| {
        slot.set(Some(ExpertSlotF64 {
            data,
            residual: invoke_expert_residual_f64::<F, J>,
            jacobian: invoke_expert_jacobian_f64::<F, J>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::ExpertF64,
    };
    let value = native_call(ExpertF64Callbacks {
        residual: trampoline_expert_residual_f64,
        jacobian: trampoline_expert_jacobian_f64,
    });
    drop(slot_guard);
    Ok(ExpertCallbackInvocation {
        value,
        failure: state.failure,
        residual_evaluations: state.residual_evaluations,
        jacobian_evaluations: state.jacobian_evaluations,
    })
}

pub(crate) fn with_expert_f32<F, J, R>(
    dimension: usize,
    residual: F,
    jacobian: J,
    native_call: impl FnOnce(ExpertF32Callbacks) -> R,
) -> Result<ExpertCallbackInvocation<R>, CallbackRuntimeError>
where
    F: FnMut(&[f32], &mut [f32]),
    J: FnMut(&[f32], &[f32], &mut [f32], usize),
{
    if is_active() {
        return Err(CallbackRuntimeError::NestedCallback);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(ExpertCallbackState {
        residual,
        jacobian,
        dimension,
        failure: None,
        residual_evaluations: 0,
        jacobian_evaluations: 0,
    });
    let data = (&mut *state as *mut ExpertCallbackState<F, J>).cast();
    ACTIVE_EXPERT_F32.with(|slot| {
        slot.set(Some(ExpertSlotF32 {
            data,
            residual: invoke_expert_residual_f32::<F, J>,
            jacobian: invoke_expert_jacobian_f32::<F, J>,
        }));
    });
    let slot_guard = SlotGuard {
        kind: CallbackKind::ExpertF32,
    };
    let value = native_call(ExpertF32Callbacks {
        residual: trampoline_expert_residual_f32,
        jacobian: trampoline_expert_jacobian_f32,
    });
    drop(slot_guard);
    Ok(ExpertCallbackInvocation {
        value,
        failure: state.failure,
        residual_evaluations: state.residual_evaluations,
        jacobian_evaluations: state.jacobian_evaluations,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CallbackFailure, CallbackRuntimeError, ExpertCallbackFailure, LeastSquaresCallbackFailure,
        VectorCallbackFailure, with_expert_f64, with_f32, with_f64, with_least_squares_f64,
        with_vector_f64,
    };

    #[test]
    fn callback_context_is_scoped_counted_and_reusable() {
        let invocation = with_f64(|x| x * 2.0, |callback| callback.call(3.0)).unwrap();
        assert_eq!(invocation.value, 6.0);
        assert_eq!(invocation.failure, None);
        assert_eq!(invocation.evaluations, 1);
        assert!(!super::is_active());

        for non_finite in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let invocation = with_f32(move |_| non_finite, |callback| callback.call(1.0)).unwrap();
            assert_eq!(invocation.failure, Some(CallbackFailure::NonFinite));
            assert_eq!(invocation.evaluations, 1);
            assert!(!super::is_active());
        }
    }

    #[test]
    fn panic_is_caught_and_cross_family_nesting_is_rejected() {
        let invocation = with_f64(
            |_| panic!("contained callback panic"),
            |callback| callback.call(1.0),
        )
        .unwrap();
        assert_eq!(invocation.failure, Some(CallbackFailure::Panicked));
        assert!(!super::is_active());

        with_f64(
            |x| x,
            |_| {
                assert!(matches!(
                    with_f32(|x| x, |_| ()),
                    Err(CallbackRuntimeError::NestedCallback)
                ));
            },
        )
        .unwrap();
    }

    #[test]
    fn vector_context_validates_results_and_is_reusable() {
        let mut output = [0.0; 2];
        let input = [1.0, 2.0];
        let invocation = with_vector_f64(
            2,
            |x, f| {
                f.copy_from_slice(x);
                true
            },
            |callback| unsafe {
                let n = 2;
                let mut iflag = 1;
                (callback.ffi())(&n, input.as_ptr(), output.as_mut_ptr(), &mut iflag);
                iflag
            },
        )
        .unwrap();
        assert_eq!(invocation.value, 1);
        assert_eq!(invocation.failure, None);
        assert_eq!(invocation.evaluations, 1);
        assert_eq!(output, input);

        let invocation = with_vector_f64(
            1,
            |_, f| {
                f[0] = f64::NAN;
                true
            },
            |callback| unsafe {
                let n = 1;
                let x = [1.0];
                let mut f = [0.0];
                let mut iflag = 1;
                (callback.ffi())(&n, x.as_ptr(), f.as_mut_ptr(), &mut iflag);
                iflag
            },
        )
        .unwrap();
        assert_eq!(
            invocation.failure,
            Some(VectorCallbackFailure::NonFinite { index: 0 })
        );
        assert_eq!(invocation.value, 1);
        assert!(!super::is_active());
    }

    #[test]
    fn least_squares_context_keeps_rectangular_dimensions_and_flags_distinct() {
        let x = [1.0_f64];
        let mut residual = [0.0_f64; 2];
        let m = 2;
        let n = 1;
        let mut iflag = 1;
        let mut fjac = 0.0_f64;
        let ldfjac = 1;
        let invocation = with_least_squares_f64(
            1,
            2,
            |input, output| {
                assert_eq!(input, &[1.0]);
                output.copy_from_slice(&[1.0, -1.0]);
            },
            |callback| {
                // SAFETY: all values point to valid storage of the reviewed
                // callback's registered M and N dimensions.
                unsafe {
                    (callback.ffi())(
                        &mut iflag,
                        &m,
                        &n,
                        x.as_ptr(),
                        residual.as_mut_ptr(),
                        &mut fjac,
                        &ldfjac,
                    );
                }
            },
        )
        .unwrap();
        assert_eq!(invocation.failure, None);
        assert_eq!(invocation.evaluations, 1);
        assert_eq!(residual, [1.0, -1.0]);

        iflag = 2;
        let unexpected = with_least_squares_f64(
            1,
            2,
            |_, _| unreachable!("unexpected flag must not reach Rust"),
            |callback| {
                // SAFETY: this direct fixture has valid storage and exercises
                // only the contained unexpected-IFLAG guard.
                unsafe {
                    (callback.ffi())(
                        &mut iflag,
                        &m,
                        &n,
                        x.as_ptr(),
                        residual.as_mut_ptr(),
                        &mut fjac,
                        &ldfjac,
                    );
                }
            },
        )
        .unwrap();
        assert_eq!(
            unexpected.failure,
            Some(LeastSquaresCallbackFailure::UnexpectedFlag)
        );
        assert!(!super::is_active());
    }

    #[test]
    fn vector_callback_cancellation_and_nesting_are_contained() {
        let invocation = with_vector_f64(
            1,
            |_, _| false,
            |callback| unsafe {
                let n = 1;
                let x = [1.0];
                let mut f = [0.0];
                let mut iflag = 1;
                (callback.ffi())(&n, x.as_ptr(), f.as_mut_ptr(), &mut iflag);
                iflag
            },
        )
        .unwrap();
        assert_eq!(invocation.failure, Some(VectorCallbackFailure::Cancelled));
        assert_eq!(invocation.value, 1);

        with_vector_f64(
            1,
            |_, f| {
                assert!(matches!(
                    with_f64(|x| x, |_| ()),
                    Err(CallbackRuntimeError::NestedCallback)
                ));
                f[0] = 0.0;
                true
            },
            |callback| unsafe {
                let n = 1;
                let x = [1.0];
                let mut f = [0.0];
                let mut iflag = 1;
                (callback.ffi())(&n, x.as_ptr(), f.as_mut_ptr(), &mut iflag);
            },
        )
        .unwrap();
    }

    #[test]
    fn expert_callback_roles_validate_dimensions_and_leading_dimensions() {
        let invocation = with_expert_f64(
            1,
            |_, residual| residual[0] = 0.0,
            |_, _, matrix, _| matrix[0] = 1.0,
            |callbacks| unsafe {
                let n = 2;
                let x = [1.0, 2.0];
                let mut residual = [0.0, 0.0];
                let mut iflag = 1;
                (callbacks.residual())(&n, x.as_ptr(), residual.as_mut_ptr(), &mut iflag);
            },
        )
        .unwrap();
        assert_eq!(
            invocation.failure,
            Some(ExpertCallbackFailure::DimensionMismatch)
        );

        let invocation = with_expert_f64(
            1,
            |_, residual| residual[0] = 0.0,
            |_, _, matrix, _| matrix[0] = 1.0,
            |callbacks| unsafe {
                let n = 1;
                let ldfjac = 0;
                let x = [1.0];
                let residual = [0.0];
                let mut matrix = [0.0];
                let mut iflag = 1;
                (callbacks.jacobian())(
                    &n,
                    x.as_ptr(),
                    residual.as_ptr(),
                    matrix.as_mut_ptr(),
                    &ldfjac,
                    &mut iflag,
                );
            },
        )
        .unwrap();
        assert_eq!(
            invocation.failure,
            Some(ExpertCallbackFailure::InvalidLeadingDimension)
        );
        assert!(!super::is_active());
    }
}
