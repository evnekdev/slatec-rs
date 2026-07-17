//! Solve a mixed free/nonnegative `WNNLS` problem with a caller order that
//! differs from the native partition.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-nonnegative`.

// slatec-safe-example: mixed nonnegative linear least squares

use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint,
    solve_nonnegative_least_squares_f32,
};

fn main() -> Result<(), slatec::linear_least_squares::LinearLeastSquaresError> {
    let matrix = MatrixRef::column_major(&[1.0_f32, 1.0, 1.0, -1.0], 2, 2, 2)?;
    let fit = solve_nonnegative_least_squares_f32(NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: &[1.0, 0.0],
        equality_matrix: None,
        equality_rhs: None,
        variable_constraints: &[VariableConstraint::Nonnegative, VariableConstraint::Free],
    })?;
    assert!(fit.solution[0] >= -1.0e-4);
    assert!(fit.least_squares_residual_norm < 1.0e-3);
    Ok(())
}
