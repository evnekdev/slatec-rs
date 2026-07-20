use crate::special::{SpecialFunctionError, runtime};
use slatec_sys::special::airy as raw;

/// Single-precision Airy Ai on the safe envelope, using `AI`.
pub fn airy_ai_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    runtime::bounded_f32("airy_ai_f32", "x", x, 20.0)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: conservative Airy input range, valid scalar pointer, and the
    // corrected single-precision FNLIB runtime profile.
    Ok(unsafe { raw::ai(&mut x) })
}
