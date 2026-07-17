//! Fit a dense model with a lower-sided linear inequality using `LSEI`.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-constrained`. The native relation is `G x >= h`; it
//! is not a general linear-programming interface.

// slatec-safe-example: inequality constrained linear least squares

use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem, GreaterEqualConstraints,
    solve_constrained_least_squares_f32,
};
use slatec::linear_least_squares::MatrixRef;

fn main() -> Result<(), slatec::constrained_least_squares::ConstrainedLeastSquaresError> {
    // min ||x - [-1, 2]|| subject to x_0 >= 0. The first inequality is active.
    let objective = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let inequalities = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f32, 0.0], 1, 2, 1)?,
        lower_bounds: &[0.0],
    };
    let fit = solve_constrained_least_squares_f32(
        ConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[-1.0, 2.0],
            equalities: None,
            inequalities: Some(inequalities),
        },
        ConstrainedLeastSquaresOptions::default(),
    )?;
    assert!(fit.solution[0].abs() < 1.0e-4);
    assert!((fit.solution[1] - 2.0).abs() < 1.0e-4);
    assert!(fit.minimum_inequality_slack().unwrap_or(-1.0) >= -1.0e-4);
    Ok(())
}
