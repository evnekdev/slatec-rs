#![cfg(all(
    feature = "approximation-polynomial-fitting",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native regressions for reviewed `POLFIT`/`DPOLFT` polynomial fitting.

use slatec::interpolation::approximation::{
    PolynomialDegreeSelection, PolynomialFit, PolynomialFitError, PolynomialFitOptions,
    PolynomialFitStatus, StatisticalSignificance,
};
use slatec::native_lock_test_support::with_native_lock;

fn quadratic(x: f64) -> f64 {
    1.0 + 2.0 * x + 0.5 * x * x
}

#[test]
fn double_precision_fit_evaluates_derivatives_and_power_coefficients() {
    let abscissas = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let values = abscissas.map(quadratic);
    let fit = PolynomialFit::<f64>::fit(&abscissas, &values, PolynomialFitOptions::all_degrees(2))
        .unwrap();

    assert_eq!(fit.degree(), 2);
    assert_eq!(fit.status(), &PolynomialFitStatus::Complete);
    assert!(fit.rms_error() < 1.0e-12);
    assert!(fit.weighted_residual_sum_squares() < 1.0e-22);
    let evaluation = fit.evaluate_with_derivatives(0.5, 2).unwrap();
    assert!((evaluation.value - quadratic(0.5)).abs() < 1.0e-11);
    assert!((evaluation.derivatives[0] - 2.5).abs() < 1.0e-10);
    assert!((evaluation.derivatives[1] - 1.0).abs() < 1.0e-10);
    let coefficients = fit.power_coefficients().unwrap();
    assert!((coefficients[0] - 1.0).abs() < 1.0e-10);
    assert!((coefficients[1] - 2.0).abs() < 1.0e-10);
    assert!((coefficients[2] - 0.5).abs() < 1.0e-10);
    let centered = fit.power_coefficients_at(1.5).unwrap();
    assert!((centered[0] - quadratic(1.5)).abs() < 1.0e-10);
    assert!((centered[1] - 3.5).abs() < 1.0e-10);
    assert!((centered[2] - 0.5).abs() < 1.0e-10);
}

#[test]
fn single_precision_weighted_fit_and_batch_evaluation_are_owned() {
    let abscissas = [-2.0_f32, -1.0, 0.0, 1.0, 2.0];
    let values = [-1.0_f32, -0.5, 1.0, 3.5, 7.0];
    let weights = [1.0_f32, 2.0, 3.0, 2.0, 1.0];
    let fit = PolynomialFit::<f32>::fit_weighted(
        &abscissas,
        &values,
        &weights,
        PolynomialFitOptions::rms_tolerance(2, 1.0e-5),
    )
    .unwrap();

    assert_eq!(fit.degree(), 2);
    assert_eq!(fit.status(), &PolynomialFitStatus::Complete);
    let points = [-0.5_f32, 0.5, 1.5];
    let mut output = [0.0_f32; 3];
    fit.evaluate_into(&points, &mut output).unwrap();
    assert!((output[0] - 0.125).abs() < 5.0e-4);
    assert!((output[1] - 2.125).abs() < 5.0e-4);
    assert!((output[2] - 5.125).abs() < 5.0e-4);
}

#[test]
fn source_statuses_keep_documented_best_fit() {
    let abscissas = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let values = [-1.0_f64, -0.5, 1.0, 3.5, 7.0];
    let tolerance = PolynomialFit::<f64>::fit(
        &abscissas,
        &values,
        PolynomialFitOptions::rms_tolerance(1, 1.0e-16),
    )
    .unwrap();
    assert!(matches!(
        tolerance.status(),
        PolynomialFitStatus::ToleranceNotReached { .. }
    ));
    assert_eq!(tolerance.degree(), 1);

    let statistical = PolynomialFit::<f64>::fit(
        &abscissas,
        &values,
        PolynomialFitOptions::statistical_test(2, StatisticalSignificance::FivePercent),
    )
    .unwrap();
    assert!(matches!(
        statistical.selection(),
        PolynomialDegreeSelection::StatisticalFTest(StatisticalSignificance::FivePercent)
    ));
}

#[test]
fn safe_fit_matches_the_reviewed_double_precision_raw_workflow() {
    let abscissas = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let values = abscissas.map(quadratic);
    let safe = PolynomialFit::<f64>::fit(&abscissas, &values, PolynomialFitOptions::all_degrees(2))
        .unwrap();

    let mut raw_x = abscissas;
    let mut raw_y = values;
    let mut raw_weights = [-1.0_f64; 5];
    let mut count = 5;
    let mut max_degree = 2;
    let mut selected_degree = 0;
    let mut rms_error = 0.0;
    let mut fitted = [0.0_f64; 5];
    let mut status = 0;
    let mut representation = [0.0_f64; 24]; // 3*N + 3*MAXDEG + 3.
    let mut query_degree = 2;
    let mut derivative_count = 0;
    let mut point = 0.5;
    let mut raw_value = 0.0;
    let mut derivatives = [0.0_f64; 1];
    with_native_lock(|| {
        // SAFETY: every scalar and private array satisfies the exact source
        // contract for the same f64 workflow exercised by the safe facade.
        unsafe {
            slatec_sys::approximation::dpolft(
                &mut count,
                raw_x.as_mut_ptr(),
                raw_y.as_mut_ptr(),
                raw_weights.as_mut_ptr(),
                &mut max_degree,
                &mut selected_degree,
                &mut rms_error,
                fitted.as_mut_ptr(),
                &mut status,
                representation.as_mut_ptr(),
            );
            slatec_sys::approximation::dp1vlu(
                &mut query_degree,
                &mut derivative_count,
                &mut point,
                &mut raw_value,
                derivatives.as_mut_ptr(),
                representation.as_mut_ptr(),
            );
        }
    });
    assert_eq!(status, 1);
    assert_eq!(selected_degree, 2);
    assert!((safe.evaluate(0.5).unwrap() - raw_value).abs() < 1.0e-13);
}

#[test]
fn preflight_rejects_invalid_values_weights_and_degree_rules() {
    assert_eq!(
        PolynomialFit::<f64>::fit(&[], &[], PolynomialFitOptions::all_degrees(0)),
        Err(PolynomialFitError::TooFewSamples)
    );
    assert_eq!(
        PolynomialFit::<f64>::fit(
            &[0.0, f64::NAN],
            &[0.0, 1.0],
            PolynomialFitOptions::all_degrees(1),
        ),
        Err(PolynomialFitError::NonFiniteAbscissa { index: 1 })
    );
    assert_eq!(
        PolynomialFit::<f64>::fit_weighted(
            &[0.0, 1.0],
            &[0.0, 1.0],
            &[1.0, 0.0],
            PolynomialFitOptions::all_degrees(1),
        ),
        Err(PolynomialFitError::InvalidWeight { index: 1 })
    );
    assert_eq!(
        PolynomialFit::<f64>::fit(
            &[0.0, 1.0],
            &[0.0, 1.0],
            PolynomialFitOptions::statistical_test(1, StatisticalSignificance::OnePercent),
        ),
        Err(PolynomialFitError::InvalidMaximumDegree {
            samples: 2,
            max_degree: 1,
            statistical_test: true,
        })
    );
    let fit = PolynomialFit::<f64>::fit(
        &[0.0, 1.0],
        &[0.0, 1.0],
        PolynomialFitOptions::all_degrees(1),
    )
    .unwrap();
    assert_eq!(
        fit.power_coefficients_at(f64::NAN),
        Err(PolynomialFitError::NonFiniteExpansionOrigin)
    );
}
