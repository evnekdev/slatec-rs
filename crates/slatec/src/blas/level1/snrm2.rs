use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use validation::{count, increment, input_pointer};

/// Returns the Euclidean norm of `x` using unit stride.
pub fn snrm2(x: &[f32]) -> Result<f32, BlasError> {
    snrm2_strided(x.len(), x, 1)
}

/// Returns the Euclidean norm of `n` strided logical elements using `SNRM2`.
pub fn snrm2_strided(n: usize, x: &[f32], incx: isize) -> Result<f32, BlasError> {
    let mut n = count(n, "n")?;
    let native_increment = incx.checked_abs().ok_or(BlasError::ArithmeticOverflow)?;
    let mut incx = increment(native_increment, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `SNRM2` requires a positive increment. Reversing the checked
    // logical traversal cannot change a norm, so the native call uses the
    // validated base slice and `abs(incx)`.
    Ok(unsafe { raw::snrm2(&mut n, x, &mut incx) })
}
