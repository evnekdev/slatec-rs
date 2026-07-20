// slatec-safe-example
//! Construct a `BINTK`/`DBINTK` interpolant and convert it exactly to PP form.
//!
//! Requires `std,external-backend,bspline,piecewise-polynomial`. Construction
//! uses a complete explicit knot sequence and does not create a second spline
//! representation or translate a spline algorithm.

use slatec::interpolation::bspline::BSpline;

fn main() {
    let nodes = [0.0, 0.75, 1.5, 2.25, 3.0];
    let values = nodes.map(|x| x * x * x - 2.0 * x + 1.0);
    let knots = [-3.0, -2.0, -1.0, 0.0, 1.5, 3.0, 4.0, 5.0, 6.0];
    let spline = BSpline::<f64>::interpolate_with_knots(&nodes, &values, &knots, 4)
        .expect("valid BINTK interpolation data");
    let piecewise = spline
        .to_piecewise_polynomial()
        .expect("reviewed BSPPP conversion");

    for point in [0.0, 0.5, 1.5, 2.5, 3.0] {
        let native = spline.evaluate(point).expect("B-spline domain");
        let converted = piecewise.evaluate(point).expect("PP domain");
        assert!((native - converted).abs() < 2.0e-11);
    }
}
