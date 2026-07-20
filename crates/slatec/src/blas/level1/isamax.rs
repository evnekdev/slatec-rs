use slatec_sys::families::blas_level1 as raw;

use super::super::{BlasError, validation};
use super::audited_candidate_call;
use validation::{count, increment, input_pointer};

/// Returns the zero-based index of the first maximum-magnitude value.
pub fn isamax(x: &[f32]) -> Result<Option<usize>, BlasError> {
    isamax_strided(x.len(), x, 1)
}

/// Returns the zero-based index of the first maximum-magnitude strided value.
pub fn isamax_strided(n: usize, x: &[f32], incx: isize) -> Result<Option<usize>, BlasError> {
    let mut n_fortran = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n, incx as isize, "x")?;
    if n == 0 {
        return Ok(None);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    let one_based = audited_candidate_call(|| unsafe { raw::isamax(&mut n_fortran, x, &mut incx) });
    let one_based = usize::try_from(one_based).map_err(|_| BlasError::NativeContractViolation {
        routine: "ISAMAX",
        detail: "returned a negative index",
    })?;
    if one_based == 0 || one_based > n {
        return Err(BlasError::NativeContractViolation {
            routine: "ISAMAX",
            detail: "returned an index outside the logical vector",
        });
    }
    Ok(Some(one_based - 1))
}
