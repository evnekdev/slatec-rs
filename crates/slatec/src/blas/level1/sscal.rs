use slatec_sys::blas::level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, output_pointer};

/// Multiplies every element of `x` by `alpha` using unit stride.
pub fn sscal(alpha: f32, x: &mut [f32]) -> Result<(), BlasError> {
    sscal_strided(x.len(), alpha, x, 1)
}

/// Multiplies `n` logical elements of `x` by `alpha` using `SSCAL`.
pub fn sscal_strided(n: usize, alpha: f32, x: &mut [f32], incx: isize) -> Result<(), BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let mut alpha = alpha;
    let x = output_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(());
    }
    // Safety: `x` is a unique, storage-checked vector and the local scalar
    // values satisfy the validated raw ABI.
    audited_candidate_call(|| unsafe { raw::sscal(&mut n, &mut alpha, x, &mut incx) });
    Ok(())
}
