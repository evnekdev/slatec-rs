use crate::special::{SpecialFunctionError, runtime};
use slatec_sys::special::airy as raw;

/// Exponentially scaled Airy Ai for `-20 <= x <= 20`, using `DAIE`: it
/// returns `Ai(x)` for `x <= 0` and `exp(2 x^(3/2) / 3) Ai(x)` for `x >= 0`.
pub fn airy_ai_scaled(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::bounded("airy_ai_scaled", "x", x, 20.0)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the conservative range prevents the documented Airy
    // overflow/underflow error paths; the FNLIB profile and pointer ABI were
    // explicitly validated.
    Ok(unsafe { raw::daie(&mut x) })
}
