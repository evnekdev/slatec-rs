#![cfg(feature = "piecewise-polynomial-native-tests")]

//! Native and independent-oracle coverage for PP-form interpolation.

use slatec::interpolation::bspline::BSpline;
use slatec::interpolation::piecewise_polynomial::{
    Extrapolation, PiecewisePolynomial, PiecewisePolynomialError,
};

fn linear_f64() -> PiecewisePolynomial<f64> {
    // Each coefficient slice contains right Taylor derivatives. Both pieces
    // therefore represent f(x) = 1 + 2*x in their local coordinates.
    PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 3.0, 2.0], 2)
        .unwrap()
}

fn linear_f32() -> PiecewisePolynomial<f32> {
    PiecewisePolynomial::<f32>::from_parts(vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 3.0, 2.0], 2)
        .unwrap()
}

#[test]
fn f64_linear_value_derivative_batch_and_integral_match_oracle() {
    let curve = linear_f64();
    assert_eq!(curve.order(), 2);
    assert_eq!(curve.degree(), 1);
    assert_eq!(curve.piece_count(), 2);
    assert_eq!(curve.coefficients_for_piece(1), Some(&[3.0, 2.0][..]));
    for point in [0.0, 0.25, 0.75, 1.0, 1.5, 2.0] {
        assert!((curve.evaluate(point).unwrap() - (1.0 + 2.0 * point)).abs() < 1.0e-12);
        assert!((curve.derivative(point, 1).unwrap() - 2.0).abs() < 1.0e-12);
    }
    let mut output = [0.0; 4];
    curve
        .evaluate_into(&[1.75, 0.5, 1.0, 0.0], &mut output)
        .unwrap();
    assert_eq!(output, [4.5, 2.0, 3.0, 1.0]);
    assert!((curve.integrate(0.0, 2.0).unwrap() - 6.0).abs() < 1.0e-12);
    assert!((curve.integrate(2.0, 0.0).unwrap() + 6.0).abs() < 1.0e-12);
    assert_eq!(curve.integrate(1.0, 1.0).unwrap(), 0.0);
}

#[test]
fn f32_linear_value_derivative_batch_and_integral_match_oracle() {
    let curve = linear_f32();
    for point in [0.0_f32, 0.25, 0.75, 1.0, 1.5, 2.0] {
        assert!((curve.evaluate(point).unwrap() - (1.0 + 2.0 * point)).abs() < 3.0e-5);
        assert!((curve.derivative(point, 1).unwrap() - 2.0).abs() < 3.0e-5);
    }
    let mut output = [0.0; 3];
    curve.evaluate_into(&[1.5, 0.5, 1.0], &mut output).unwrap();
    assert!((output[0] - 4.0).abs() < 3.0e-5);
    assert!((output[1] - 2.0).abs() < 3.0e-5);
    assert!((output[2] - 3.0).abs() < 3.0e-5);
    assert!((curve.integrate(0.0, 2.0).unwrap() - 6.0).abs() < 5.0e-5);
}

#[test]
fn taylor_coefficients_and_breakpoint_side_are_exact() {
    // 1 + 2*x + 3*x^2 has [f(0), f'(0), f''(0)] = [1, 2, 6].
    let quadratic =
        PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0, 2.0, 6.0], 3).unwrap();
    assert!((quadratic.evaluate(0.5).unwrap() - 2.75).abs() < 1.0e-12);
    assert!((quadratic.derivative(0.5, 1).unwrap() - 5.0).abs() < 1.0e-12);
    assert!((quadratic.derivative(0.5, 2).unwrap() - 6.0).abs() < 1.0e-12);
    assert!((quadratic.integrate(0.0, 1.0).unwrap() - 3.0).abs() < 1.0e-12);

    let discontinuous =
        PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0, 2.0], vec![2.0, 3.0], 1).unwrap();
    assert_eq!(discontinuous.evaluate(0.999).unwrap(), 2.0);
    assert_eq!(discontinuous.evaluate(1.0).unwrap(), 3.0);
    assert_eq!(discontinuous.evaluate(2.0).unwrap(), 3.0);
    assert_eq!(discontinuous.integrate(0.0, 2.0).unwrap(), 5.0);
}

#[test]
fn validation_and_explicit_clamping_prevent_native_contract_errors() {
    let curve = linear_f64();
    assert_eq!(
        curve.evaluate(-0.1),
        Err(PiecewisePolynomialError::OutOfDomain)
    );
    assert_eq!(
        curve.evaluate_with_extrapolation(-0.1, Extrapolation::Clamp),
        Ok(1.0)
    );
    assert_eq!(
        curve.derivative(0.5, 2),
        Err(PiecewisePolynomialError::DerivativeOrderTooHigh {
            requested: 2,
            maximum: 1,
        })
    );
    let mut wrong = [0.0; 1];
    assert_eq!(
        curve.evaluate_into(&[0.0, 1.0], &mut wrong),
        Err(PiecewisePolynomialError::OutputLengthMismatch {
            expected: 2,
            actual: 1,
        })
    );
    assert_eq!(
        PiecewisePolynomial::<f64>::from_parts(vec![0.0, 0.0], vec![1.0], 1),
        Err(PiecewisePolynomialError::BreakpointsNotStrictlyIncreasing { index: 1 })
    );
}

#[test]
fn bspline_conversion_preserves_values_derivatives_integrals_and_domain() {
    let spline =
        BSpline::<f64>::from_parts(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], vec![0.0, 0.0, 1.0], 3)
            .unwrap();
    let pp = spline.to_piecewise_polynomial().unwrap();
    assert_eq!(pp.domain(), spline.domain());
    for point in [0.0, 0.125, 0.5, 0.875, 1.0] {
        assert!((pp.evaluate(point).unwrap() - spline.evaluate(point).unwrap()).abs() < 3.0e-12);
        assert!(
            (pp.derivative(point, 1).unwrap() - spline.derivative(point, 1).unwrap()).abs()
                < 5.0e-12
        );
    }
    assert!(
        (pp.integrate(0.0, 1.0).unwrap() - spline.integrate(0.0, 1.0).unwrap()).abs() < 3.0e-12
    );

    // Repeated interior knots preserve a right-side discontinuity while the
    // converter removes duplicate PP breakpoints.
    let discontinuous = BSpline::<f64>::from_parts(
        vec![0.0, 0.0, 0.5, 0.5, 1.0, 1.0],
        vec![0.0, 0.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    let converted = discontinuous.to_piecewise_polynomial().unwrap();
    assert_eq!(converted.breakpoints(), &[0.0, 0.5, 1.0]);
    assert!(converted.evaluate(0.49).unwrap().abs() < 1.0e-12);
    assert!((converted.evaluate(0.5).unwrap() - 1.0).abs() < 1.0e-12);
}

#[test]
fn bspline_conversion_is_available_in_both_precisions() {
    let spline = BSpline::<f32>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap();
    let pp = spline.to_piecewise_polynomial().unwrap();
    assert!((pp.evaluate(0.25).unwrap() - 0.25).abs() < 3.0e-5);
    assert!((pp.integrate(0.0, 1.0).unwrap() - 0.5).abs() < 4.0e-5);
}
