//! Shared checked vector validation used immediately before raw BLAS calls.

use slatec_core::{to_fortran_increment, to_fortran_integer};
use slatec_sys::FortranInteger;

use super::BlasError;

pub(crate) fn count(value: usize, argument: &'static str) -> Result<FortranInteger, BlasError> {
    to_fortran_integer(value).map_err(|_| BlasError::IntegerOverflow { argument, value })
}

pub(crate) fn increment(value: isize, argument: &'static str) -> Result<FortranInteger, BlasError> {
    to_fortran_increment(value).map_err(|_| BlasError::IncrementOverflow { argument, value })
}

/// Returns the number of backing elements needed for `n` values with `inc`.
pub(crate) fn required_storage(n: usize, inc: isize) -> Result<usize, BlasError> {
    if inc == 0 {
        return Err(BlasError::InvalidIncrement {
            argument: "vector",
            increment: inc,
        });
    }
    if n == 0 {
        return Ok(0);
    }
    let magnitude = inc.checked_abs().ok_or(BlasError::ArithmeticOverflow)?;
    let magnitude = usize::try_from(magnitude).map_err(|_| BlasError::ArithmeticOverflow)?;
    n.checked_sub(1)
        .and_then(|last| last.checked_mul(magnitude))
        .and_then(|last| last.checked_add(1))
        .ok_or(BlasError::ArithmeticOverflow)
}

pub(crate) fn validate_vector(
    n: usize,
    inc: isize,
    actual: usize,
    argument: &'static str,
) -> Result<usize, BlasError> {
    if inc == 0 {
        return Err(BlasError::InvalidIncrement {
            argument,
            increment: inc,
        });
    }
    let required = required_storage(n, inc)?;
    if actual < required {
        return Err(BlasError::InsufficientStorage {
            argument,
            required,
            actual,
        });
    }
    // Original netlib BLAS adjusts its own base index for a negative
    // increment. Retain the derived start to prove it lies in this slice.
    Ok(if inc < 0 {
        required.saturating_sub(1)
    } else {
        0
    })
}

pub(crate) fn input_pointer<T>(
    values: &[T],
    n: usize,
    inc: isize,
    argument: &'static str,
) -> Result<*mut T, BlasError> {
    let _start = validate_vector(n, inc, values.len(), argument)?;
    Ok(values.as_ptr().cast_mut())
}

pub(crate) fn output_pointer<T>(
    values: &mut [T],
    n: usize,
    inc: isize,
    argument: &'static str,
) -> Result<*mut T, BlasError> {
    let _start = validate_vector(n, inc, values.len(), argument)?;
    Ok(values.as_mut_ptr())
}

pub(crate) fn matching_lengths(x: usize, y: usize) -> Result<(), BlasError> {
    if x == y {
        Ok(())
    } else {
        Err(BlasError::LengthMismatch { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::{count, required_storage, validate_vector};
    use crate::blas::BlasError;

    #[test]
    fn storage_calculation_handles_negative_increment() {
        assert_eq!(required_storage(0, -1), Ok(0));
        assert_eq!(required_storage(3, 2), Ok(5));
        assert_eq!(required_storage(3, -2), Ok(5));
    }

    #[test]
    fn storage_calculation_rejects_zero_and_minimum_increment() {
        assert!(matches!(
            required_storage(1, 0),
            Err(BlasError::InvalidIncrement { .. })
        ));
        assert_eq!(
            required_storage(2, isize::MIN),
            Err(BlasError::ArithmeticOverflow)
        );
    }

    #[test]
    fn validates_storage_and_negative_start_position() {
        assert_eq!(validate_vector(3, -2, 5, "x"), Ok(4));
        assert!(matches!(
            validate_vector(2, 2, 1, "x"),
            Err(BlasError::InsufficientStorage { argument: "x", .. })
        ));
    }

    #[test]
    fn count_overflow_is_rejected_without_native_linking() {
        assert!(matches!(
            count(i32::MAX as usize + 1, "n"),
            Err(BlasError::IntegerOverflow { .. })
        ));
    }
}
