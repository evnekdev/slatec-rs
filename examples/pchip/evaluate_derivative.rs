// slatec-safe-example
//! Value and derivative evaluation through SLATEC `PCHFD`/`DPCHFD`.
//! Requires `std,external-backend,pchip`.

use slatec::pchip::PiecewiseCubicHermite;

fn main() {
    let curve =
        PiecewiseCubicHermite::from_derivatives(vec![0.0_f32, 1.0], vec![1.0, 2.0], vec![1.0, 1.0])
            .expect("valid Hermite data");
    let sample = curve.evaluate_with_derivative(0.5).expect("PCHFD");
    assert!((sample.value - 1.5).abs() < 1.0e-5);
    assert!((sample.first_derivative - 1.0).abs() < 1.0e-5);
}
