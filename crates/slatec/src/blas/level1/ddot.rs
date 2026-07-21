use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer, matching_lengths};

/// Returns the unit-stride double-precision dot product.
pub fn ddot(x: &[f64], y: &[f64]) -> Result<f64, BlasError> {
    matching_lengths(x.len(), y.len())?;
    ddot_strided(x.len(), x, 1, y, 1)
}

/// Returns the double-precision dot product of `n` strided logical elements.
pub fn ddot_strided(
    n: usize,
    x: &[f64],
    incx: isize,
    y: &[f64],
    incy: isize,
) -> Result<f64, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut incy = increment(incy, "incy")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    let y = input_pointer(y, n as usize, incy as isize, "y")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: both vectors are read-only, storage-checked backing stores for
    // the selected routine and all scalar arguments fit its ABI.
    Ok(audited_candidate_call(|| unsafe {
        raw::ddot(&mut n, x, &mut incx, y, &mut incy)
    }))
}
