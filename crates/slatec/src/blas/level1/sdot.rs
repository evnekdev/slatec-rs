use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer, matching_lengths};

/// Returns the unit-stride single-precision dot product.
pub fn sdot(x: &[f32], y: &[f32]) -> Result<f32, BlasError> {
    matching_lengths(x.len(), y.len())?;
    sdot_strided(x.len(), x, 1, y, 1)
}

/// Returns the single-precision dot product of `n` strided logical elements.
pub fn sdot_strided(
    n: usize,
    x: &[f32],
    incx: isize,
    y: &[f32],
    incy: isize,
) -> Result<f32, BlasError> {
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
        raw::sdot(&mut n, x, &mut incx, y, &mut incy)
    }))
}
