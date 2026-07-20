use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use validation::{count, increment, input_pointer};

/// Returns the Euclidean norm of `x` using unit stride.
pub fn dnrm2(x: &[f64]) -> Result<f64, BlasError> {
    dnrm2_strided(x.len(), x, 1)
}

/// Returns the Euclidean norm of `n` strided logical elements using `DNRM2`.
pub fn dnrm2_strided(n: usize, x: &[f64], incx: isize) -> Result<f64, BlasError> {
    let mut n = count(n, "n")?;
    let native_increment = incx.checked_abs().ok_or(BlasError::ArithmeticOverflow)?;
    let mut incx = increment(native_increment, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `DNRM2` itself requires a positive increment. The checked safe
    // backing-store convention accepts a negative logical stride, whose
    // reversal does not affect a norm, so the native call receives `abs(incx)`
    // from the same validated base slice.
    Ok(unsafe { raw::dnrm2(&mut n, x, &mut incx) })
}
