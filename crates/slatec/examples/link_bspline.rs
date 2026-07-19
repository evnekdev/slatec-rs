//! Narrow link probe for the reviewed B-spline source closure.

use slatec::interpolation::bspline::BSpline;

fn main() {
    let spline = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid native B-spline storage");
    let _ = spline.evaluate(0.5).expect("DBVALU");
}
