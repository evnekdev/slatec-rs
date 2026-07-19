//! Narrow link probe for the reviewed scalar piecewise-polynomial closure.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;

fn main() {
    let curve = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1)
        .expect("valid constant PP form");
    assert_eq!(curve.evaluate(0.5).expect("in-domain evaluation"), 1.0);
}
