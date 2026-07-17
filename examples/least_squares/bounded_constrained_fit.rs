//! Fit a scalar with a bounded linear expression using SLATEC `DBOCLS`.
//!
//! Run with `std,source-build,least-squares-linear-bounded-constrained` on
//! the validated GNU MinGW target.

// slatec-safe-example: bounded constrained linear least squares

use slatec::bounded_constrained_least_squares::{
    BoundedConstrainedLeastSquaresOptions, BoundedConstrainedLeastSquaresProblem,
    BoundedLinearConstraints, ConstraintFeasibility, solve_bounded_constrained_least_squares,
};
use slatec::linear_least_squares::{MatrixRef, VariableBounds};

fn main() {
    let objective = MatrixRef::column_major(&[1.0_f64, 1.0], 2, 1, 2).expect("objective");
    let constraints = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).expect("constraint");
    let result = solve_bounded_constrained_least_squares(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[2.0, 2.0],
            constraints: BoundedLinearConstraints {
                matrix: constraints,
                bounds: &[VariableBounds::Fixed(1.0)],
            },
            variable_bounds: &[VariableBounds::Between {
                lower: 0.0,
                upper: 3.0,
            }],
        },
        BoundedConstrainedLeastSquaresOptions,
    )
    .expect("DBOCLS fit");

    assert!((result.solution[0] - 1.0).abs() < 1.0e-10);
    assert!(result.objective_residual_norm > 1.3);
    assert_eq!(result.constraint_feasibility, ConstraintFeasibility::Feasible);
}
