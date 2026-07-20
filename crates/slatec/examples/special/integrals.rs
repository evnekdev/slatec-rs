//! Logarithmic and Spence integral examples using `special-scalar-expanded`.
//!
//! Requires `std`, an explicit native backend, and `special-scalar-expanded`.
// slatec-safe-example: real scalar logarithmic and Spence integrals.

use slatec::special::scalar_expanded::{logarithmic_integral, spence_integral};

fn main() -> Result<(), slatec::special::SpecialFunctionError> {
    let li_two = logarithmic_integral(2.0)?;
    let spence_one = spence_integral(1.0)?;

    assert!((li_two - 1.045_163_780_117_492_8).abs() < 1.0e-12);
    assert!((spence_one - core::f64::consts::PI.powi(2) / 6.0).abs() < 1.0e-12);
    Ok(())
}
