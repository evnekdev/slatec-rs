// slatec-safe-example
//! Multiply a checked piecewise polynomial by `x^2` with `DPFQAD`.

use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;
use slatec::quadrature::{PiecewiseQuadratureStatus, integrate_piecewise_polynomial};

fn main() {
    let constant = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1)
        .expect("valid constant piecewise polynomial");
    let result = integrate_piecewise_polynomial(&constant, 0, 0.0..=1.0, 1.0e-8, |x| x * x)
        .expect("piecewise-polynomial quadrature succeeds");
    assert_eq!(result.status, PiecewiseQuadratureStatus::Converged);
    assert!((result.value - 1.0 / 3.0).abs() < 1.0e-8);
}
