//! Fit an exponential model with the safe residual-only `DNLS1E` facade.
//!
//! Required features: `std`, one backend, and
//! `least-squares-nonlinear-easy`. Original SLATEC routine: `DNLS1E`.

// slatec-safe-example: least-squares exponential fit

use slatec::least_squares::{LeastSquaresOptions, least_squares};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let xs = [0.0, 1.0, 2.0, 3.0];
    let ys = [1.5, 3.0, 6.0, 12.0];
    let fit = least_squares(
        &[1.0, 0.5],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] * (parameters[1] * x).exp() - y;
            }
        },
        LeastSquaresOptions::default(),
    )?;
    assert!((fit.parameters[0] - 1.5).abs() < 2.0e-8);
    assert!((fit.parameters[1] - core::f64::consts::LN_2).abs() < 2.0e-8);
    assert!(fit.cost < 1.0e-8);
    Ok(())
}
