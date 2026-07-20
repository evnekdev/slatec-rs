// slatec-safe-example
//! Evaluate a B-spline derivative with SLATEC `BVALU`/`DBVALU`.
//! Requires `std,external-backend,bspline`; no coefficient or knot conversion occurs.

use slatec::interpolation::bspline::BSpline;

fn main() {
    let spline = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid B-spline parts");
    let slope = spline.derivative(0.5, 1).expect("first derivative");
    assert!((slope - 1.0).abs() < 1.0e-12);
}
