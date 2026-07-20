use crate::special::{SpecialFunctionError, runtime};
use slatec_sys::special::airy as raw;

/// Exponentially scaled single-precision Airy Ai using `AIE`: it returns
/// `Ai(x)` for `x <= 0` and `exp(2 x^(3/2) / 3) Ai(x)` for `x >= 0`.
pub fn airy_ai_scaled_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::bounded_f32("airy_ai_scaled_f32", "x", x, 20.0)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: conservative Airy input range, valid scalar pointer, and the
    // corrected single-precision FNLIB runtime profile.
    Ok(unsafe { raw::aie(&mut x) })
}
