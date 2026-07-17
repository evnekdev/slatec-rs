//! Fit an exponential curve with the expert analytic-Jacobian `DNLS1` API.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-nonlinear-expert`. Original SLATEC routine: `DNLS1` with
//! `IOPT = 2`.

// slatec-safe-example: expert least-squares analytic Jacobian

use slatec::least_squares::{
    ExpertLeastSquaresOptions, least_squares_with_jacobian,
};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let xs = [0.0, 1.0, 2.0, 3.0];
    let ys = [1.5, 3.0, 6.0, 12.0];
    let fit = least_squares_with_jacobian(
        &[1.0, 0.5],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] * (parameters[1] * x).exp() - y;
            }
        },
        |parameters, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                let exponential = (parameters[1] * x).exp();
                jacobian.set(row, 0, exponential).unwrap();
                jacobian.set(row, 1, parameters[0] * x * exponential).unwrap();
            }
        },
        ExpertLeastSquaresOptions::default(),
    )?;
    assert!((fit.parameters[0] - 1.5).abs() < 2.0e-8);
    assert!((fit.parameters[1] - core::f64::consts::LN_2).abs() < 2.0e-8);
    assert!(fit.cost < 1.0e-10);
    assert!(fit.jacobian_evaluations > 0);
    Ok(())
}
