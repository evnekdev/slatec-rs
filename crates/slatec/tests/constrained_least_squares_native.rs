#![cfg(all(
    feature = "least-squares-linear-constrained-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native numerical validation for reviewed `LSEI` and `DLSEI` calls.
//!
//! The small identity and KKT-equivalent cases have independent analytic
//! solutions. They exercise the safe equality/objective/inequality block
//! construction, status mapping, and scoped XERROR restoration without
//! reimplementing the native active-set method.

use slatec::constrained_least_squares::{
    ConstrainedLeastSquaresError, ConstrainedLeastSquaresOptions, ConstrainedLeastSquaresProblem,
    ConstrainedLeastSquaresStatus, GreaterEqualConstraints, LinearConstraintBlock,
    solve_constrained_least_squares, solve_constrained_least_squares_f32,
};
use slatec::linear_least_squares::MatrixRef;

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn f64_problem<'a>(
    objective_matrix: MatrixRef<'a, f64>,
    objective_rhs: &'a [f64],
    equalities: Option<LinearConstraintBlock<'a, f64>>,
    inequalities: Option<GreaterEqualConstraints<'a, f64>>,
) -> ConstrainedLeastSquaresProblem<'a, f64> {
    ConstrainedLeastSquaresProblem {
        objective_matrix,
        objective_rhs,
        equalities,
        inequalities,
    }
}

#[test]
fn f64_equality_inactive_and_mixed_constraints_match_analytic_references() {
    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1).unwrap(),
        rhs: &[3.0],
    };
    let equality_fit = solve_constrained_least_squares(
        f64_problem(identity, &[1.0, 2.0], Some(equality), None),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(equality_fit.solution[0], 1.0, 2.0e-11);
    close(equality_fit.solution[1], 2.0, 2.0e-11);
    close(equality_fit.equality_residual_norm, 0.0, 2.0e-11);
    close(equality_fit.objective_residual_norm, 0.0, 2.0e-11);
    assert_eq!(
        equality_fit.status,
        ConstrainedLeastSquaresStatus::Converged
    );
    assert_eq!(equality_fit.ranks.equality, 1);

    let equality_only = solve_constrained_least_squares(
        f64_problem(
            MatrixRef::column_major(&[] as &[f64], 0, 1, 0).unwrap(),
            &[],
            Some(LinearConstraintBlock {
                matrix: MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap(),
                rhs: &[3.0],
            }),
            None,
        ),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(equality_only.solution[0], 3.0, 2.0e-11);
    close(equality_only.equality_residual_norm, 0.0, 2.0e-11);

    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let inactive = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, 0.0], 1, 2, 1).unwrap(),
        lower_bounds: &[0.0],
    };
    let inactive_fit = solve_constrained_least_squares(
        f64_problem(identity, &[1.0, 2.0], None, Some(inactive)),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(inactive_fit.solution[0], 1.0, 2.0e-11);
    close(inactive_fit.solution[1], 2.0, 2.0e-11);
    assert!(inactive_fit.minimum_inequality_slack().unwrap() > 0.9);

    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1).unwrap(),
        rhs: &[3.0],
    };
    let inequality = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, 0.0], 1, 2, 1).unwrap(),
        lower_bounds: &[1.0],
    };
    let mixed = solve_constrained_least_squares(
        f64_problem(identity, &[0.0, 3.0], Some(equality), Some(inequality)),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(mixed.solution[0], 1.0, 2.0e-11);
    close(mixed.solution[1], 2.0, 2.0e-11);
    close(mixed.minimum_inequality_slack().unwrap(), 0.0, 2.0e-11);
}

#[test]
fn f64_active_multiple_and_rank_deficient_cases_preserve_native_meaning() {
    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let inequalities = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap(),
        lower_bounds: &[0.0, 0.0],
    };
    let active = solve_constrained_least_squares(
        f64_problem(identity, &[-1.0, -2.0], None, Some(inequalities)),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(active.solution[0], 0.0, 2.0e-11);
    close(active.solution[1], 0.0, 2.0e-11);
    close(active.objective_residual_norm, 5.0_f64.sqrt(), 2.0e-11);

    let equality = LinearConstraintBlock {
        // E = [[1, 0], [2, 0]] has rank one.
        matrix: MatrixRef::column_major(&[1.0_f64, 2.0, 0.0, 0.0], 2, 2, 2).unwrap(),
        rhs: &[1.0, 2.0],
    };
    let objective = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let rank_deficient = solve_constrained_least_squares(
        f64_problem(objective, &[1.0, 0.0], Some(equality), None),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    assert_eq!(rank_deficient.ranks.equality, 1);
    close(rank_deficient.solution[0], 1.0, 2.0e-11);
    close(rank_deficient.solution[1], 0.0, 2.0e-11);

    // The reduced objective has two dependent columns, so its numerical rank
    // is one even though this feasible least-squares problem has solutions.
    let dependent_objective = MatrixRef::column_major(&[1.0_f64, 1.0, 1.0, 1.0], 2, 2, 2).unwrap();
    let reduced_rank = solve_constrained_least_squares(
        f64_problem(dependent_objective, &[1.0, 1.0], None, None),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    assert_eq!(reduced_rank.ranks.reduced_objective, 1);
    close(
        reduced_rank.solution[0] + reduced_rank.solution[1],
        1.0,
        2.0e-10,
    );
    close(reduced_rank.objective_residual_norm, 0.0, 2.0e-10);
}

#[test]
fn f64_native_statuses_and_xerror_state_are_contained() {
    let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    let contradictory_equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 2, 1, 2).unwrap(),
        rhs: &[1.0, 2.0],
    };
    let generalized_inverse = solve_constrained_least_squares(
        f64_problem(objective, &[0.0], Some(contradictory_equality), None),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    assert_eq!(
        generalized_inverse.status,
        ConstrainedLeastSquaresStatus::EqualityConstraintsContradictory
    );
    assert!(generalized_inverse.equality_residual_norm > 0.0);

    let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    let infeasible = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, -1.0], 2, 1, 2).unwrap(),
        lower_bounds: &[1.0, 0.0],
    };
    let mut before_infeasible = 0;
    // SAFETY: reviewed legacy getter takes a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before_infeasible) };
    assert!(matches!(
        solve_constrained_least_squares(
            f64_problem(objective, &[0.0], None, Some(infeasible)),
            ConstrainedLeastSquaresOptions::default(),
        ),
        Err(ConstrainedLeastSquaresError::InequalityConstraintsInfeasible)
    ));
    let mut after_infeasible = 0;
    // SAFETY: reviewed legacy getter takes a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after_infeasible) };
    assert_eq!(
        after_infeasible, before_infeasible,
        "DLSEI must restore XERROR state after an infeasible status"
    );

    let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    let contradictory_equality = LinearConstraintBlock {
        matrix: MatrixRef::column_major(&[1.0_f64, 1.0], 2, 1, 2).unwrap(),
        rhs: &[0.0, 1.0],
    };
    let contradictory_inequalities = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f64, -1.0], 2, 1, 2).unwrap(),
        lower_bounds: &[1.0, 0.0],
    };
    assert!(matches!(
        solve_constrained_least_squares(
            f64_problem(
                objective,
                &[0.0],
                Some(contradictory_equality),
                Some(contradictory_inequalities),
            ),
            ConstrainedLeastSquaresOptions::default(),
        ),
        Err(ConstrainedLeastSquaresError::ConstraintsInfeasible)
    ));

    let mut before = 0;
    // SAFETY: reviewed legacy getter takes a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
    let objective = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    solve_constrained_least_squares(
        f64_problem(objective, &[1.0], None, None),
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    let mut after = 0;
    // SAFETY: reviewed legacy getter takes a valid INTEGER pointer.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(after, before, "DLSEI must restore XERROR state");
}

#[test]
fn f32_active_inequality_matches_the_same_reference() {
    let objective = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let inequality = GreaterEqualConstraints {
        matrix: MatrixRef::column_major(&[1.0_f32, 0.0], 1, 2, 1).unwrap(),
        lower_bounds: &[0.0],
    };
    let fit = solve_constrained_least_squares_f32(
        ConstrainedLeastSquaresProblem {
            objective_matrix: objective,
            objective_rhs: &[-1.0, 2.0],
            equalities: None,
            inequalities: Some(inequality),
        },
        ConstrainedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(f64::from(fit.solution[0]), 0.0, 3.0e-4);
    close(f64::from(fit.solution[1]), 2.0, 3.0e-4);
    close(f64::from(fit.objective_residual_norm), 1.0, 3.0e-4);
}
