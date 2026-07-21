use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, matching_lengths, output_pointer};

/// Swaps `x` and `y` using unit stride.
pub fn sswap(x: &mut [f32], y: &mut [f32]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    sswap_strided(x.len(), x, 1, y, 1)
}

/// Swaps `n` logical elements using the original `SSWAP` routine.
pub fn sswap_strided(
    n: usize,
    x: &mut [f32],
    incx: isize,
    y: &mut [f32],
    incy: isize,
) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut incy = increment(incy, "incy")?;
    let x = output_pointer(x, n as usize, incx as isize, "x")?;
    let y = output_pointer(y, n as usize, incy as isize, "y")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: both vectors were storage-checked and are distinct mutable
    // borrows; all values match the validated GNU MinGW raw ABI.
    audited_candidate_call(|| unsafe { raw::sswap(&mut n, x, &mut incx, y, &mut incy) });
    Ok(())
}
