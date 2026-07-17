//! Real Airy functions with a conservative overflow-safe input envelope.

use slatec_sys::families::special_airy as raw;

use super::{SpecialFunctionError, runtime};

macro_rules! airy_f64 {
    ($name:ident, $raw:ident, $doc:literal) => {
        #[doc = $doc]
        pub fn $name(x: f64) -> Result<f64, SpecialFunctionError> {
            runtime::bounded(stringify!($name), "x", x, 20.0)?;
            let _guard = runtime::lock_fnlib();
            let mut x = x;
            // Safety: the conservative range prevents the documented Airy
            // overflow/underflow error paths; the FNLIB profile and pointer ABI
            // were explicitly validated.
            Ok(unsafe { raw::$raw(&mut x) })
        }
    };
}

airy_f64!(
    airy_ai,
    dai,
    "Airy Ai for `-20 <= x <= 20`, using SLATEC `DAI`."
);
airy_f64!(
    airy_ai_scaled,
    daie,
    "Exponentially scaled Airy Ai for `-20 <= x <= 20`, using `DAIE`."
);
airy_f64!(
    airy_bi,
    dbi,
    "Airy Bi for `-20 <= x <= 20`, using SLATEC `DBI`."
);
airy_f64!(
    airy_bi_scaled,
    dbie,
    "Exponentially scaled Airy Bi for `-20 <= x <= 20`, using `DBIE`."
);

#[cfg(feature = "special-f32")]
macro_rules! airy_f32 {
    ($name:ident, $raw:ident, $doc:literal) => {
        #[doc = $doc]
        pub fn $name(x: f32) -> Result<f32, SpecialFunctionError> {
            runtime::bounded_f32(stringify!($name), "x", x, 20.0)?;
            let _guard = runtime::lock_fnlib();
            let mut x = x;
            // Safety: conservative Airy input range, valid scalar pointer, and
            // the corrected single-precision FNLIB runtime profile.
            Ok(unsafe { raw::$raw(&mut x) })
        }
    };
}

#[cfg(feature = "special-f32")]
airy_f32!(
    airy_ai_f32,
    ai,
    "Single-precision Airy Ai on the safe envelope, using `AI`."
);
#[cfg(feature = "special-f32")]
airy_f32!(
    airy_ai_scaled_f32,
    aie,
    "Scaled single-precision Airy Ai using `AIE`."
);
#[cfg(feature = "special-f32")]
airy_f32!(
    airy_bi_f32,
    bi,
    "Single-precision Airy Bi on the safe envelope, using `BI`."
);
#[cfg(feature = "special-f32")]
airy_f32!(
    airy_bi_scaled_f32,
    bie,
    "Scaled single-precision Airy Bi using `BIE`."
);
