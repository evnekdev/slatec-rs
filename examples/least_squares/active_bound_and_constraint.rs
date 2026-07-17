//! A `DBOCLS` fit with a simultaneously active variable bound and constraint.
//!
//! Requires `std,source-build,least-squares-linear-bounded-constrained`.

// slatec-safe-example: bounded constrained linear least squares f32

use slatec::bounded_constrained_least_squares::{
    BoundedConstrainedLeastSquaresOptions, BoundedConstrainedLeastSquaresProblem,
    BoundedLinearConstraints, solve_bounded_constrained_least_squares,
};
use slatec::linear_least_squares::{MatrixRef, VariableBounds};

fn main() {
    let objective = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2)
        .expect("objective");
    let constraints = MatrixRef::column_major(&[1.0_f32, 1.0, 0.0, 1.0], 1, 2, 1)
        .expect("constraint");
    let result = solve_bounded_constrained_least_squares_f32(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[3.0, 1.0],
            constraints: BoundedLinearConstraints {
                matrix: constraints,
                bounds: &[VariableBounds::Fixed(3.0)],
            },
            variable_bounds: &[VariableBounds::Upper(2.0), VariableBounds::Lower(0.0)],
        },
        BoundedConstrainedLeastSquaresOptions,
    )
    .expect("SBOCLS fit");

    assert!((result.solution[0] - 2.0).abs() < 3.0e-4);
    assert!((result.solution[1] - 1.0).abs() < 3.0e-4);
}

use slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32;
