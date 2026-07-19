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

fn cubic_nodes_f64() -> [f64; 5] {
    [0.0, 0.75, 1.5, 2.25, 3.0]
}

fn cubic_knots_f64() -> [f64; 9] {
    // The complete knot vector is supplied directly to DBINTK. The first
    // four knots lie at or to the left of x[0], and the final four lie at or
    // to the right of x[N-1]; every data point meets the strict
    // Schoenberg--Whitney support condition.
    [-3.0, -2.0, -1.0, 0.0, 1.5, 3.0, 4.0, 5.0, 6.0]
}

fn cubic_value_f64(x: f64) -> f64 {
    x * x * x - 2.0 * x + 1.0
}

fn cubic_derivative_f64(x: f64) -> f64 {
    3.0 * x * x - 2.0
}

#[test]
fn dbintk_construction_reproduces_a_cubic_and_integrates_it() {
    let nodes = cubic_nodes_f64();
    let values = nodes.map(cubic_value_f64);
    let spline =
        BSpline::<f64>::interpolate_with_knots(&nodes, &values, &cubic_knots_f64(), 4).unwrap();
    assert_eq!(spline.order(), 4);
    assert_eq!(spline.knots(), &cubic_knots_f64());
    for (&node, value) in nodes.iter().zip(values) {
        assert!((spline.evaluate(node).unwrap() - value).abs() < 5.0e-12);
    }
    for point in [0.125, 0.5, 1.25, 2.5] {
        assert!((spline.evaluate(point).unwrap() - cubic_value_f64(point)).abs() < 2.0e-11);
        assert!(
            (spline.derivative(point, 1).unwrap() - cubic_derivative_f64(point)).abs() < 5.0e-11
        );
    }
    assert!((spline.integrate(0.0, 3.0).unwrap() - 14.25).abs() < 1.0e-10);
}

#[test]
fn bintk_construction_reproduces_nonuniform_cubic_data() {
    let nodes = [0.0_f32, 0.75, 1.5, 2.25, 3.0];
    let values = nodes.map(|x| x * x * x - 2.0 * x + 1.0);
    let knots = [-3.0_f32, -2.0, -1.0, 0.0, 1.5, 3.0, 4.0, 5.0, 6.0];
    let spline = BSpline::<f32>::interpolate_with_knots(&nodes, &values, &knots, 4).unwrap();
    for (&node, value) in nodes.iter().zip(values) {
        assert!((spline.evaluate(node).unwrap() - value).abs() < 3.0e-4);
    }
    for point in [0.125_f32, 0.5, 1.25, 2.5] {
        let expected = point * point * point - 2.0 * point + 1.0;
        let derivative = 3.0 * point * point - 2.0;
        assert!((spline.evaluate(point).unwrap() - expected).abs() < 5.0e-4);
        assert!((spline.derivative(point, 1).unwrap() - derivative).abs() < 1.0e-3);
    }
    assert!((spline.integrate(0.0, 3.0).unwrap() - 14.25).abs() < 1.0e-3);
}

#[test]
fn bintk_accepts_order_one_and_clamped_order_two_endpoint_conditions() {
    let constant = BSpline::<f64>::interpolate_with_knots(&[0.5], &[3.25], &[0.0, 1.0], 1).unwrap();
    assert_eq!(constant.order(), 1);
    assert!((constant.evaluate(0.0).unwrap() - 3.25).abs() < 1.0e-12);
    assert!((constant.evaluate(1.0).unwrap() - 3.25).abs() < 1.0e-12);

    let nodes = [0.0_f64, 1.0, 2.0, 3.0];
    let values = nodes.map(|x| 2.0 * x - 1.0);
    // Endpoint equality is valid only because both endpoint knots repeat K=2
    // times. The two interior supports remain strict.
    let spline =
        BSpline::<f64>::interpolate_with_knots(&nodes, &values, &[0.0, 0.0, 1.5, 2.5, 3.0, 3.0], 2)
            .unwrap();
    for (&node, &value) in nodes.iter().zip(values.iter()) {
        assert!((spline.evaluate(node).unwrap() - value).abs() < 2.0e-12);
    }
    assert!((spline.evaluate(1.25).unwrap() - 1.5).abs() < 2.0e-12);
}

#[test]
fn interpolation_is_affine_in_the_ordinates() {
    let nodes = cubic_nodes_f64();
    let knots = cubic_knots_f64();
    let values = nodes.map(cubic_value_f64);
    let shifted = values.map(|value| value + 4.5);
    let scaled = values.map(|value| -2.0 * value);
    let base = BSpline::<f64>::interpolate_with_knots(&nodes, &values, &knots, 4).unwrap();
    let shifted = BSpline::<f64>::interpolate_with_knots(&nodes, &shifted, &knots, 4).unwrap();
    let scaled = BSpline::<f64>::interpolate_with_knots(&nodes, &scaled, &knots, 4).unwrap();
    for point in [0.125, 0.5, 1.25, 2.5] {
        let value = base.evaluate(point).unwrap();
        assert!((shifted.evaluate(point).unwrap() - (value + 4.5)).abs() < 3.0e-11);
        assert!((scaled.evaluate(point).unwrap() + 2.0 * value).abs() < 5.0e-11);
    }
}

#[test]
fn interpolation_input_and_schoenberg_whitney_preflight_are_typed() {
    let nodes = [0.0_f64, 1.0, 2.0];
    let knots = [-1.0, 0.0, 1.0, 2.0, 3.0];
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&nodes, &[0.0, 1.0], &knots, 2),
        Err(BSplineError::InterpolationLengthMismatch {
            nodes: 3,
            values: 2,
        })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&[], &[], &[], 1),
        Err(BSplineError::TooFewInterpolationPoints {
            points: 0,
            order: 1,
        })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&nodes, &[0.0, 1.0, 2.0], &knots, 0),
        Err(BSplineError::InvalidOrder)
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&nodes, &[0.0, 1.0, 2.0], &knots, 4),
        Err(BSplineError::TooFewInterpolationPoints {
            points: 3,
            order: 4,
        })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&nodes, &[0.0, 1.0, 2.0], &knots[..4], 2),
        Err(BSplineError::KnotCountMismatch {
            expected: 5,
            actual: 4,
        })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(
            &nodes,
            &[0.0, 1.0, 2.0],
            &[-1.0, 0.0, f64::NAN, 2.0, 3.0],
            2,
        ),
        Err(BSplineError::NonFiniteInput)
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(
            &nodes,
            &[0.0, 1.0, 2.0],
            &[-1.0, 1.5, 1.0, 2.0, 3.0],
            2,
        ),
        Err(BSplineError::KnotsNotNondecreasing { index: 2 })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&[0.0, 0.0, 2.0], &[0.0, 1.0, 2.0], &knots, 2),
        Err(BSplineError::InterpolationNodesNotStrictlyIncreasing { index: 1 })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&[0.0, f64::NAN, 2.0], &[0.0, 1.0, 2.0], &knots, 2),
        Err(BSplineError::NonFiniteInterpolationNode { index: 1 })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(&nodes, &[0.0, f64::INFINITY, 2.0], &knots, 2),
        Err(BSplineError::NonFiniteInterpolationValue { index: 1 })
    );
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(
            &nodes,
            &[0.0, 1.0, 2.0],
            &[-1.0, 0.0, 0.5, 0.75, 3.0],
            2,
        ),
        Err(BSplineError::SchoenbergWhitneyViolation { index: 2 })
    );
}

#[test]
fn construction_scopes_xerror_and_invalid_preflight_does_not_touch_it() {
    let mut before = 0;
    // SAFETY: XGETF has the reviewed scalar Fortran ABI. The safe constructor
    // itself holds the process-wide runtime lock for its complete XERROR scope.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };

    let nodes = cubic_nodes_f64();
    let values = nodes.map(cubic_value_f64);
    BSpline::<f64>::interpolate_with_knots(&nodes, &values, &cubic_knots_f64(), 4).unwrap();
    assert_eq!(
        BSpline::<f64>::interpolate_with_knots(
            &nodes,
            &values,
            &[-3.0, -2.0, -1.0, 0.5, 1.5, 3.0, 4.0, 5.0, 6.0],
            4,
        ),
        Err(BSplineError::SchoenbergWhitneyViolation { index: 0 })
    );

    let mut after = 0;
    // SAFETY: XGETF has the reviewed scalar Fortran ABI.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(after, before);
}
