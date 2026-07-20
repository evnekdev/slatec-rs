use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer, matching_lengths, output_pointer};

/// Copies `x` into `y` using unit stride.
pub fn dcopy(x: &[f64], y: &mut [f64]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    dcopy_strided(x.len(), x, 1, y, 1)
}

/// Copies `n` logical elements using the original `DCOPY` routine.
pub fn dcopy_strided(
    n: usize,
    x: &[f64],
    incx: isize,
    y: &mut [f64],
    incy: isize,
) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut incy = increment(incy, "incy")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    let y = output_pointer(y, n as usize, incy as isize, "y")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: vectors were storage-checked for the logical BLAS access;
    // `y` is uniquely borrowed; all scalar arguments use the validated ABI.
    audited_candidate_call(|| unsafe { raw::dcopy(&mut n, x, &mut incx, y, &mut incy) });
    Ok(())
}
