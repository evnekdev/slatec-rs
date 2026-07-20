// slatec-safe-example
//! Integrate a B-spline with SLATEC `BSQAD`/`DBSQAD`.
//! Requires `std,external-backend,bspline`. Integration is native and accepts
//! only in-domain endpoints; it does not extrapolate.

use slatec::interpolation::bspline::BSpline;

fn main() {
    let spline = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid B-spline parts");
    let area = spline.integrate(0.0, 1.0).expect("in-domain integral");
    assert!((area - 0.5).abs() < 1.0e-12);
}
