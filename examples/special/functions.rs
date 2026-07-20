//! Runtime-validated examples spanning every exposed special-function family.
// slatec-safe-example: scalar, Airy, Bessel, gamma/beta, integral, and polynomial families.

use slatec::polynomials::chebyshev::chebyshev_series;
use slatec::special::{
    airy::{airy_ai, airy_ai_scaled, airy_bi, airy_bi_scaled},
    bessel::bessel_j0,
    elementary::sin_degrees,
    error_functions::erf,
    gamma::{beta, gamma, reciprocal_gamma},
    integrals::exponential_integral_ei,
};

fn main() -> Result<(), slatec::special::SpecialFunctionError> {
    assert!((gamma(0.5)? - core::f64::consts::PI.sqrt()).abs() < 1.0e-12);
    assert!((reciprocal_gamma(1.0)? - 1.0).abs() < 1.0e-14);
    assert!((beta(1.0, 1.0)? - 1.0).abs() < 1.0e-14);
    assert!(erf(0.0)?.abs() < 1.0e-15);
    let x: f64 = 1.0;
    let ai = airy_ai(x)?;
    let bi = airy_bi(x)?;
    assert!((ai - 0.135_292_416_312_881_4).abs() < 1.0e-13);
    assert!((bi - 1.207_423_594_952_871_3).abs() < 1.0e-12);
    let scale = (2.0 * x * x.sqrt() / 3.0).exp();
    assert!((airy_ai_scaled(x)? - scale * ai).abs() < 1.0e-12);
    assert!((airy_bi_scaled(x)? - bi / scale).abs() < 1.0e-12);

    // FNLIB has no separately promoted real derivative driver.  Public value
    // calls still demonstrate the Airy Wronskian with centred differences.
    let h = 1.0e-4;
    let ai_prime = (airy_ai(x + h)? - airy_ai(x - h)?) / (2.0 * h);
    let bi_prime = (airy_bi(x + h)? - airy_bi(x - h)?) / (2.0 * h);
    assert!((ai * bi_prime - ai_prime * bi - 1.0 / core::f64::consts::PI).abs() < 3.0e-8);
    assert!((exponential_integral_ei(1.0)? - 1.895_117_816_355_936_8).abs() < 1.0e-12);
    assert!((bessel_j0(0.0)? - 1.0).abs() < 1.0e-13);
    assert!((sin_degrees(30.0)? - 0.5).abs() < 1.0e-13);
    assert!(chebyshev_series(0.0, &[1.0, 0.5])?.is_finite());
    Ok(())
}
