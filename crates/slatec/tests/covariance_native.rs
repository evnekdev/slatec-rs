#![cfg(all(
    feature = "least-squares-covariance-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Narrow native execution tests for reviewed GNU MinGW `SCOV` and `DCOV`.

use slatec::least_squares::{
    CovarianceOptions, estimate_covariance, estimate_covariance_f32,
    estimate_covariance_finite_difference, estimate_covariance_finite_difference_f32,
};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

#[test]
fn scov_and_dcov_execute_against_their_narrow_source_closure() {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.1, 2.9, 5.2, 7.1, 8.8];
    let analytic = estimate_covariance(
        &[1.1, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x).unwrap();
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    close(analytic.covariance[0], 0.0184, 3.0e-11);

    let finite_difference = estimate_covariance_finite_difference(
        &[1.1, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    close(
        finite_difference.covariance[0],
        analytic.covariance[0],
        2.0e-8,
    );

    let single = estimate_covariance_f32(
        &[1.1_f32, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x as f32 - y as f32;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x as f32).unwrap();
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    assert_eq!(single.rank, 2);
    assert!(
        estimate_covariance_finite_difference_f32(
            &[1.1_f32, 1.96],
            xs.len(),
            |parameters, residuals| {
                for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                    *residual = parameters[0] + parameters[1] * x as f32 - y as f32;
                }
            },
            CovarianceOptions::default(),
        )
        .is_ok()
    );
}
