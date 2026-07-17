//! Narrow-link probe for the safe expert nonlinear least-squares family.

use slatec::least_squares::{ExpertLeastSquaresOptions, least_squares_expert};

fn main() {
    let _ = least_squares_expert(
        &[0.0_f64],
        1,
        |parameters, residuals| residuals[0] = parameters[0] - 1.0,
        ExpertLeastSquaresOptions::default(),
    );
}
