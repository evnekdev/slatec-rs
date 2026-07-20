//! Construct and evaluate a real SLATEC PP-form polynomial.
//!
//! <!-- slatec-safe-example -->
//!
//! Requires `std`, a native backend, and `piecewise-polynomial`. This uses
//! `PPVAL`/`DPPVAL`; it is interpolation, not a fitting algorithm.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;

fn main() {
    // f(x) = 1 + 2x on [0, 2]. Each piece stores right Taylor derivatives.
    let curve =
        PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 3.0, 2.0], 2)
            .expect("valid PP representation");
    let value = curve.evaluate(1.5).expect("in-domain point");
    assert!((value - 4.0).abs() < 1.0e-12);
}
