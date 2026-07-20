//! Evaluate a PP-form polynomial and one of its derivatives.
//!
//! <!-- slatec-safe-example -->
//!
//! Requires `std`, a native backend, and `piecewise-polynomial`. `PPVAL` and
//! `DPPVAL` receive the exact owned coefficient allocation without copying.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;

fn main() {
    // 1 + 2x + 3x^2: PP coefficients are f(0), f'(0), f''(0).
    let curve = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0, 2.0, 6.0], 3)
        .expect("valid PP representation");
    let mut values = [0.0; 3];
    curve
        .evaluate_into(&[0.0, 0.5, 1.0], &mut values)
        .expect("in-domain batch");
    assert_eq!(values, [1.0, 2.75, 6.0]);
    assert!((curve.derivative(0.5, 1).expect("valid derivative") - 5.0).abs() < 1.0e-12);
}
