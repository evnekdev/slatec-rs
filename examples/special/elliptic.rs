//! Carlson symmetric elliptic integrals using `special-scalar-expanded`.
//!
//! Requires `std`, an explicit native backend, and `special-scalar-expanded`.
// slatec-safe-example: real scalar Carlson symmetric elliptic integrals.

use slatec::special::scalar_expanded::{carlson_rc, carlson_rd, carlson_rf, carlson_rj};

fn main() -> Result<(), slatec::special::SpecialFunctionError> {
    assert!((carlson_rc(0.0, 1.0)? - core::f64::consts::FRAC_PI_2).abs() < 1.0e-12);
    assert!((carlson_rf(1.0, 1.0, 1.0)? - 1.0).abs() < 1.0e-13);
    assert!((carlson_rd(1.0, 1.0, 1.0)? - 1.0).abs() < 1.0e-13);
    assert!((carlson_rj(1.0, 1.0, 1.0, 1.0)? - 1.0).abs() < 1.0e-13);
    Ok(())
}
