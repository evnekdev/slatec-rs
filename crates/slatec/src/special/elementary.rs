//! Stable elementary helpers whose SLATEC implementations differ from a
//! direct portable intrinsic in their intended small-argument behavior.

use slatec_sys::special::elementary as raw;

use super::{SpecialFunctionError, runtime};

/// Accurate `ln(1 + x)` for finite `x > -1` using SLATEC `DLNREL`.
pub fn log1p(x: f64) -> Result<f64, SpecialFunctionError> {
    if !(x.is_finite() && x > -1.0) {
        return Err(SpecialFunctionError::Domain {
            function: "log1p",
            argument: "x",
            value: x,
        });
    }
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: `x` is finite and in DLNREL's nonfatal domain; the profile and
    // FNLIB initialization path are validated, and the scalar pointer is valid.
    Ok(unsafe { raw::dlnrel(&mut x) })
}

/// Accurate `(exp(x) - 1) / x`, with the limiting value at zero, using `DEXPRL`.
pub fn exprel(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("exprel", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input, validated FNLIB runtime, and valid pointer.
    Ok(unsafe { raw::dexprl(&mut x) })
}

/// Real cube root for a finite value using SLATEC `DCBRT`.
pub fn cbrt(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("cbrt", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and validated FNLIB runtime/pointer ABI.
    Ok(unsafe { raw::dcbrt(&mut x) })
}

/// Sine of a finite degree-valued angle using SLATEC `DSINDG`.
pub fn sin_degrees(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("sin_degrees", "degrees", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and the validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::dsindg(&mut x) })
}

/// Cosine of a finite degree-valued angle using SLATEC `DCOSDG`.
pub fn cos_degrees(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::finite("cos_degrees", "degrees", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and the validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::dcosdg(&mut x) })
}

/// Dawson's integral for a finite argument using SLATEC `DDAWS`.
pub fn dawson(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::bounded("dawson", "x", x, 1.0e6)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the conservative finite range avoids DDAWS's error path; the
    // scalar pointer and corrected FNLIB profile are valid.
    Ok(unsafe { raw::ddaws(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Accurate `ln(1 + x)` for finite `x > -1` using SLATEC `ALNREL`.
pub fn log1p_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    if !(x.is_finite() && x > -1.0) {
        return Err(SpecialFunctionError::Domain {
            function: "log1p_f32",
            argument: "x",
            value: f64::from(x),
        });
    }
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: x is within ALNREL's nonfatal domain and uses the validated ABI.
    Ok(unsafe { raw::alnrel(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Accurate `(exp(x) - 1) / x`, including its limiting value, using `EXPREL`.
pub fn exprel_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("exprel_f32", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::exprel(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Real cube root for a finite value using SLATEC `CBRT`.
pub fn cbrt_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("cbrt_f32", "x", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::cbrt(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Sine of a finite degree-valued angle using SLATEC `SINDG`.
pub fn sin_degrees_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("sin_degrees_f32", "degrees", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::sindg(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Cosine of a finite degree-valued angle using SLATEC `COSDG`.
pub fn cos_degrees_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::finite_f32("cos_degrees_f32", "degrees", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: finite scalar input and validated FNLIB ABI/runtime state.
    Ok(unsafe { raw::cosdg(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Dawson's integral for a conservative finite range using SLATEC `DAWS`.
pub fn dawson_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::bounded_f32("dawson_f32", "x", x, 1.0e3)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: conservative finite input, valid pointer, and validated profile.
    Ok(unsafe { raw::daws(&mut x) })
}
