//! Quadrature-specific mapping over the shared scalar callback runtime.

use crate::callback_runtime;
pub(crate) use crate::callback_runtime::CallbackFailure;
use slatec_sys::quadrature::{IntegrandF32, IntegrandF64};

use super::IntegrationError;

pub(crate) fn with_f64<F, R>(
    callback: F,
    native_call: impl FnOnce(IntegrandF64) -> R,
) -> Result<(R, Option<CallbackFailure>), IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    let invocation = callback_runtime::with_f64(callback, |handle| native_call(handle.ffi()))
        .map_err(|_| IntegrationError::NestedIntegration)?;
    Ok((invocation.value, invocation.failure))
}

#[allow(dead_code)] // DPFQAD is f64-only; sibling quadrature features use this adapter.
pub(crate) fn with_f32<F, R>(
    callback: F,
    native_call: impl FnOnce(IntegrandF32) -> R,
) -> Result<(R, Option<CallbackFailure>), IntegrationError>
where
    F: FnMut(f32) -> f32,
{
    let invocation = callback_runtime::with_f32(callback, |handle| native_call(handle.ffi()))
        .map_err(|_| IntegrationError::NestedIntegration)?;
    Ok((invocation.value, invocation.failure))
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
    fn quadrature_mapping_preserves_scoped_callback_behavior() {
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
        assert!(!crate::callback_runtime::is_active());

        let (_, failure) = with_f32(
            |_| f32::INFINITY,
            |callback| {
                let x = 1.0;
                unsafe { callback(&x) }
            },
        )
        .unwrap();
        assert_eq!(failure, Some(CallbackFailure::NonFinite));
    }

    #[test]
    fn nested_callback_registration_maps_to_quadrature_error() {
        let result = with_f64(|x| x, |_| with_f64(|x| x, |_| ())).unwrap();
        assert!(matches!(
            result.0,
            Err(crate::quadrature::IntegrationError::NestedIntegration)
        ));
    }
}
