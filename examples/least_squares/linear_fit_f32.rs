//! Fit a line with the safe residual-only `SNLS1E` facade.
//!
//! Required features: `std`, one backend, and
//! `least-squares-nonlinear-easy`. Original SLATEC routine: `SNLS1E`.

// slatec-safe-example: least-squares single-precision linear fit

use slatec::least_squares::{LeastSquaresOptions, least_squares_f32};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let fit = least_squares_f32(
        &[0.0, 0.0],
        4,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
                parameters[0] + 3.0 * parameters[1] - 7.0,
            ]);
        },
        LeastSquaresOptions::single_precision(),
    )?;
    assert!((fit.parameters[0] - 1.0).abs() < 2.0e-3);
    assert!((fit.parameters[1] - 2.0).abs() < 2.0e-3);
    Ok(())
}
