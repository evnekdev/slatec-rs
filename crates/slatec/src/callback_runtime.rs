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

thread_local! {
    static ACTIVE_F64: Cell<Option<SlotF64>> = const { Cell::new(None) };
    static ACTIVE_F32: Cell<Option<SlotF32>> = const { Cell::new(None) };
    static ACTIVE_VECTOR_F64: Cell<Option<VectorSlotF64>> = const { Cell::new(None) };
    static ACTIVE_VECTOR_F32: Cell<Option<VectorSlotF32>> = const { Cell::new(None) };
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

struct SlotGuard {
    kind: CallbackKind,
}

enum CallbackKind {
    F32,
    F64,
    VectorF32,
    VectorF64,
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        match self.kind {
            CallbackKind::F32 => ACTIVE_F32.with(|slot| slot.set(None)),
            CallbackKind::F64 => ACTIVE_F64.with(|slot| slot.set(None)),
            CallbackKind::VectorF32 => ACTIVE_VECTOR_F32.with(|slot| slot.set(None)),
            CallbackKind::VectorF64 => ACTIVE_VECTOR_F64.with(|slot| slot.set(None)),
        }
    }
}

/// Returns whether any reviewed scalar callback is active on this thread.
pub(crate) fn is_active() -> bool {
    ACTIVE_F64.with(|slot| slot.get().is_some())
        || ACTIVE_F32.with(|slot| slot.get().is_some())
        || ACTIVE_VECTOR_F64.with(|slot| slot.get().is_some())
        || ACTIVE_VECTOR_F32.with(|slot| slot.get().is_some())
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

#[cfg(test)]
mod tests {
    use super::{
        CallbackFailure, CallbackRuntimeError, VectorCallbackFailure, with_f32, with_f64,
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
}
