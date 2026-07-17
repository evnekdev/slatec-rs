//! Combine exact equalities and lower-sided inequalities with `DLSEI`.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-constrained`. The solve is a residual minimization
//! with linear feasibility constraints, rather than a linear-programming task.

// slatec-safe-example: mixed equality and inequality constrained linear least squares

use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem, GreaterEqualConstraints,
    LinearConstraintBlock, solve_constrained_least_squares,
};
use slatec::linear_least_squares::MatrixRef;

fn main() -> Result<(), slatec::constrained_least_squares::ConstrainedLeastSquaresError> {
    // The objective prefers [0, 3], while x_0 + x_1 = 3 and x_0 >= 1 force
    // the feasible minimizer [1, 2].
    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1)?,
        rhs: &[3.0],
    };
    let inequalities = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, 0.0], 1, 2, 1)?,
        lower_bounds: &[1.0],
    };
    let fit = solve_constrained_least_squares(
        ConstrainedLeastSquaresProblem {
            objective_matrix: identity,
            objective_rhs: &[0.0, 3.0],
            equalities: Some(equality),
            inequalities: Some(inequalities),
        },
        ConstrainedLeastSquaresOptions::default(),
    )?;
    assert!((fit.solution[0] - 1.0).abs() < 1.0e-12);
    assert!((fit.solution[1] - 2.0).abs() < 1.0e-12);
    assert!(fit.minimum_inequality_slack().unwrap_or(-1.0) >= -1.0e-12);
    Ok(())
}
