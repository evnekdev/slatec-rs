use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer, matching_lengths, output_pointer};

/// Performs `y = alpha * x + y` using unit stride.
///
/// ```no_run
/// # fn main() -> Result<(), slatec::blas::BlasError> {
/// use slatec::blas::level1::daxpy;
/// let mut y = [4.0, 5.0, 6.0];
/// daxpy(2.0, &[1.0, 2.0, 3.0], &mut y)?;
/// assert_eq!(y, [6.0, 9.0, 12.0]);
/// # Ok(())
/// # }
/// ```
pub fn daxpy(alpha: f64, x: &[f64], y: &mut [f64]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    daxpy_strided(x.len(), alpha, x, 1, y, 1)
}

/// Performs `y = alpha * x + y` for `n` strided logical elements.
pub fn daxpy_strided(
    n: usize,
    alpha: f64,
    x: &[f64],
    incx: isize,
    y: &mut [f64],
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
        raw::daxpy(&mut n, &mut alpha, x, &mut incx, y, &mut incy)
    });
    Ok(())
}
