use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};

use slatec_sys::quadrature::{IntegrandF32, IntegrandF64};

use super::IntegrationError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallbackFailure {
    Panicked,
    NonFinite,
    InvalidPointer,
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

pub(crate) fn with_f64<F, R>(
    callback: F,
    native_call: impl FnOnce(IntegrandF64) -> R,
) -> Result<(R, Option<CallbackFailure>), IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    if is_active() {
        return Err(IntegrationError::NestedIntegration);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(CallbackState {
        callback,
        failure: None,
    });
    ACTIVE_F64.with(|slot| {
        slot.set(Some(SlotF64 {
            data: (&mut *state as *mut CallbackState<F>).cast(),
            invoke: invoke_f64::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        precision: Precision::F64,
    };
    let output = native_call(trampoline_f64);
    drop(slot_guard);
    Ok((output, state.failure))
}

pub(crate) fn with_f32<F, R>(
    callback: F,
    native_call: impl FnOnce(IntegrandF32) -> R,
) -> Result<(R, Option<CallbackFailure>), IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    if is_active() {
        return Err(IntegrationError::NestedIntegration);
    }
    let _runtime_guard = crate::runtime::lock_native();
    let mut state = Box::new(CallbackState {
        callback,
        failure: None,
    });
    ACTIVE_F32.with(|slot| {
        slot.set(Some(SlotF32 {
            data: (&mut *state as *mut CallbackState<F>).cast(),
            invoke: invoke_f32::<F>,
        }));
    });
    let slot_guard = SlotGuard {
        precision: Precision::F32,
    };
    let output = native_call(trampoline_f32);
    drop(slot_guard);
    Ok((output, state.failure))
}

pub(crate) fn failure_error(failure: CallbackFailure) -> IntegrationError {
    match failure {
        CallbackFailure::Panicked => IntegrationError::CallbackPanicked,
        CallbackFailure::NonFinite => IntegrationError::CallbackFailed,
        CallbackFailure::InvalidPointer => IntegrationError::NativeContractViolation,
    }
}

#[cfg(test)]
mod tests {
    use super::{CallbackFailure, with_f32, with_f64};

    #[test]
    fn callback_context_is_scoped_and_reusable() {
        let (value, failure) = with_f64(
            |x| x * 2.0,
            |callback| {
                let x = 3.0;
                unsafe { callback(&x) }
            },
        )
        .unwrap();
        assert_eq!(value, 6.0);
        assert_eq!(failure, None);
        assert!(!super::is_active());

        for non_finite in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let (_, failure) = with_f32(
                move |_| non_finite,
                |callback| {
                    let x = 1.0;
                    unsafe { callback(&x) }
                },
            )
            .unwrap();
            assert_eq!(failure, Some(CallbackFailure::NonFinite));
            assert!(!super::is_active());
        }
    }

    #[test]
    fn panic_is_caught_and_nested_registration_is_rejected() {
        let (_, failure) = with_f64(
            |_| panic!("contained callback panic"),
            |callback| {
                let x = 1.0;
                unsafe { callback(&x) }
            },
        )
        .unwrap();
        assert_eq!(failure, Some(CallbackFailure::Panicked));
        assert!(!super::is_active());

        with_f64(
            |x| x,
            |_| {
                assert!(matches!(
                    with_f64(|x| x, |_| ()),
                    Err(crate::quadrature::IntegrationError::NestedIntegration)
                ));
            },
        )
        .unwrap();
    }
}
