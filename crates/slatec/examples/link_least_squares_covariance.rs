//! Narrow-link probe for the safe `DCOV` covariance facade.
//!
//! Build with `--features source-build,least-squares-covariance` on the
//! validated GNU MinGW target. The call is intentionally not executed; this
//! binary exists only to inspect normal static-archive extraction.

use slatec::least_squares::{CovarianceOptions, estimate_covariance};

fn main() {
    let _ = estimate_covariance(
        &[1.0, 2.0],
        3,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.1,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.2,
            ]);
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
            }
            jacobian.set(0, 1, 0.0).unwrap();
            jacobian.set(1, 1, 1.0).unwrap();
            jacobian.set(2, 1, 2.0).unwrap();
        },
        CovarianceOptions::default(),
    );
}
