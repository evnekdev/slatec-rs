//! Checked safe wrappers for the initial real-valued BLAS Level 1 surface.
//!
//! A strided slice is a backing store, not a pre-adjusted pointer. For a
//! negative increment the original BLAS routine derives the logical starting
//! position from the base address; this module validates that position before
//! making the call and deliberately passes the base pointer to Fortran.

use slatec_sys::generated::numeric_array_subroutines as arrays;
use slatec_sys::generated::scalar_functions as functions;

use super::{BlasError, validation};
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
    unsafe { arrays::dcopy(&mut n, x, &mut incx, y, &mut incy) };
    Ok(())
}

/// Swaps `x` and `y` using unit stride.
pub fn dswap(x: &mut [f64], y: &mut [f64]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    dswap_strided(x.len(), x, 1, y, 1)
}

/// Swaps `n` logical elements using the original `DSWAP` routine.
pub fn dswap_strided(
    n: usize,
    x: &mut [f64],
    incx: isize,
    y: &mut [f64],
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
    unsafe { arrays::dswap(&mut n, x, &mut incx, y, &mut incy) };
    Ok(())
}

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
    unsafe { arrays::dscal(&mut n, &mut alpha, x, &mut incx) };
    Ok(())
}

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
    unsafe { arrays::daxpy(&mut n, &mut alpha, x, &mut incx, y, &mut incy) };
    Ok(())
}

/// Returns the unit-stride double-precision dot product.
pub fn ddot(x: &[f64], y: &[f64]) -> Result<f64, BlasError> {
    matching_lengths(x.len(), y.len())?;
    ddot_strided(x.len(), x, 1, y, 1)
}

/// Returns the double-precision dot product of `n` strided logical elements.
pub fn ddot_strided(
    n: usize,
    x: &[f64],
    incx: isize,
    y: &[f64],
    incy: isize,
) -> Result<f64, BlasError> {
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
    Ok(unsafe { functions::ddot(&mut n, x, &mut incx, y, &mut incy) })
}

/// Returns the Euclidean norm of `x` using unit stride.
pub fn dnrm2(x: &[f64]) -> Result<f64, BlasError> {
    dnrm2_strided(x.len(), x, 1)
}

/// Returns the Euclidean norm of `n` strided logical elements using `DNRM2`.
pub fn dnrm2_strided(n: usize, x: &[f64], incx: isize) -> Result<f64, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    Ok(unsafe { functions::dnrm2(&mut n, x, &mut incx) })
}

/// Returns the sum of absolute values of `x` using unit stride.
pub fn dasum(x: &[f64]) -> Result<f64, BlasError> {
    dasum_strided(x.len(), x, 1)
}

/// Returns the sum of absolute values of `n` strided elements using `DASUM`.
pub fn dasum_strided(n: usize, x: &[f64], incx: isize) -> Result<f64, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    Ok(unsafe { functions::dasum(&mut n, x, &mut incx) })
}

/// Returns the zero-based index of the first maximum-magnitude value.
pub fn idamax(x: &[f64]) -> Result<Option<usize>, BlasError> {
    idamax_strided(x.len(), x, 1)
}

/// Returns the zero-based index of the first maximum-magnitude strided value.
pub fn idamax_strided(n: usize, x: &[f64], incx: isize) -> Result<Option<usize>, BlasError> {
    let mut n_fortran = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n, incx as isize, "x")?;
    if n == 0 {
        return Ok(None);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    let one_based = unsafe { functions::idamax(&mut n_fortran, x, &mut incx) };
    let one_based = usize::try_from(one_based).map_err(|_| BlasError::NativeContractViolation {
        routine: "IDAMAX",
        detail: "returned a negative index",
    })?;
    if one_based == 0 || one_based > n {
        return Err(BlasError::NativeContractViolation {
            routine: "IDAMAX",
            detail: "returned an index outside the logical vector",
        });
    }
    Ok(Some(one_based - 1))
}

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
    unsafe { arrays::drot(&mut n, x, &mut incx, y, &mut incy, &mut c, &mut s) };
    Ok(())
}

/// Copies `x` into `y` using unit stride.
pub fn scopy(x: &[f32], y: &mut [f32]) -> Result<(), BlasError> {
    matching_lengths(x.len(), y.len())?;
    scopy_strided(x.len(), x, 1, y, 1)
}

/// Copies `n` logical elements using the original `SCOPY` routine.
pub fn scopy_strided(
    n: usize,
    x: &[f32],
    incx: isize,
    y: &mut [f32],
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
    unsafe { arrays::scopy(&mut n, x, &mut incx, y, &mut incy) };
    Ok(())
}

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
    unsafe { arrays::sswap(&mut n, x, &mut incx, y, &mut incy) };
    Ok(())
}

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
    unsafe { arrays::sscal(&mut n, &mut alpha, x, &mut incx) };
    Ok(())
}

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
    unsafe { arrays::saxpy(&mut n, &mut alpha, x, &mut incx, y, &mut incy) };
    Ok(())
}

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
    Ok(unsafe { functions::sdot(&mut n, x, &mut incx, y, &mut incy) })
}

/// Returns the Euclidean norm of `x` using unit stride.
pub fn snrm2(x: &[f32]) -> Result<f32, BlasError> {
    snrm2_strided(x.len(), x, 1)
}

/// Returns the Euclidean norm of `n` strided logical elements using `SNRM2`.
pub fn snrm2_strided(n: usize, x: &[f32], incx: isize) -> Result<f32, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    Ok(unsafe { functions::snrm2(&mut n, x, &mut incx) })
}

/// Returns the sum of absolute values of `x` using unit stride.
pub fn sasum(x: &[f32]) -> Result<f32, BlasError> {
    sasum_strided(x.len(), x, 1)
}

/// Returns the sum of absolute values of `n` strided elements using `SASUM`.
pub fn sasum_strided(n: usize, x: &[f32], incx: isize) -> Result<f32, BlasError> {
    let mut n = count(n, "n")?;
    let mut incx = increment(incx, "incx")?;
    let x = input_pointer(x, n as usize, incx as isize, "x")?;
    if n == 0 {
        return Ok(0.0);
    }
    // Safety: `x` is read-only and storage-checked, and the raw arguments
    // conform to the validated ABI profile.
    Ok(unsafe { functions::sasum(&mut n, x, &mut incx) })
}

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
    let one_based = unsafe { functions::isamax(&mut n_fortran, x, &mut incx) };
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
    unsafe { arrays::srot(&mut n, x, &mut incx, y, &mut incy, &mut c, &mut s) };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{BlasError, daxpy, daxpy_strided, validation};

    #[test]
    fn storage_calculation_handles_negative_increment() {
        assert_eq!(validation::required_storage(0, -1), Ok(0));
        assert_eq!(validation::required_storage(3, 2), Ok(5));
        assert_eq!(validation::required_storage(3, -2), Ok(5));
    }

    #[test]
    fn storage_calculation_rejects_zero_and_minimum_increment() {
        assert!(matches!(
            validation::required_storage(1, 0),
            Err(BlasError::InvalidIncrement { .. })
        ));
        assert_eq!(
            validation::required_storage(2, isize::MIN),
            Err(BlasError::ArithmeticOverflow)
        );
    }

    #[test]
    fn mismatched_contiguous_lengths_fail_before_native_call() {
        let mut y = [0.0];
        assert_eq!(
            daxpy(1.0, &[1.0, 2.0], &mut y),
            Err(BlasError::LengthMismatch { x: 2, y: 1 })
        );
    }

    #[test]
    fn invalid_strided_storage_fails_before_native_call() {
        let mut y = [0.0; 2];
        assert!(matches!(
            daxpy_strided(2, 1.0, &[1.0], 2, &mut y, 1),
            Err(BlasError::InsufficientStorage { argument: "x", .. })
        ));
    }
}
