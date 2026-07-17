//! Estimate a line-fit covariance matrix with the original `DCOV` routine.
//!
//! Required features: `std`, a selected native backend, and
//! `least-squares-covariance`. `DCOV` uses a forward-difference Jacobian here.
//! Its explicit residual-variance scaling is appropriate because this example
//! has more observations than fitted parameters.

// slatec-safe-example: least-squares covariance linear fit

use slatec::least_squares::{
    CovarianceOptions, CovarianceScaling, estimate_covariance_finite_difference,
};

fn main() -> Result<(), slatec::least_squares::CovarianceError> {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.1, 2.9, 5.2, 7.1, 8.8];
    let covariance = estimate_covariance_finite_difference(
        &[1.0, 2.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        CovarianceOptions {
            scaling: CovarianceScaling::ResidualVariance,
        },
    )?;
    assert_eq!(covariance.rank, 2);
    assert!(covariance.get(0, 0).unwrap() > &0.0);
    assert_eq!(covariance.get(0, 1), covariance.get(1, 0));
    Ok(())
}
