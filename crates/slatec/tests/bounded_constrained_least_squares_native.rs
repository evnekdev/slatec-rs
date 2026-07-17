#![cfg(all(
    feature = "least-squares-linear-bounded-constrained-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native validation for reviewed GNU MinGW `SBOCLS` and `DBOCLS` calls.
//!
//! The controlled examples have analytic active-set references. They validate
//! the distinct native model: the constraints are bounds on `C x`, not a
//! generic LP or the independent `DLSEI` inequality interface.

use slatec::bounded_constrained_least_squares::{
    BoundedConstrainedLeastSquaresError, BoundedConstrainedLeastSquaresOptions,
    BoundedConstrainedLeastSquaresProblem, BoundedConstrainedLeastSquaresStatus,
    BoundedLinearConstraints, ConstraintFeasibility, solve_bounded_constrained_least_squares,
    solve_bounded_constrained_least_squares_f32,
};
use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, solve_bounded_least_squares,
};
use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem, LinearConstraintBlock,
    solve_constrained_least_squares,
};
use slatec::linear_least_squares::{MatrixRef, VariableBounds};
use std::sync::Mutex;

// XGETF/XSETF are process-global native state. The safe calls serialize them;
// this test-level gate also keeps direct XGETF assertions out of another
// test's temporary scoped state.
static TEST_RUNTIME: Mutex<()> = Mutex::new(());

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn f64_problem<'a>(
    objective_matrix: MatrixRef<'a, f64>,
    objective_rhs: &'a [f64],
    constraint_matrix: MatrixRef<'a, f64>,
    constraint_bounds: &'a [VariableBounds<f64>],
    variable_bounds: &'a [VariableBounds<f64>],
) -> BoundedConstrainedLeastSquaresProblem<'a, f64> {
    BoundedConstrainedLeastSquaresProblem {
        objective_matrix,
        objective_rhs,
        constraints: BoundedLinearConstraints {
            matrix: constraint_matrix,
            bounds: constraint_bounds,
        },
        variable_bounds,
    }
}

#[test]
fn f64_active_variable_and_linear_form_bounds_match_analytic_reference() {
    let _test = TEST_RUNTIME
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let objective = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    // C = [1, 1], so the fixed form says x0 + x1 = 3.
    let constraints = MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1).unwrap();
    let fit = solve_bounded_constrained_least_squares(
        f64_problem(
            objective,
            &[3.0, 1.0],
            constraints,
            &[VariableBounds::Fixed(3.0)],
            &[VariableBounds::Upper(2.0), VariableBounds::Lower(0.0)],
        ),
        BoundedConstrainedLeastSquaresOptions,
    )
    .unwrap();
    close(fit.solution[0], 2.0, 2.0e-10);
    close(fit.solution[1], 1.0, 2.0e-10);
    close(fit.objective_residual_norm, 1.0, 2.0e-10);
    close(fit.constraint_residual_norm, 0.0, 2.0e-10);
    assert_eq!(fit.constraint_feasibility, ConstraintFeasibility::Feasible);
    assert_eq!(fit.status, BoundedConstrainedLeastSquaresStatus::Converged);
}

#[test]
fn f64_bound_and_constraint_relaxation_are_distinguished_and_xerror_restores() {
    let _test = TEST_RUNTIME
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let one = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    let mut before = 0;
    // SAFETY: reviewed legacy getter receives a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
    let fit = solve_bounded_constrained_least_squares(
        f64_problem(
            one,
            &[0.0],
            one,
            &[VariableBounds::Fixed(1.0)],
            &[VariableBounds::Upper(0.0)],
        ),
        BoundedConstrainedLeastSquaresOptions,
    )
    .unwrap();
    assert_eq!(
        fit.constraint_feasibility,
        ConstraintFeasibility::BoundsRelaxed
    );
    assert!(fit.constraint_residual_norm > 0.9);
    assert!(fit.solution[0] <= 1.0e-10);
    let mut after = 0;
    // SAFETY: reviewed legacy getter receives a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(after, before, "DBOCLS must restore XERROR control");
}

#[test]
fn f32_and_overlap_with_bounded_driver_have_expected_solutions() {
    let _test = TEST_RUNTIME
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let objective = MatrixRef::column_major(&[1.0_f32], 1, 1, 1).unwrap();
    let constraints = MatrixRef::column_major(&[] as &[f32], 0, 1, 0).unwrap();
    let fit = solve_bounded_constrained_least_squares_f32(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[3.0],
            constraints: BoundedLinearConstraints {
                matrix: constraints,
                bounds: &[],
            },
            variable_bounds: &[VariableBounds::Between {
                lower: 0.0,
                upper: 2.0,
            }],
        },
        BoundedConstrainedLeastSquaresOptions,
    )
    .unwrap();
    close(f64::from(fit.solution[0]), 2.0, 4.0e-4);
    close(f64::from(fit.objective_residual_norm), 1.0, 4.0e-4);

    let bounded = solve_bounded_least_squares(
        BoundedLeastSquaresProblem {
            matrix: MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap(),
            rhs: &[3.0],
            bounds: &[VariableBounds::Between {
                lower: 0.0,
                upper: 2.0,
            }],
        },
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(f64::from(fit.solution[0]), bounded.solution[0], 4.0e-4);

    let invalid = solve_bounded_constrained_least_squares_f32(
        BoundedConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[1.0],
            constraints: BoundedLinearConstraints {
                matrix: MatrixRef::column_major(&[1.0_f32], 1, 1, 1).unwrap(),
                bounds: &[VariableBounds::Between {
                    lower: 2.0,
                    upper: 1.0,
                }],
            },
            variable_bounds: &[VariableBounds::Unbounded],
        },
        BoundedConstrainedLeastSquaresOptions,
    );
    assert!(matches!(
        invalid,
        Err(BoundedConstrainedLeastSquaresError::InconsistentBounds {
            argument: "constraints.bounds",
            index: 0
        })
    ));
}

#[test]
fn equality_only_overlap_matches_dlsei() {
    let _test = TEST_RUNTIME
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let objective = MatrixRef::column_major(&[] as &[f64], 0, 1, 0).unwrap();
    let constraint = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    let bocl = solve_bounded_constrained_least_squares(
        f64_problem(
            objective,
            &[],
            constraint,
            &[VariableBounds::Fixed(1.0)],
            &[VariableBounds::Unbounded],
        ),
        BoundedConstrainedLeastSquaresOptions,
    )
    .unwrap();
    let lsei = solve_constrained_least_squares(
        ConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[],
            equalities: Some(LinearConstraintBlock {
                matrix: constraint,
                rhs: &[1.0],
            }),
            inequalities: None,
        },
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(bocl.solution[0], lsei.solution[0], 2.0e-10);
    close(
        bocl.objective_residual_norm,
        lsei.objective_residual_norm,
        2.0e-10,
    );
}
