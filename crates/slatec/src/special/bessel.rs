//! Scalar real Bessel functions with explicit scaled and unscaled names.

use slatec_sys::generated::scalar_functions as raw;

use super::{SpecialFunctionError, runtime};

fn oscillatory(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded(function, "x", x, 1.0e6)
}

fn positive(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive(function, "x", x, 20.0)
}

macro_rules! unary_f64 {
    ($name:ident, $raw:ident, $doc:literal, $validate:ident) => {
        #[doc = $doc]
        pub fn $name(x: f64) -> Result<f64, SpecialFunctionError> {
            $validate(stringify!($name), x)?;
            let _guard = runtime::lock_fnlib();
            let mut x = x;
            // Safety: the wrapper checks the routine's conservative nonfatal
            // input envelope, serializes FNLIB state, and passes one valid
            // scalar pointer through the validated GNU MinGW ABI.
            Ok(unsafe { raw::$raw(&mut x) })
        }
    };
}

unary_f64!(
    bessel_j0,
    dbesj0,
    "Bessel J of order zero, using `DBESJ0`.",
    oscillatory
);
unary_f64!(
    bessel_j1,
    dbesj1,
    "Bessel J of order one, using `DBESJ1`.",
    oscillatory
);
unary_f64!(
    bessel_y0,
    dbesy0,
    "Bessel Y of order zero for `0 < x <= 20`, using `DBESY0`.",
    positive
);
unary_f64!(
    bessel_y1,
    dbesy1,
    "Bessel Y of order one for `0 < x <= 20`, using `DBESY1`.",
    positive
);
unary_f64!(
    bessel_i0,
    dbesi0,
    "Modified Bessel I of order zero on the conservative safe envelope, using `DBESI0`.",
    positive_or_negative
);
unary_f64!(
    bessel_i1,
    dbesi1,
    "Modified Bessel I of order one on the conservative safe envelope, using `DBESI1`.",
    positive_or_negative
);
unary_f64!(
    bessel_i0_scaled,
    dbsi0e,
    "Exponentially scaled modified Bessel I0, using `DBSI0E`.",
    positive_or_negative
);
unary_f64!(
    bessel_i1_scaled,
    dbsi1e,
    "Exponentially scaled modified Bessel I1, using `DBSI1E`.",
    positive_or_negative
);
unary_f64!(
    bessel_k0,
    dbesk0,
    "Modified Bessel K of order zero for `0 < x <= 20`, using `DBESK0`.",
    positive
);
unary_f64!(
    bessel_k1,
    dbesk1,
    "Modified Bessel K of order one for `0 < x <= 20`, using `DBESK1`.",
    positive
);
unary_f64!(
    bessel_k0_scaled,
    dbsk0e,
    "Exponentially scaled modified Bessel K0, using `DBSK0E`.",
    positive
);
unary_f64!(
    bessel_k1_scaled,
    dbsk1e,
    "Exponentially scaled modified Bessel K1, using `DBSK1E`.",
    positive
);

fn positive_or_negative(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded(function, "x", x, 20.0)
}

#[cfg(feature = "special-functions-f32")]
fn oscillatory_f32(function: &'static str, x: f32) -> Result<(), SpecialFunctionError> {
    runtime::bounded_f32(function, "x", x, 1.0e3)
}

#[cfg(feature = "special-functions-f32")]
fn positive_f32(function: &'static str, x: f32) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive_f32(function, "x", x, 20.0)
}

#[cfg(feature = "special-functions-f32")]
fn positive_or_negative_f32(function: &'static str, x: f32) -> Result<(), SpecialFunctionError> {
    runtime::bounded_f32(function, "x", x, 20.0)
}

#[cfg(feature = "special-functions-f32")]
macro_rules! unary_f32 {
    ($name:ident, $raw:ident, $doc:literal, $validate:ident) => {
        #[doc = $doc]
        pub fn $name(x: f32) -> Result<f32, SpecialFunctionError> {
            $validate(stringify!($name), x)?;
            let _guard = runtime::lock_fnlib();
            let mut x = x;
            // Safety: conservative domain, serialized FNLIB state, and a valid
            // scalar pointer using the explicitly validated profile ABI.
            Ok(unsafe { raw::$raw(&mut x) })
        }
    };
}

#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_j0_f32,
    besj0,
    "Single-precision Bessel J0 using `BESJ0`.",
    oscillatory_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_j1_f32,
    besj1,
    "Single-precision Bessel J1 using `BESJ1`.",
    oscillatory_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_y0_f32,
    besy0,
    "Single-precision Bessel Y0 for positive x, using `BESY0`.",
    positive_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_y1_f32,
    besy1,
    "Single-precision Bessel Y1 for positive x, using `BESY1`.",
    positive_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_i0_f32,
    besi0,
    "Single-precision modified Bessel I0 using `BESI0`.",
    positive_or_negative_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_i1_f32,
    besi1,
    "Single-precision modified Bessel I1 using `BESI1`.",
    positive_or_negative_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_i0_scaled_f32,
    besi0e,
    "Scaled single-precision modified Bessel I0 using `BESI0E`.",
    positive_or_negative_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_i1_scaled_f32,
    besi1e,
    "Scaled single-precision modified Bessel I1 using `BESI1E`.",
    positive_or_negative_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_k0_f32,
    besk0,
    "Single-precision modified Bessel K0 for positive x using `BESK0`.",
    positive_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_k1_f32,
    besk1,
    "Single-precision modified Bessel K1 for positive x using `BESK1`.",
    positive_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_k0_scaled_f32,
    besk0e,
    "Scaled single-precision modified Bessel K0 using `BESK0E`.",
    positive_f32
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    bessel_k1_scaled_f32,
    besk1e,
    "Scaled single-precision modified Bessel K1 using `BESK1E`.",
    positive_f32
);
