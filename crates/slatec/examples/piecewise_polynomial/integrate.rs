//! Integrate a real SLATEC PP-form polynomial exactly over its domain.
//!
//! <!-- slatec-safe-example -->
//!
//! Requires `std`, a native backend, and `piecewise-polynomial`. This calls
//! `PPQAD`/`DPPQAD`, not a numerical quadrature routine.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;

fn main() {
    // f(x) = x on [0, 1].
    let curve = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![0.0, 1.0], 2)
        .expect("valid PP representation");
    assert!((curve.integrate(0.0, 1.0).expect("in-domain bounds") - 0.5).abs() < 1.0e-12);
    assert!((curve.integrate(1.0, 0.0).expect("reversed bounds") + 0.5).abs() < 1.0e-12);
}
