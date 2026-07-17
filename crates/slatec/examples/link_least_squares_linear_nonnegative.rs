//! Narrow-link probe for the safe `DWNNLS` facade.
//!
//! This is intentionally not executed. It proves normal archive extraction for
//! the `least-squares-linear-nonnegative` family without retaining unrelated
//! safe-family entry points.

use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint, solve_nonnegative_least_squares,
};

fn main() {
    let matrix = MatrixRef::column_major(&[1.0_f64, 1.0], 2, 1, 2).unwrap();
    let _ = solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: &[1.0, 1.0],
        equality_matrix: None,
        equality_rhs: None,
        variable_constraints: &[VariableConstraint::Nonnegative],
    });
}
