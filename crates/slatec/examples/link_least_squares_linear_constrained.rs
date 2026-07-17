//! Narrow-link probe for the safe `DLSEI` wrapper.

use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem, solve_constrained_least_squares,
};
use slatec::linear_least_squares::MatrixRef;

fn main() {
    let objective =
        MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("checked one-by-one objective");
    let fit = solve_constrained_least_squares(
        ConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[1.0],
            equalities: None,
            inequalities: None,
        },
        ConstrainedLeastSquaresOptions::default(),
    )
    .expect("DLSEI solve");
    assert!((fit.solution[0] - 1.0).abs() < 1.0e-12);
}
