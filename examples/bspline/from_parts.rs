// slatec-safe-example
//! Construct and evaluate a SLATEC `BVALU`/`DBVALU` B-spline.
//! Requires `std,external-backend,bspline`. This owns an existing B-spline
//! representation; it does not fit interpolation data or call PCHIP.

use slatec::interpolation::bspline::BSpline;

fn main() {
    // Order two with these clamped knots represents f(x) = x on [0, 1].
    let spline = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid B-spline parts");
    let value = spline.evaluate(0.25).expect("in-domain point");
    assert!((value - 0.25).abs() < 1.0e-12);
    let mut values = [0.0; 2];
    spline
        .evaluate_into(&[0.25, 0.75], &mut values)
        .expect("matching in-domain output");
    assert!((values[1] - 0.75).abs() < 1.0e-12);
}
