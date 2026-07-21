use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer, matching_lengths, output_pointer};

/// Performs `y = alpha * x + y` using unit stride.
pub fn saxpy(alpha: f32, x: &[f32], y: &mut [f32]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    saxpy_strided(x.len(), alpha, x, 1, y, 1)
}

/// Performs `y = alpha * x + y` for `n` strided logical elements.
pub fn saxpy_strided(
    n: usize,
    alpha: f32,
    x: &[f32],
    incx: isize,
    y: &mut [f32],
    incy: isize,
) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut incy = increment(incy, "incy")?;
    let mut alpha = alpha;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    let y = output_pointer(y, n as usize, incy as isize, "y")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: `x` is read only, `y` is uniquely borrowed, both vectors were
    // storage-checked, and all arguments use the validated raw ABI.
    audited_candidate_call(|| unsafe {
        raw::saxpy(&mut n, &mut alpha, x, &mut incx, y, &mut incy)
    });
    Ok(())
}
