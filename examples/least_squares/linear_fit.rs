//! Fit a line with the safe residual-only `DNLS1E` facade.
//!
//! Required features: `std`, one backend, and
//! `least-squares-nonlinear-easy`. Original SLATEC routine: `DNLS1E`.

// slatec-safe-example: least-squares linear fit

use slatec::least_squares::{LeastSquaresOptions, least_squares};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.0, 3.0, 5.0, 7.0, 9.0];
    let fit = least_squares(
        &[0.25, 0.25],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        LeastSquaresOptions::default(),
    )?;
    assert!((fit.parameters[0] - 1.0).abs() < 1.0e-8);
    assert!((fit.parameters[1] - 2.0).abs() < 1.0e-8);
    assert!(fit.residual_norm < 1.0e-8);
    Ok(())
}
