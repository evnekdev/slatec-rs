//! Error functions from the validated SLATEC FNLIB runtime.

use slatec_sys::special::error as raw;

use super::{SpecialFunctionError, runtime};

/// Error function for a finite argument using SLATEC `DERF`.
pub fn erf(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("erf", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input, validated FNLIB runtime, and valid pointer.
    Ok(unsafe { raw::derf(&mut x) })
}

/// Complementary error function for a finite argument using SLATEC `DERFC`.
pub fn erfc(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("erfc", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input, validated FNLIB runtime, and valid pointer.
    Ok(unsafe { raw::derfc(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Single-precision error function for a finite argument using SLATEC `ERF`.
pub fn erf_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("erf_f32", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input, validated FNLIB runtime, and valid pointer.
    Ok(unsafe { raw::erf(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Single-precision complementary error function using SLATEC `ERFC`.
pub fn erfc_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("erfc_f32", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input, validated FNLIB runtime, and valid pointer.
    Ok(unsafe { raw::erfc(&mut x) })
}
