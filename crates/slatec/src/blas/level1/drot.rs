use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use validation::{count, increment, matching_lengths, output_pointer};

/// Applies a plane rotation to two unit-stride vectors.
pub fn drot(c: f64, s: f64, x: &mut [f64], y: &mut [f64]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    drot_strided(x.len(), c, s, x, 1, y, 1)
}

/// Applies a plane rotation to `n` strided logical elements.
pub fn drot_strided(
    n: usize,
    c: f64,
    s: f64,
    x: &mut [f64],
    incx: isize,
    y: &mut [f64],
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
    unsafe { raw::drot(&mut n, x, &mut incx, y, &mut incy, &mut c, &mut s) };
    Ok(())
}
