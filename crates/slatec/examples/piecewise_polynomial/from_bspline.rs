//! Convert an owned SLATEC B-spline exactly to PP form.
//!
//! <!-- slatec-safe-example -->
//!
//! Requires `std`, a native backend, `bspline`, and `piecewise-polynomial`.
//! The conversion calls `BSPPP`/`DBSPPP`; it does not reconstruct coefficients
//! in Rust.

use slatec::interpolation::bspline::BSpline;

fn main() {
    let spline = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid B-spline");
    let curve = spline
        .to_piecewise_polynomial()
        .expect("reviewed BSPPP conversion");
    assert!((curve.evaluate(0.25).expect("in-domain point") - 0.25).abs() < 1.0e-12);
}
