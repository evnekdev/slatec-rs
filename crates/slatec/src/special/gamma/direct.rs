#[cfg(feature = "special-gamma")]
use slatec_sys::families::special_gamma as gamma_raw;

use super::super::{SpecialFunctionError, runtime};

#[cfg(feature = "special-gamma")]
fn gamma_argument(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive(function, "x", x, 30.0)
}

/// Gamma for `0 < x <= 30`, using SLATEC `DGAMMA`.
#[cfg(feature = "special-gamma")]
pub fn gamma(x: f64) -> Result<f64, SpecialFunctionError> {
    gamma_argument("gamma", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: poles and conservative overflow inputs are rejected, the FNLIB
    // global state is serialized, and the scalar ABI/profile is validated.
    Ok(unsafe { gamma_raw::dgamma(&mut x) })
}
