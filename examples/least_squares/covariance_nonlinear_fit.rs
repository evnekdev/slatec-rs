//! Estimate covariance after an analytic-Jacobian nonlinear fit with `DCOV`.
//!
//! Required features: `std`, a selected native backend,
//! `least-squares-nonlinear-expert`, and `least-squares-covariance`. `DCOV`
//! recomputes the model residuals and Jacobian; it does not reuse the solver's
//! QR factorization.

// slatec-safe-example: least-squares covariance nonlinear fit

use slatec::least_squares::{
    CovarianceEligibility, CovarianceOptions, ExpertLeastSquaresOptions,
    covariance_from_expert_fit, least_squares_with_jacobian,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xs = [0.0, 1.0, 2.0, 3.0];
    let ys = [1.1, 3.0, 5.1, 6.8];
    let fit = least_squares_with_jacobian(
        &[0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x).unwrap();
            }
        },
        ExpertLeastSquaresOptions::default(),
    )?;
    let covariance = covariance_from_expert_fit(
        &fit,
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x).unwrap();
            }
        },
        CovarianceOptions::default(),
        CovarianceEligibility::ConvergedOnly,
    )?;
    assert!(covariance.standard_errors()?[0] > 0.0);
    Ok(())
}
