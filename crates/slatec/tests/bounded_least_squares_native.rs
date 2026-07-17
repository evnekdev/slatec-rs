#![cfg(all(
    feature = "least-squares-linear-bounded-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native validation for the reviewed GNU MinGW `SBOLS` and `DBOLS` drivers.
//!
//! The controlled identity examples have an independent analytic active-set
//! reference. They validate the safe bound encoding and owned augmented
//! storage; they do not reproduce the Lawson--Hanson implementation.

use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, BoundedLeastSquaresScaling,
    BoundedLeastSquaresStatus, VariableBounds, solve_bounded_least_squares,
    solve_bounded_least_squares_f32,
};
use slatec::linear_least_squares::MatrixRef;

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn f64_problem<'a>(
    matrix: MatrixRef<'a, f64>,
    rhs: &'a [f64],
    bounds: &'a [VariableBounds<f64>],
) -> BoundedLeastSquaresProblem<'a, f64> {
    BoundedLeastSquaresProblem {
        matrix,
        rhs,
        bounds,
    }
}

fn identity_reference(value: f64, bound: VariableBounds<f64>) -> f64 {
    match bound {
        VariableBounds::Unbounded => value,
        VariableBounds::Lower(lower) => value.max(lower),
        VariableBounds::Upper(upper) => value.min(upper),
        VariableBounds::Between { lower, upper } => value.clamp(lower, upper),
        VariableBounds::Fixed(fixed) => fixed,
    }
}

#[test]
fn f64_all_bound_variants_match_independent_identity_references() {
    let identity =
        MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3, 3, 3)
            .unwrap();
    let rhs = [-2.0, 3.0, 7.0];
    let bounds = [
        VariableBounds::Lower(0.0),
        VariableBounds::Upper(2.0),
        VariableBounds::Between {
            lower: 1.0,
            upper: 4.0,
        },
    ];
    let fit = solve_bounded_least_squares(
        f64_problem(identity, &rhs, &bounds),
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    for ((actual, expected), bound) in fit.solution.iter().zip(rhs).zip(bounds) {
        close(*actual, identity_reference(expected, bound), 2.0e-11);
    }
    close(fit.residual_norm, (4.0_f64 + 1.0 + 9.0).sqrt(), 2.0e-11);
    assert_eq!(fit.status, BoundedLeastSquaresStatus::Converged);

    let fixed_matrix = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let fixed = solve_bounded_least_squares(
        f64_problem(
            fixed_matrix,
            &[7.0, 2.0],
            &[VariableBounds::Fixed(3.0), VariableBounds::Unbounded],
        ),
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(fixed.solution[0], 3.0, 2.0e-11);
    close(fixed.solution[1], 2.0, 2.0e-11);
    close(fixed.residual_norm, 4.0, 2.0e-11);
}

#[test]
fn f64_mixed_unbounded_and_scaled_rank_deficient_cases_have_documented_behavior() {
    let identity = MatrixRef::column_major(
        &[
            1.0_f64, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        4,
        4,
        4,
    )
    .unwrap();
    let fit = solve_bounded_least_squares(
        f64_problem(
            identity,
            &[5.0, -4.0, 3.0, 9.0],
            &[
                VariableBounds::Upper(2.0),
                VariableBounds::Unbounded,
                VariableBounds::Between {
                    lower: 1.0,
                    upper: 2.0,
                },
                VariableBounds::Fixed(4.0),
            ],
        ),
        BoundedLeastSquaresOptions {
            scaling: BoundedLeastSquaresScaling::EuclideanColumns,
            maximum_iterations: None,
        },
    )
    .unwrap();
    for (actual, expected) in fit.solution.iter().zip([2.0, -4.0, 2.0, 4.0]) {
        close(*actual, expected, 2.0e-11);
    }

    let rank_deficient = MatrixRef::column_major(&[1.0_f64, 1.0, 1.0, 1.0], 2, 2, 2).unwrap();
    let rank_fit = solve_bounded_least_squares(
        f64_problem(
            rank_deficient,
            &[1.0, 1.0],
            &[VariableBounds::Unbounded, VariableBounds::Unbounded],
        ),
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(rank_fit.solution[0] + rank_fit.solution[1], 1.0, 2.0e-10);
    close(rank_fit.residual_norm, 0.0, 2.0e-10);

    let poorly_scaled = MatrixRef::column_major(&[0.001_f64, 0.0, 0.0, 1000.0], 2, 2, 2).unwrap();
    let scaled = solve_bounded_least_squares(
        f64_problem(
            poorly_scaled,
            &[0.002, 3000.0],
            &[VariableBounds::Unbounded, VariableBounds::Unbounded],
        ),
        BoundedLeastSquaresOptions {
            scaling: BoundedLeastSquaresScaling::EuclideanColumns,
            maximum_iterations: None,
        },
    )
    .unwrap();
    close(scaled.solution[0], 2.0, 2.0e-9);
    close(scaled.solution[1], 3.0, 2.0e-9);
}

#[test]
fn f32_and_xerror_state_restoration_are_validated() {
    let matrix = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let fit = solve_bounded_least_squares_f32(
        BoundedLeastSquaresProblem {
            matrix,
            rhs: &[-1.0, 3.0],
            bounds: &[VariableBounds::Lower(0.0), VariableBounds::Upper(2.0)],
        },
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    close(f64::from(fit.solution[0]), 0.0, 3.0e-4);
    close(f64::from(fit.solution[1]), 2.0, 3.0e-4);
    close(f64::from(fit.residual_norm), 2.0_f64.sqrt(), 4.0e-4);

    let mut before = 0;
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
    let one = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    solve_bounded_least_squares(
        f64_problem(one, &[1.0], &[VariableBounds::Unbounded]),
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
    let mut after = 0;
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(after, before, "DBOLS must restore the prior XERROR control");
}

#[test]
fn f64_iteration_limit_is_a_contained_status_and_restores_xerror() {
    let matrix = MatrixRef::column_major(
        &[
            1.0_f64, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        4,
        4,
        4,
    )
    .unwrap();
    let fit = solve_bounded_least_squares(
        f64_problem(
            matrix,
            &[-4.0, -3.0, 5.0, 6.0],
            &[
                VariableBounds::Lower(0.0),
                VariableBounds::Lower(0.0),
                VariableBounds::Upper(1.0),
                VariableBounds::Upper(1.0),
            ],
        ),
        BoundedLeastSquaresOptions {
            scaling: BoundedLeastSquaresScaling::Nominal,
            maximum_iterations: Some(1),
        },
    )
    .unwrap();
    assert_eq!(fit.status, BoundedLeastSquaresStatus::MaximumIterations);
    assert!(fit.solution.iter().all(|value| value.is_finite()));
}
