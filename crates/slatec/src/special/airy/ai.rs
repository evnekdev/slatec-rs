use crate::special::{SpecialFunctionError, runtime};
use slatec_sys::special::airy as raw;

/// Airy Ai for `-20 <= x <= 20`, using SLATEC `DAI`.
pub fn airy_ai(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::bounded("airy_ai", "x", x, 20.0)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the conservative range prevents the documented Airy
    // overflow/underflow error paths; the FNLIB profile and pointer ABI were
    // explicitly validated.
    Ok(unsafe { raw::dai(&mut x) })
}
