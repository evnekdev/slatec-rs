use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use validation::{count, increment, matching_lengths, output_pointer};

/// Applies a plane rotation to two unit-stride vectors.
pub fn srot(c: f32, s: f32, x: &mut [f32], y: &mut [f32]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    srot_strided(x.len(), c, s, x, 1, y, 1)
}

/// Applies a plane rotation to `n` strided logical elements.
pub fn srot_strided(
    n: usize,
    c: f32,
    s: f32,
    x: &mut [f32],
    incx: isize,
    y: &mut [f32],
    incy: isize,
) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut incy = increment(incy, "incy")?;
    let mut c = c;
    let mut s = s;
    let x = output_pointer(x, n as usize, incx as isize, "x")?;
    let y = output_pointer(y, n as usize, incy as isize, "y")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: both vector arguments are distinct mutable, storage-checked
    // borrows; scalar arguments and symbols use the validated raw ABI.
    unsafe { raw::srot(&mut n, x, &mut incx, y, &mut incy, &mut c, &mut s) };
    Ok(())
}
