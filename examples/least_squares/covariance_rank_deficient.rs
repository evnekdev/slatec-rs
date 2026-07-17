//! Demonstrate the structural singularity result from original `DCOV`.
//!
//! Required features: `std`, a selected native backend, and
//! `least-squares-covariance`. The two columns below are identical, so no
//! parameter-wise covariance can be claimed.

// slatec-safe-example: least-squares covariance rank deficiency

use slatec::least_squares::{
    CovarianceError, CovarianceOptions, estimate_covariance,
};

fn main() {
    let result = estimate_covariance(
        &[1.0, 1.0],
        3,
        |parameters, residuals| {
            for (row, residual) in residuals.iter_mut().enumerate() {
                *residual = parameters[0] + parameters[1] - (row as f64 + 1.0);
            }
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, 1.0).unwrap();
            }
        },
        CovarianceOptions::default(),
    );
    assert!(matches!(result, Err(CovarianceError::RankDeficient { .. })));
}
