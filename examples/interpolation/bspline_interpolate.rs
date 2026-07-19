// slatec-safe-example
//! Construct an exact cubic B-spline interpolant with `BINTK`/`DBINTK`.
//!
//! Requires `std,external-backend,bspline`. The supplied values are
//! interpolated exactly in native precision; this is not a least-squares fit
//! or a PCHIP curve. `knots` is the complete B-spline knot sequence.

use slatec::interpolation::bspline::BSpline;

fn cubic(x: f64) -> f64 {
    x * x * x - 2.0 * x + 1.0
}

fn main() {
    let nodes = [0.0, 0.75, 1.5, 2.25, 3.0];
    let values = nodes.map(cubic);
    // Four knots lie at or below the first node and four at or above the last
    // node. Interior supports meet the Schoenberg--Whitney condition.
    let knots = [-3.0, -2.0, -1.0, 0.0, 1.5, 3.0, 4.0, 5.0, 6.0];
    let spline = BSpline::<f64>::interpolate_with_knots(&nodes, &values, &knots, 4)
        .expect("valid BINTK interpolation data");

    assert!((spline.evaluate(1.25).expect("in domain") - cubic(1.25)).abs() < 2.0e-11);
    assert!((spline.derivative(1.25, 1).expect("native derivative") - 2.6875).abs() < 5.0e-11);
    assert!((spline.integrate(0.0, 3.0).expect("native integral") - 14.25).abs() < 1.0e-10);
}
