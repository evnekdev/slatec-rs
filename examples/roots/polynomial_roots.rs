//! Reviewed single-precision polynomial roots from SLATEC.
// slatec-safe-example: complex polynomial roots

use num_complex::Complex32;
use slatec::roots::{PolynomialRootMethod, complex_polynomial_roots_with_method};

fn main() -> Result<(), slatec::roots::PolynomialRootError> {
    // (z - (1 + i)) * (z - (-2 + 0.5i)), in descending powers.
    let coefficients = [
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, -1.5),
        Complex32::new(-2.5, 1.5),
    ];
    let roots =
        complex_polynomial_roots_with_method(&coefficients, PolynomialRootMethod::CompanionQr)?;
    println!("status={:?}, roots={:?}", roots.status(), roots.roots());
    Ok(())
}
