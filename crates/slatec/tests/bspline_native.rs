#![cfg(feature = "bspline-native-tests")]

//! Native regression coverage for the restricted B-spline representation API.

use slatec::interpolation::bspline::{BSpline, BSplineError, Extrapolation};

fn linear_f32() -> BSpline<f32> {
    BSpline::<f32>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap()
}

fn linear_f64() -> BSpline<f64> {
    BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap()
}

fn quadratic_f32() -> BSpline<f32> {
    // The order-three Bernstein B-spline basis on [0, 1] is
    // ((1-x)^2, 2*x*(1-x), x^2), so these coefficients give x^2 exactly.
    BSpline::<f32>::from_parts(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], vec![0.0, 0.0, 1.0], 3).unwrap()
}

fn quadratic_f64() -> BSpline<f64> {
    BSpline::<f64>::from_parts(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], vec![0.0, 0.0, 1.0], 3).unwrap()
}

#[test]
fn f32_value_derivative_batch_and_integral_match_linear_reference() {
    let spline = linear_f32();
    assert!((spline.evaluate(0.25).unwrap() - 0.25).abs() < 2.0e-5);
    assert!((spline.derivative(0.5, 1).unwrap() - 1.0).abs() < 2.0e-5);
    let mut values = [0.0; 3];
    spline.evaluate_into(&[0.0, 0.5, 1.0], &mut values).unwrap();
    assert_eq!(values[0], 0.0);
    assert!((values[1] - 0.5).abs() < 2.0e-5);
    assert!((values[2] - 1.0).abs() < 2.0e-5);
    assert!((spline.integrate(0.0, 1.0).unwrap() - 0.5).abs() < 3.0e-5);
    assert!((spline.integrate(1.0, 0.0).unwrap() + 0.5).abs() < 3.0e-5);
}

#[test]
fn f64_value_derivative_batch_and_integral_match_linear_reference() {
    let spline = linear_f64();
    assert!((spline.evaluate(0.25).unwrap() - 0.25).abs() < 1.0e-12);
    assert!((spline.derivative(0.5, 1).unwrap() - 1.0).abs() < 1.0e-12);
    let mut values = [0.0; 3];
    spline.evaluate_into(&[0.0, 0.5, 1.0], &mut values).unwrap();
    assert_eq!(values[0], 0.0);
    assert!((values[1] - 0.5).abs() < 1.0e-12);
    assert!((values[2] - 1.0).abs() < 1.0e-12);
    assert!((spline.integrate(0.0, 1.0).unwrap() - 0.5).abs() < 1.0e-12);
    assert!((spline.integrate(1.0, 0.0).unwrap() + 0.5).abs() < 1.0e-12);
}

#[test]
fn endpoint_clamping_and_contract_rejection_are_explicit() {
    let spline = linear_f64();
    assert_eq!(spline.evaluate(-0.5), Err(BSplineError::OutOfDomain));
    assert_eq!(
        spline.evaluate_with_extrapolation(-0.5, Extrapolation::Clamp),
        Ok(0.0)
    );
    assert_eq!(
        spline.evaluate_with_extrapolation(1.5, Extrapolation::Clamp),
        Ok(1.0)
    );
    assert_eq!(
        spline.derivative(0.5, 2),
        Err(BSplineError::DerivativeOrderTooHigh {
            requested: 2,
            maximum: 1
        })
    );
}

#[test]
fn f32_constant_and_quadratic_representations_match_independent_polynomials() {
    let constant = BSpline::<f32>::from_parts(vec![0.0, 1.0], vec![3.5], 1).unwrap();
    assert!((constant.evaluate(0.25).unwrap() - 3.5).abs() < 2.0e-5);
    assert!((constant.integrate(0.0, 1.0).unwrap() - 3.5).abs() < 3.0e-5);

    let quadratic = quadratic_f32();
    for point in [0.0_f32, 0.125, 0.5, 0.875, 1.0] {
        assert!((quadratic.evaluate(point).unwrap() - point * point).abs() < 4.0e-5);
        assert!((quadratic.derivative(point, 1).unwrap() - 2.0 * point).abs() < 6.0e-5);
    }
    assert!((quadratic.integrate(0.0, 1.0).unwrap() - 1.0 / 3.0).abs() < 5.0e-5);
}

#[test]
fn f64_constant_and_quadratic_representations_match_independent_polynomials() {
    let constant = BSpline::<f64>::from_parts(vec![0.0, 1.0], vec![3.5], 1).unwrap();
    assert!((constant.evaluate(0.25).unwrap() - 3.5).abs() < 1.0e-12);
    assert!((constant.integrate(0.0, 1.0).unwrap() - 3.5).abs() < 1.0e-12);

    let quadratic = quadratic_f64();
    for point in [0.0_f64, 0.125, 0.5, 0.875, 1.0] {
        assert!((quadratic.evaluate(point).unwrap() - point * point).abs() < 2.0e-12);
        assert!((quadratic.derivative(point, 1).unwrap() - 2.0 * point).abs() < 3.0e-12);
    }
    assert!((quadratic.integrate(0.0, 1.0).unwrap() - 1.0 / 3.0).abs() < 3.0e-12);
}

#[test]
fn repeated_interior_knots_preserve_the_native_right_limit_convention() {
    // Multiplicity equal to the order permits a discontinuity. The left
    // interval is zero and the right interval is one.
    let spline = BSpline::<f64>::from_parts(
        vec![0.0, 0.0, 0.5, 0.5, 1.0, 1.0],
        vec![0.0, 0.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    assert!(spline.evaluate(0.49).unwrap().abs() < 1.0e-12);
    assert!((spline.evaluate(0.5).unwrap() - 1.0).abs() < 1.0e-12);
    assert!((spline.evaluate(0.51).unwrap() - 1.0).abs() < 1.0e-12);
}

#[test]
fn coefficient_linearity_and_integral_additivity_hold() {
    let linear = linear_f64();
    let constant = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![1.0, 1.0], 2).unwrap();
    let sum = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![1.0, 2.0], 2).unwrap();
    for point in [0.1, 0.4, 0.9] {
        assert!(
            (sum.evaluate(point).unwrap()
                - linear.evaluate(point).unwrap()
                - constant.evaluate(point).unwrap())
            .abs()
                < 2.0e-12
        );
    }
    assert!(
        (sum.integrate(0.0, 1.0).unwrap()
            - linear.integrate(0.0, 1.0).unwrap()
            - constant.integrate(0.0, 1.0).unwrap())
        .abs()
            < 2.0e-12
    );
}
