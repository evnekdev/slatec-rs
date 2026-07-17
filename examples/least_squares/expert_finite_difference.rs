//! Fit an overdetermined line with the expert `DNLS1` forward-difference API.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-nonlinear-expert`. Original SLATEC routine: `DNLS1` with
//! `IOPT = 1`.

// slatec-safe-example: expert least-squares finite differences

use slatec::least_squares::{
    ExpertLeastSquaresOptions, LeastSquaresScaling, least_squares_expert,
};

fn main() -> Result<(), slatec::least_squares::LeastSquaresError> {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.1, 2.9, 5.2, 6.8, 9.05];
    let scales = [1.0, 1.0];
    let options = ExpertLeastSquaresOptions {
        function_tolerance: 1.0e-10,
        parameter_tolerance: 1.0e-10,
        gradient_tolerance: 0.0,
        maximum_function_evaluations: Some(400),
        finite_difference_step: None,
        scaling: LeastSquaresScaling::User(&scales),
        step_bound_factor: 10.0,
    };
    let fit = least_squares_expert(
        &[0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        options,
    )?;
    assert!((fit.parameters[0] - 1.04).abs() < 0.1);
    assert!((fit.parameters[1] - 1.995).abs() < 0.1);
    assert!(fit.cost > 0.0);
    Ok(())
}
