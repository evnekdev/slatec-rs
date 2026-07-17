#![cfg(all(
    feature = "least-squares-linear-nonnegative-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native validation for the reviewed GNU MinGW `WNNLS` and `DWNNLS` drivers.
//!
//! The cases use immutable caller matrices and independently obvious small
//! active sets. They exercise the safe augmented-array conversion, native
//! free/nonnegative partition, and equality-row ordering rather than a Rust
//! implementation of the Lawson--Hanson algorithm.

use slatec::linear_least_squares::{
    LinearLeastSquaresError, MatrixRef, NonnegativeLeastSquaresProblem,
    NonnegativeLeastSquaresStatus, VariableConstraint, solve_nonnegative_least_squares,
    solve_nonnegative_least_squares_f32,
};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn problem_f64<'a>(
    matrix: MatrixRef<'a, f64>,
    rhs: &'a [f64],
    equality_matrix: Option<MatrixRef<'a, f64>>,
    equality_rhs: Option<&'a [f64]>,
    constraints: &'a [VariableConstraint],
) -> NonnegativeLeastSquaresProblem<'a, f64> {
    NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: rhs,
        equality_matrix,
        equality_rhs,
        variable_constraints: constraints,
    }
}

#[test]
fn f64_boundary_interior_mixed_and_all_free_cases_match_small_references() {
    let identity = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let boundary = solve_nonnegative_least_squares(problem_f64(
        identity,
        &[-1.0, 2.0],
        None,
        None,
        &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Nonnegative,
        ],
    ))
    .unwrap();
    close(boundary.solution[0], 0.0, 1.0e-11);
    close(boundary.solution[1], 2.0, 1.0e-11);
    close(boundary.least_squares_residual_norm, 1.0, 1.0e-11);
    assert_eq!(boundary.status, NonnegativeLeastSquaresStatus::Converged);

    let interior = solve_nonnegative_least_squares(problem_f64(
        identity,
        &[1.0, 2.0],
        None,
        None,
        &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Nonnegative,
        ],
    ))
    .unwrap();
    close(interior.solution[0], 1.0, 1.0e-11);
    close(interior.solution[1], 2.0, 1.0e-11);
    close(interior.least_squares_residual_norm, 0.0, 1.0e-11);

    let mixed_matrix =
        MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0], 3, 3, 3)
            .unwrap();
    let mixed = solve_nonnegative_least_squares(problem_f64(
        mixed_matrix,
        &[2.0, -1.0, 3.0],
        None,
        None,
        &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Free,
            VariableConstraint::Nonnegative,
        ],
    ))
    .unwrap();
    close(mixed.solution[0], 2.0, 1.0e-11);
    close(mixed.solution[1], -1.0, 1.0e-11);
    close(mixed.solution[2], 3.0, 1.0e-11);

    let all_free = solve_nonnegative_least_squares(problem_f64(
        identity,
        &[-2.0, 3.0],
        None,
        None,
        &[VariableConstraint::Free, VariableConstraint::Free],
    ))
    .unwrap();
    close(all_free.solution[0], -2.0, 1.0e-11);
    close(all_free.solution[1], 3.0, 1.0e-11);
}

#[test]
fn f64_equality_and_rank_deficient_cases_have_documented_behavior() {
    let least = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let equality = MatrixRef::column_major(&[1.0_f64, 1.0], 1, 2, 1).unwrap();
    let fit = solve_nonnegative_least_squares(problem_f64(
        least,
        &[0.25, 0.75],
        Some(equality),
        Some(&[1.0]),
        &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Nonnegative,
        ],
    ))
    .unwrap();
    close(fit.solution[0], 0.25, 1.0e-10);
    close(fit.solution[1], 0.75, 1.0e-10);
    close(fit.equality_residual_norm, 0.0, 1.0e-10);

    let rank_deficient = MatrixRef::column_major(&[1.0_f64, 1.0, 1.0, 1.0], 2, 2, 2).unwrap();
    let rank_fit = solve_nonnegative_least_squares(problem_f64(
        rank_deficient,
        &[1.0, 1.0],
        None,
        None,
        &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Nonnegative,
        ],
    ))
    .unwrap();
    close(rank_fit.solution[0] + rank_fit.solution[1], 1.0, 1.0e-10);
    close(rank_fit.least_squares_residual_norm, 0.0, 1.0e-10);
}

#[test]
fn f32_and_validation_cases_are_contained_before_native_execution() {
    let matrix = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2).unwrap();
    let fit = solve_nonnegative_least_squares_f32(NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: &[-1.0, 2.0],
        equality_matrix: None,
        equality_rhs: None,
        variable_constraints: &[
            VariableConstraint::Nonnegative,
            VariableConstraint::Nonnegative,
        ],
    })
    .unwrap();
    close(f64::from(fit.solution[0]), 0.0, 2.0e-4);
    close(f64::from(fit.solution[1]), 2.0, 2.0e-4);

    let one = MatrixRef::column_major(&[1.0_f64], 1, 1, 1).unwrap();
    assert!(matches!(
        solve_nonnegative_least_squares(problem_f64(
            one,
            &[f64::NAN],
            None,
            None,
            &[VariableConstraint::Nonnegative]
        )),
        Err(LinearLeastSquaresError::NonFiniteInput { .. })
    ));
}
