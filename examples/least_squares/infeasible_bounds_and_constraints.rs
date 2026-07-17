//! Inspect `DBOCLS` constraint-bound relaxation instead of treating it as exact feasibility.
//!
//! Requires `std,source-build,least-squares-linear-bounded-constrained`.

// slatec-safe-example: bounded constrained linear least squares relaxed constraints

use slatec::bounded_constrained_least_squares::{
    BoundedConstrainedLeastSquaresOptions, BoundedConstrainedLeastSquaresProblem,
    BoundedLinearConstraints, ConstraintFeasibility, solve_bounded_constrained_least_squares,
};
use slatec::linear_least_squares::{MatrixRef, VariableBounds};

fn main() {
    let one = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("matrix");
    let result = solve_bounded_constrained_least_squares(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: one,
            objective_rhs: &[0.0],
            constraints: BoundedLinearConstraints {
                matrix: one,
                bounds: &[VariableBounds::Fixed(1.0)],
            },
            variable_bounds: &[VariableBounds::Upper(0.0)],
        },
        BoundedConstrainedLeastSquaresOptions,
    )
    .expect("the default DBOCLS filter returns its relaxation diagnostic");

    assert_eq!(result.constraint_feasibility, ConstraintFeasibility::BoundsRelaxed);
    assert!(result.constraint_residual_norm > 0.9);
}
