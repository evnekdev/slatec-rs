use crate::special::{SpecialFunctionError, runtime};
use slatec_sys::special::airy as raw;

/// Airy Bi for `-20 <= x <= 20`, using SLATEC `DBI`.
pub fn airy_bi(x: f64) -> Result<f64, SpecialFunctionError> {
    runtime::bounded("airy_bi", "x", x, 20.0)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the conservative range prevents the documented Airy
    // overflow/underflow error paths; the FNLIB profile and pointer ABI were
    // explicitly validated.
    Ok(unsafe { raw::dbi(&mut x) })
}
