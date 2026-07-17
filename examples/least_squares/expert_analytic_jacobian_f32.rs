//! Fit an overdetermined line with the analytic-Jacobian `SNLS1` API.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-nonlinear-expert`. Original SLATEC routine: `SNLS1` with
//! `IOPT = 2`.

// slatec-safe-example: expert least-squares single precision analytic Jacobian

use slatec::least_squares::{
    ExpertLeastSquaresOptions, least_squares_with_jacobian_f32,
};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let fit = least_squares_with_jacobian_f32(
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
        |_, _, mut jacobian| {
            for row in 0..4 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f32).unwrap();
            }
        },
        ExpertLeastSquaresOptions::single_precision(),
    )?;
    assert!((fit.parameters[0] - 1.0).abs() < 2.0e-3);
    assert!((fit.parameters[1] - 2.0).abs() < 2.0e-3);
    assert!(fit.cost < 1.0e-4);
    Ok(())
}
