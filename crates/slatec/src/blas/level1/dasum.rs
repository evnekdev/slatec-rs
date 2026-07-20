use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer};

/// Returns the sum of absolute values of `x` using unit stride.
pub fn dasum(x: &[f64]) -> Result<f64, BlasError> {
    dasum_strided(x.len(), x, 1)
}

/// Returns the sum of absolute values of `n` strided elements using `DASUM`.
pub fn dasum_strided(n: usize, x: &[f64], incx: isize) -> Result<f64, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    Ok(audited_candidate_call(|| unsafe {
        raw::dasum(&mut n, x, &mut incx)
    }))
}
