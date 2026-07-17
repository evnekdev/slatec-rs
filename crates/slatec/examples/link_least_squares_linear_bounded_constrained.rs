//! Narrow static-link probe for the safe `DBOCLS` facade.

use slatec::bounded_constrained_least_squares::{
    BoundedConstrainedLeastSquaresOptions, BoundedConstrainedLeastSquaresProblem,
    BoundedLinearConstraints, solve_bounded_constrained_least_squares,
};
use slatec::linear_least_squares::{MatrixRef, VariableBounds};

fn main() {
    let objective =
        MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("checked one-by-one objective");
    let constraints =
        MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("checked one-by-one constraint");
    let fit = solve_bounded_constrained_least_squares(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[1.0],
            constraints: BoundedLinearConstraints {
                matrix: constraints,
                bounds: &[VariableBounds::Fixed(1.0)],
            },
            variable_bounds: &[VariableBounds::Lower(0.0)],
        },
        BoundedConstrainedLeastSquaresOptions,
    )
    .expect("bounded constrained fit");
    assert!((fit.solution[0] - 1.0).abs() < 1.0e-12);
}
