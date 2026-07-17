//! Fit a dense model while enforcing an exact linear equality with `DLSEI`.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-constrained`. This is constrained least squares, not
//! linear programming: it minimizes an objective residual after satisfying
//! `E x = f`.

// slatec-safe-example: equality constrained linear least squares

use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem, LinearConstraintBlock,
    solve_constrained_least_squares,
};
use slatec::linear_least_squares::MatrixRef;

fn main() -> Result<(), slatec::constrained_least_squares::ConstrainedLeastSquaresError> {
    // min ||x - [1, 2]|| subject to x_0 + x_1 = 3.
    let objective = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1)?,
        rhs: &[3.0],
    };
    let fit = solve_constrained_least_squares(
        ConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[1.0, 2.0],
            equalities: Some(equality),
            inequalities: None,
        },
        ConstrainedLeastSquaresOptions::default(),
    )?;
    assert!((fit.solution[0] - 1.0).abs() < 1.0e-12);
    assert!((fit.solution[1] - 2.0).abs() < 1.0e-12);
    assert!(fit.equality_residual_norm < 1.0e-12);
    assert!(fit.objective_residual_norm < 1.0e-12);
    Ok(())
}
