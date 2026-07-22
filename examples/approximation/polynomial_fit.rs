//! Checked weighted polynomial least-squares fitting.
//!
//! slatec-safe-example

use slatec::interpolation::approximation::{
    PolynomialFit, PolynomialFitOptions, PolynomialFitStatus,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // These samples lie exactly on 1 + 2x + 0.5x². Unlike interpolation,
    // polynomial fitting also accepts repeated or unordered abscissas.
    let abscissas = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let values = [-1.0_f64, -0.5, 1.0, 3.5, 7.0];
    let fit = PolynomialFit::<f64>::fit(&abscissas, &values, PolynomialFitOptions::all_degrees(2))?;

    assert_eq!(fit.degree(), 2);
    assert!(matches!(fit.status(), PolynomialFitStatus::Complete));
    assert!((fit.evaluate(0.5)? - 2.125).abs() < 1.0e-10);
    assert_eq!(fit.power_coefficients()?.len(), 3);
    Ok(())
}
