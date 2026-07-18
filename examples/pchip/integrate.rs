// slatec-safe-example
//! Definite integration through SLATEC `PCHIA`/`DPCHIA`.
//! Requires `std,external-backend,pchip`.

use slatec::pchip::PiecewiseCubicHermite;

fn main() {
    let curve =
        PiecewiseCubicHermite::from_derivatives(vec![0.0_f64, 1.0], vec![0.0, 1.0], vec![1.0, 1.0])
            .expect("linear Hermite curve");
    assert!((curve.integrate(0.0, 1.0).expect("PCHIA") - 0.5).abs() < 1.0e-12);
    assert!((curve.integrate(1.0, 0.0).expect("reversed PCHIA") + 0.5).abs() < 1.0e-12);
}
