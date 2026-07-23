#![no_std]

pub fn dot(x: &[f64], y: &[f64]) -> Result<f64, slatec::linear_algebra::blas::BlasError> {
    slatec::linear_algebra::blas::level1::ddot(x, y)
}
