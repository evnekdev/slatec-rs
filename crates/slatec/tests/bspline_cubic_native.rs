#![cfg(all(
    feature = "bspline-cubic-interpolation",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native checks for the source-accurate `BINT4`/`DBINT4` safe constructors.

use slatec::interpolation::bspline::{
    BSpline, BSplineError, CubicBoundaryCondition, CubicKnotPlacement,
};
use slatec::native_lock_test_support::with_native_lock;

fn cubic(x: f64) -> f64 {
    x * x * x - 2.0 * x + 1.0
}

fn cubic_derivative(x: f64) -> f64 {
    3.0 * x * x - 2.0
}

#[test]
fn dbint4_reproduces_data_and_prescribed_first_derivatives() {
    let nodes = [0.0_f64, 0.5, 1.5, 2.0];
    let values = nodes.map(cubic);
    let spline = BSpline::<f64>::interpolate_cubic(
        &nodes,
        &values,
        CubicBoundaryCondition::FirstDerivative(cubic_derivative(nodes[0])),
        CubicBoundaryCondition::FirstDerivative(cubic_derivative(nodes[nodes.len() - 1])),
        CubicKnotPlacement::EndpointMultiplicity,
    )
    .unwrap();

    assert_eq!(spline.order(), 4);
    assert_eq!(spline.coefficient_count(), nodes.len() + 2);
    assert_eq!(spline.knots().len(), nodes.len() + 6);
    for (&node, &value) in nodes.iter().zip(&values) {
        assert!((spline.evaluate(node).unwrap() - value).abs() < 2.0e-11);
    }
    assert!((spline.derivative(nodes[0], 1).unwrap() - cubic_derivative(nodes[0])).abs() < 3.0e-10);
    assert!(
        (spline.derivative(nodes[nodes.len() - 1], 1).unwrap()
            - cubic_derivative(nodes[nodes.len() - 1]))
        .abs()
            < 3.0e-10
    );
}

#[test]
fn bint4_accepts_explicit_exterior_knots_and_reproduces_f32_nodes() {
    let nodes = [0.0_f32, 0.75, 1.5, 2.25, 3.0];
    let values = nodes.map(|x| x * x * x - 2.0 * x + 1.0);
    let spline = BSpline::<f32>::interpolate_cubic(
        &nodes,
        &values,
        CubicBoundaryCondition::FirstDerivative(-2.0),
        CubicBoundaryCondition::FirstDerivative(25.0),
        CubicKnotPlacement::Explicit {
            left: [-3.0, -2.0, -1.0],
            right: [4.0, 5.0, 6.0],
        },
    )
    .unwrap();

    for (&node, &value) in nodes.iter().zip(&values) {
        assert!((spline.evaluate(node).unwrap() - value).abs() < 5.0e-4);
    }
}

#[test]
fn dbint4_owned_result_matches_the_reviewed_raw_constructor() {
    let nodes = [0.0_f64, 0.5, 1.5, 2.0];
    let values = nodes.map(cubic);
    let safe = BSpline::<f64>::interpolate_cubic(
        &nodes,
        &values,
        CubicBoundaryCondition::FirstDerivative(cubic_derivative(nodes[0])),
        CubicBoundaryCondition::FirstDerivative(cubic_derivative(nodes[nodes.len() - 1])),
        CubicKnotPlacement::EndpointMultiplicity,
    )
    .unwrap();

    let mut raw_nodes = nodes;
    let mut raw_values = values;
    let mut point_count = 4;
    let mut left_kind = 1;
    let mut right_kind = 1;
    let mut left_value = cubic_derivative(nodes[0]);
    let mut right_value = cubic_derivative(nodes[nodes.len() - 1]);
    let mut knot_option = 1;
    let mut knots = [0.0_f64; 10];
    let mut coefficients = [0.0_f64; 6];
    let mut coefficient_count = 0;
    let mut order = 0;
    let mut workspace = [0.0_f64; 30];
    with_native_lock(|| {
        // SAFETY: this probe holds the production process-wide native lock
        // and supplies DBINT4's reviewed dimensions and private buffers.
        unsafe {
            slatec_sys::interpolation::dbint4(
                raw_nodes.as_mut_ptr(),
                raw_values.as_mut_ptr(),
                &mut point_count,
                &mut left_kind,
                &mut right_kind,
                &mut left_value,
                &mut right_value,
                &mut knot_option,
                knots.as_mut_ptr(),
                coefficients.as_mut_ptr(),
                &mut coefficient_count,
                &mut order,
                workspace.as_mut_ptr(),
            );
        }
    });
    assert_eq!((coefficient_count, order), (6, 4));
    for (actual, expected) in safe.knots().iter().zip(knots) {
        assert!((actual - expected).abs() < 1.0e-13);
    }
    for (actual, expected) in safe.coefficients().iter().zip(coefficients) {
        assert!((actual - expected).abs() < 1.0e-13);
    }
}

#[test]
fn cubic_constructor_preflights_boundary_and_exterior_knot_contracts() {
    assert_eq!(
        BSpline::<f64>::interpolate_cubic(
            &[0.0, 1.0],
            &[0.0, 1.0],
            CubicBoundaryCondition::FirstDerivative(f64::NAN),
            CubicBoundaryCondition::SecondDerivative(0.0),
            CubicKnotPlacement::EndpointMultiplicity,
        ),
        Err(BSplineError::NonFiniteCubicBoundaryCondition)
    );
    assert_eq!(
        BSpline::<f64>::interpolate_cubic(
            &[0.0, 1.0],
            &[0.0, 1.0],
            CubicBoundaryCondition::SecondDerivative(0.0),
            CubicBoundaryCondition::SecondDerivative(0.0),
            CubicKnotPlacement::Explicit {
                left: [-2.0, -1.0, 0.0],
                right: [2.0, 3.0, 4.0],
            },
        ),
        Err(BSplineError::InvalidCubicExteriorKnots)
    );
}
