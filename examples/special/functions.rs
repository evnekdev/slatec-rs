//! Runtime-validated examples spanning every exposed special-function family.
// slatec-safe-example: scalar, Airy, Bessel, gamma/beta, integral, and polynomial families.

use slatec::polynomials::chebyshev::chebyshev_series;
use slatec::special::{airy::airy_ai, bessel::bessel_j0, elementary::sin_degrees, error_functions::erf, gamma::{beta, gamma, reciprocal_gamma}, integrals::exponential_integral_ei};

fn main() -> Result<(), slatec::special::SpecialFunctionError> {
    assert!((gamma(0.5)? - core::f64::consts::PI.sqrt()).abs() < 1.0e-12);
    assert!((reciprocal_gamma(1.0)? - 1.0).abs() < 1.0e-14);
    assert!((beta(1.0, 1.0)? - 1.0).abs() < 1.0e-14);
    assert!(erf(0.0)?.abs() < 1.0e-15);
    assert!((airy_ai(0.0)? - 0.355_028_053_887_817_2).abs() < 1.0e-13);
    assert!((exponential_integral_ei(1.0)? - 1.895_117_816_355_936_8).abs() < 1.0e-12);
    assert!((bessel_j0(0.0)? - 1.0).abs() < 1.0e-13);
    assert!((sin_degrees(30.0)? - 0.5).abs() < 1.0e-13);
    assert!(chebyshev_series(0.0, &[1.0, 0.5])?.is_finite());
    Ok(())
}
