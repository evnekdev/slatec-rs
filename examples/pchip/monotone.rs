// slatec-safe-example
//! Monotone PCHIP construction with SLATEC `PCHIM`/`DPCHIM`.
//! Requires `std,external-backend,pchip`; PCHIP is linear interpolation, not a B-spline API.

use slatec::pchip::PiecewiseCubicHermite;

fn main() {
    let curve =
        PiecewiseCubicHermite::<f64>::monotone(&[0.0, 0.25, 1.0, 2.0], &[0.0, 0.1, 0.8, 1.0])
            .expect("valid monotone data");
    for step in 0..=32 {
        let value = curve.evaluate(step as f64 / 16.0).expect("in-domain point");
        assert!((0.0..=1.0).contains(&value));
    }
}
