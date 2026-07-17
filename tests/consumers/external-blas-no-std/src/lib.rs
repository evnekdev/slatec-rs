#![no_std]

pub fn dot(x: &[f64], y: &[f64]) -> Result<f64, slatec::blas::error::BlasError> {
    slatec::blas::level1::ddot(x, y)
}
