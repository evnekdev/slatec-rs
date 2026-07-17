//! Solve an equality-constrained nonnegative `DWNNLS` problem.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-nonnegative`. Original SLATEC routine: `DWNNLS`.

// slatec-safe-example: equality constrained nonnegative linear least squares

use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint,
    solve_nonnegative_least_squares,
};

fn main() -> Result<(), slatec::linear_least_squares::LinearLeastSquaresError> {
    let least = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let equality = MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1)?;
    let fit = solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
        least_squares_matrix: least,
        least_squares_rhs: &[0.25, 0.75],
        equality_matrix: Some(equality),
        equality_rhs: Some(&[1.0]),
        variable_constraints: &[VariableConstraint::Nonnegative, VariableConstraint::Nonnegative],
    })?;
    assert!(fit.equality_residual_norm < 1.0e-10);
    assert!(fit.solution.iter().all(|value| *value >= -1.0e-12));
    Ok(())
}
