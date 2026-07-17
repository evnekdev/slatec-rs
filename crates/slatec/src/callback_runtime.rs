//! Private scoped runtime for reviewed scalar Fortran callbacks.
//!
//! The selected GNU Fortran scalar callback ABI has no user-data argument.
//! This module therefore owns a thread-local, lexical callback slot while the
//! process-wide native runtime lock is held. It is shared by quadrature and
//! scalar-root adapters so callback-bearing families cannot nest into each
//! other accidentally.

use alloc::boxed::Box;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread_local;

/// Reviewed GNU Fortran callback shape for a double-precision scalar value.
pub(crate) type ScalarFnF64 = unsafe extern "C" fn(*const f64) -> f64;

/// Reviewed GNU Fortran callback shape for a single-precision scalar value.
pub(crate) type ScalarFnF32 = unsafe extern "C" fn(*const f32) -> f32;

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

pub(crate) struct CallbackInvocation<R> {
    pub(crate) value: R,
    pub(crate) failure: Option<CallbackFailure>,
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

thread_local! {
    static ACTIVE_F64: Cell<Option<SlotF64>> = const { Cell::new(None) };
    static ACTIVE_F32: Cell<Option<SlotF32>> = const { Cell::new(None) };
}

struct CallbackState<F> {
    callback: F,
    failure: Option<CallbackFailure>,
    evaluations: usize,
}

struct SlotGuard {
    precision: Precision,
}

enum Precision {
    F32,
    F64,
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        match self.precision {
            Precision::F32 => ACTIVE_F32.with(|slot| slot.set(None)),
            Precision::F64 => ACTIVE_F64.with(|slot| slot.set(None)),
        }
    }
}

/// Returns whether any reviewed scalar callback is active on this thread.
pub(crate) fn is_active() -> bool {
    ACTIVE_F64.with(|slot| slot.get().is_some()) || ACTIVE_F32.with(|slot| slot.get().is_some())
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
        precision: Precision::F64,
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
        precision: Precision::F32,
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

#[cfg(test)]
mod tests {
    use super::{CallbackFailure, CallbackRuntimeError, with_f32, with_f64};

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
}
