use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, output_pointer};

/// Multiplies every element of `x` by `alpha` using unit stride.
pub fn dscal(alpha: f64, x: &mut [f64]) -> Result<(), BlasError> {
    dscal_strided(x.len(), alpha, x, 1)
}

/// Multiplies `n` logical elements of `x` by `alpha` using `DSCAL`.
pub fn dscal_strided(n: usize, alpha: f64, x: &mut [f64], incx: isize) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut alpha = alpha;
    let x = output_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: `x` is a unique, storage-checked vector and the local scalar
    // values satisfy the validated raw ABI.
    audited_candidate_call(|| unsafe { raw::dscal(&mut n, &mut alpha, x, &mut incx) });
    Ok(())
}
