//! Solve a boundary nonnegative linear least-squares fit with `DWNNLS`.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-nonnegative`. This is constrained linear least
//! squares, not linear programming; it minimizes a squared residual.

// slatec-safe-example: nonnegative linear least squares

use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint,
    solve_nonnegative_least_squares,
};

fn main() -> Result<(), slatec::linear_least_squares::LinearLeastSquaresError> {
    // A = [[1, 1], [1, -1]], b = [1, 0]. Both variables are nonnegative.
    let matrix = MatrixRef::column_major(&[1.0_f64, 1.0, 1.0, -1.0], 2, 2, 2)?;
    let fit = solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: &[1.0, 0.0],
        equality_matrix: None,
        equality_rhs: None,
        variable_constraints: &[VariableConstraint::Nonnegative, VariableConstraint::Nonnegative],
    })?;
    assert!(fit.solution[0] >= -1.0e-12);
    assert!(fit.solution[1] >= -1.0e-12);
    assert!(fit.least_squares_residual_norm < 1.0e-10);
    Ok(())
}
