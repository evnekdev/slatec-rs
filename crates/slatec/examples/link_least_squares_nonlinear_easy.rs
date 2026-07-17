//! Narrow link-retention probe for the nonlinear least-squares easy drivers.

use slatec::least_squares::{LeastSquaresOptions, least_squares};

fn main() {
    let _ = least_squares(
        &[0.0_f64, 0.0],
        3,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ]);
        },
        LeastSquaresOptions::default(),
    );
}
