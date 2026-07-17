//! Serialization and checked domain helpers for FNLIB-backed calls.
//!
//! The selected FNLIB routines initialise `SAVE`d coefficient metadata and
//! share the legacy XERROR state.  They are therefore serialized here.  This
//! lock is shared with safe callback-based quadrature so an integrand can call
//! a runtime-validated special function reentrantly without deadlocking. It is
//! not a blanket lock around the raw FFI or the BLAS APIs.

#![allow(dead_code)] // Individual family features intentionally select subsets.

use slatec_sys::FortranInteger;

use super::SpecialFunctionError;

/// Acquires the process-global FNLIB runtime guard.
///
/// A poison marker means a prior Rust caller panicked while holding the guard;
/// no Rust callback crosses these calls, so continuing with exclusive access
/// is safer than turning a recoverable poison state into a second panic.
pub(crate) fn lock_fnlib() -> crate::runtime::NativeRuntimeGuard {
    crate::runtime::lock_native()
}

pub(crate) fn finite(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

pub(crate) fn finite_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
) -> Result<(), SpecialFunctionError> {
    finite(function, argument, f64::from(value))
}

pub(crate) fn closed_unit(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

pub(crate) fn closed_unit_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value: f64::from(value),
        })
    }
}

pub(crate) fn bounded(
    function: &'static str,
    argument: &'static str,
    value: f64,
    bound: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value.abs() <= bound {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

pub(crate) fn bounded_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
    bound: f32,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value.abs() <= bound {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value: f64::from(value),
        })
    }
}

pub(crate) fn bounded_positive(
    function: &'static str,
    argument: &'static str,
    value: f64,
    upper: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value > 0.0 && value <= upper {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

pub(crate) fn bounded_positive_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
    upper: f32,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value > 0.0 && value <= upper {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value: f64::from(value),
        })
    }
}

pub(crate) fn integer(
    function: &'static str,
    argument: &'static str,
    value: i32,
) -> Result<FortranInteger, SpecialFunctionError> {
    FortranInteger::try_from(value)
        .map_err(|_| SpecialFunctionError::IntegerOverflow { function, argument })
}
