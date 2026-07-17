//! Narrow static-link probe for the safe `DBOLS` facade.

use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, VariableBounds,
    solve_bounded_least_squares,
};
use slatec::linear_least_squares::MatrixRef;

fn main() {
    let matrix = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("matrix");
    let result = solve_bounded_least_squares(
        BoundedLeastSquaresProblem {
            matrix,
            rhs: &[2.0],
            bounds: &[VariableBounds::Lower(0.0)],
        },
        BoundedLeastSquaresOptions::default(),
    )
    .expect("bounded fit");
    assert!((result.solution[0] - 2.0).abs() < 1.0e-12);
}
