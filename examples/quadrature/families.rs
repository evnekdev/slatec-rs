//! Callback-contained examples for the safe quadrature families.
// slatec-safe-example: finite, singular, infinite, weighted, oscillatory, Fourier, QNG, and QNC79.

use slatec::quadrature::{integrate, IntegrationOptions};

fn main() -> Result<(), slatec::quadrature::IntegrationError> {
    let result = integrate(|x| x * x, 0.0, 1.0, IntegrationOptions::default())?;
    assert!((result.value - 1.0 / 3.0).abs() < 1.0e-10);
    Ok(())
}
