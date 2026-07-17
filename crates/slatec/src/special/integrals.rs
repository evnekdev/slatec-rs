//! Exponential integrals with fatal singular inputs prechecked in Rust.

use slatec_sys::families::special_integrals as raw;

use super::{SpecialFunctionError, runtime};

fn finite_nonzero(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    if x.is_finite() && x != 0.0 && x.abs() <= 20.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: x,
        })
    }
}

/// Exponential integral E1 on a conservative nonzero finite domain using `DE1`.
pub fn exponential_integral_e1(x: f64) -> Result<f64, SpecialFunctionError> {
    finite_nonzero("exponential_integral_e1", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the singular zero and large error-producing values are excluded;
    // the corrected FNLIB runtime and scalar pointer ABI are validated.
    Ok(unsafe { raw::de1(&mut x) })
}

/// Exponential integral Ei on a conservative nonzero finite domain using `DEI`.
pub fn exponential_integral_ei(x: f64) -> Result<f64, SpecialFunctionError> {
    finite_nonzero("exponential_integral_ei", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: DEI delegates to DE1; zero and conservative overflow cases are
    // rejected before entering the validated FNLIB runtime.
    Ok(unsafe { raw::dei(&mut x) })
}

#[cfg(feature = "special-f32")]
fn finite_nonzero_f32(function: &'static str, x: f32) -> Result<(), SpecialFunctionError> {
    if x.is_finite() && x != 0.0 && x.abs() <= 20.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: f64::from(x),
        })
    }
}

#[cfg(feature = "special-f32")]
/// Single-precision E1 on a conservative nonzero finite domain using `E1`.
pub fn exponential_integral_e1_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    finite_nonzero_f32("exponential_integral_e1_f32", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: singular/large inputs are rejected; scalar ABI/runtime validated.
    Ok(unsafe { raw::e1(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Single-precision Ei on a conservative nonzero finite domain using `EI`.
pub fn exponential_integral_ei_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    finite_nonzero_f32("exponential_integral_ei_f32", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: singular/large inputs are rejected; scalar ABI/runtime validated.
    Ok(unsafe { raw::ei(&mut x) })
}
