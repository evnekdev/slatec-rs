#![cfg(feature = "pchip-native-tests")]

use slatec::pchip::{
    EndpointCondition, Extrapolation, MonotoneEndpointCondition, PchipError, PiecewiseCubicHermite,
    SwitchPointPolicy,
};

fn cubic_f32(x: f32) -> f32 {
    x * x * x - 2.0 * x + 1.0
}

fn cubic_derivative_f32(x: f32) -> f32 {
    3.0 * x * x - 2.0
}

fn cubic_integral_f32(x: f32) -> f32 {
    0.25 * x.powi(4) - x * x + x
}

fn cubic_f64(x: f64) -> f64 {
    x * x * x - 2.0 * x + 1.0
}

fn cubic_derivative_f64(x: f64) -> f64 {
    3.0 * x * x - 2.0
}

fn cubic_integral_f64(x: f64) -> f64 {
    0.25 * x.powi(4) - x * x + x
}

fn assert_close_f32(actual: f32, expected: f32, tolerance: f32) {
    let scale = 1.0_f32.max(actual.abs()).max(expected.abs());
    assert!(
        (actual - expected).abs() <= tolerance * scale,
        "{actual} != {expected}"
    );
}

fn assert_close_f64(actual: f64, expected: f64, tolerance: f64) {
    let scale = 1.0_f64.max(actual.abs()).max(expected.abs());
    assert!(
        (actual - expected).abs() <= tolerance * scale,
        "{actual} != {expected}"
    );
}

#[test]
fn f32_exact_cubic_evaluation_derivative_and_integral() {
    let knots = [-2.0_f32, -0.5, 0.25, 1.75, 3.0];
    let values = knots.map(cubic_f32);
    let derivatives = knots.map(cubic_derivative_f32);
    let curve = PiecewiseCubicHermite::from_derivatives(
        knots.to_vec(),
        values.to_vec(),
        derivatives.to_vec(),
    )
    .unwrap();
    let points = [-2.0, -1.25, -0.1, 0.7, 2.5, 3.0];
    let mut output = [0.0; 6];
    let mut derivative_output = [0.0; 6];
    assert_eq!(
        curve
            .evaluate_with_derivatives_into(&points, &mut output, &mut derivative_output)
            .unwrap()
            .extrapolated_points,
        0
    );
    for index in 0..points.len() {
        assert_close_f32(output[index], cubic_f32(points[index]), 4.0e-5);
        assert_close_f32(
            derivative_output[index],
            cubic_derivative_f32(points[index]),
            7.0e-5,
        );
    }
    let integral = curve.integrate(-1.25, 2.5).unwrap();
    assert_close_f32(
        integral,
        cubic_integral_f32(2.5) - cubic_integral_f32(-1.25),
        7.0e-5,
    );
    assert_close_f32(curve.integrate(2.5, -1.25).unwrap(), -integral, 7.0e-5);
}

#[test]
fn f64_exact_cubic_evaluation_derivative_and_integral() {
    let knots = [-2.0_f64, -0.5, 0.25, 1.75, 3.0];
    let values = knots.map(cubic_f64);
    let derivatives = knots.map(cubic_derivative_f64);
    let curve = PiecewiseCubicHermite::from_derivatives(
        knots.to_vec(),
        values.to_vec(),
        derivatives.to_vec(),
    )
    .unwrap();
    let points = [-2.0, -1.25, -0.1, 0.7, 2.5, 3.0];
    let mut output = [0.0; 6];
    let mut derivative_output = [0.0; 6];
    curve
        .evaluate_with_derivatives_into(&points, &mut output, &mut derivative_output)
        .unwrap();
    for index in 0..points.len() {
        assert_close_f64(output[index], cubic_f64(points[index]), 2.0e-12);
        assert_close_f64(
            derivative_output[index],
            cubic_derivative_f64(points[index]),
            4.0e-12,
        );
    }
    let integral = curve.integrate(-1.25, 2.5).unwrap();
    assert_close_f64(
        integral,
        cubic_integral_f64(2.5) - cubic_integral_f64(-1.25),
        3.0e-12,
    );
    assert_close_f64(curve.integrate(2.5, -1.25).unwrap(), -integral, 3.0e-12);
}

#[test]
fn monotone_pchip_has_no_interval_overshoot_in_both_precisions() {
    let knots_f32 = [0.0_f32, 0.1, 1.0, 3.5, 4.0];
    let values_f32 = [0.0_f32, 0.02, 0.8, 0.95, 2.0];
    let curve_f32 = PiecewiseCubicHermite::monotone(&knots_f32, &values_f32).unwrap();
    let knots_f64 = [0.0_f64, 0.1, 1.0, 3.5, 4.0];
    let values_f64 = [0.0_f64, 0.02, 0.8, 0.95, 2.0];
    let curve_f64 = PiecewiseCubicHermite::monotone(&knots_f64, &values_f64).unwrap();
    for interval in 0..knots_f32.len() - 1 {
        for step in 0..=32 {
            let ratio = step as f32 / 32.0;
            let point =
                knots_f32[interval] + ratio * (knots_f32[interval + 1] - knots_f32[interval]);
            let value = curve_f32.evaluate(point).unwrap();
            assert!(
                value >= values_f32[interval] - 2.0e-5
                    && value <= values_f32[interval + 1] + 2.0e-5
            );
            let ratio64 = step as f64 / 32.0;
            let point64 =
                knots_f64[interval] + ratio64 * (knots_f64[interval + 1] - knots_f64[interval]);
            let value64 = curve_f64.evaluate(point64).unwrap();
            assert!(
                value64 >= values_f64[interval] - 2.0e-12
                    && value64 <= values_f64[interval + 1] + 2.0e-12
            );
        }
    }
}

#[test]
fn controlled_and_spline_constructors_use_typed_endpoint_conditions() {
    let knots = [0.0_f64, 1.0, 2.0, 3.0, 4.0];
    let values = [0.0_f64, 1.0, 0.5, 1.5, 1.0];
    let controlled = PiecewiseCubicHermite::monotone_with_conditions(
        &knots,
        &values,
        MonotoneEndpointCondition::FirstDerivative {
            value: 1.0,
            adjust_for_monotonicity: true,
        },
        MonotoneEndpointCondition::SecondDerivative {
            value: 0.0,
            adjust_for_monotonicity: true,
        },
        SwitchPointPolicy::ForceExtrema,
    )
    .unwrap();
    for index in 0..knots.len() {
        assert_close_f64(
            controlled.evaluate(knots[index]).unwrap(),
            values[index],
            1.0e-13,
        );
    }
    let spline = PiecewiseCubicHermite::spline(
        &knots,
        &values,
        EndpointCondition::FirstDerivative(1.0),
        EndpointCondition::SecondDerivative(0.0),
    )
    .unwrap();
    assert_close_f64(spline.derivatives()[0], 1.0, 2.0e-12);
    for index in 0..knots.len() {
        assert_close_f64(
            spline.evaluate(knots[index]).unwrap(),
            values[index],
            2.0e-12,
        );
    }
}

#[test]
fn extrapolation_is_explicit_and_batch_order_is_preserved() {
    let curve = PiecewiseCubicHermite::<f32>::from_derivatives(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
    )
    .unwrap();
    assert_eq!(curve.evaluate(-0.1), Err(PchipError::OutOfDomain));
    let points = [1.25_f32, 0.25, -0.5];
    let mut values = [0.0; 3];
    let report = curve
        .evaluate_with_policy_into(&points, &mut values, Extrapolation::Allow)
        .unwrap();
    assert_eq!(report.extrapolated_points, 2);
    assert_close_f32(values[0], 1.25, 2.0e-6);
    assert_close_f32(values[1], 0.25, 2.0e-6);
    assert_close_f32(values[2], -0.5, 2.0e-6);
    let integral = curve
        .integrate_with_policy(-1.0, 2.0, Extrapolation::Allow)
        .unwrap();
    assert!(integral.lower_extrapolated && integral.upper_extrapolated);
    assert_close_f32(integral.value, 1.5, 2.0e-6);
}

#[test]
fn invalid_input_is_rejected_before_native_entry() {
    assert_eq!(
        PiecewiseCubicHermite::<f64>::monotone(&[0.0], &[1.0]),
        Err(PchipError::TooFewPoints)
    );
    assert_eq!(
        PiecewiseCubicHermite::<f64>::monotone(&[0.0, 0.0], &[1.0, 2.0]),
        Err(PchipError::KnotsNotStrictlyIncreasing)
    );
    assert_eq!(
        PiecewiseCubicHermite::<f64>::monotone(&[0.0, 1.0], &[1.0, f64::NAN]),
        Err(PchipError::NonFiniteInput)
    );
    let curve = PiecewiseCubicHermite::<f64>::from_derivatives(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![1.0, 1.0],
    )
    .unwrap();
    assert_eq!(
        curve.evaluate_into(&[0.5], &mut []),
        Err(PchipError::LengthMismatch)
    );
    assert_eq!(
        curve.integrate(f64::INFINITY, 1.0),
        Err(PchipError::NonFiniteInput)
    );
}
