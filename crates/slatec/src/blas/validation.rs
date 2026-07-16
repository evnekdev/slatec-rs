//! Shared checked vector validation used immediately before raw BLAS calls.

use slatec_core::{to_fortran_increment, to_fortran_integer};
use slatec_sys::FortranInteger;

use super::{BlasError, Side, Transpose};

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

pub(crate) fn validate_leading_dimension(
    argument: &'static str,
    leading_dimension: usize,
    minimum: usize,
) -> Result<(), BlasError> {
    if leading_dimension < minimum {
        return Err(BlasError::InvalidLeadingDimension {
            argument,
            provided: leading_dimension,
            minimum,
        });
    }
    Ok(())
}

/// Calculates the conservative column-major backing length.
pub(crate) fn required_col_major_len(
    rows: usize,
    cols: usize,
    leading_dimension: usize,
) -> Result<usize, BlasError> {
    validate_leading_dimension("leading_dimension", leading_dimension, rows.max(1))?;
    if cols == 0 {
        return Ok(0);
    }
    leading_dimension
        .checked_mul(cols)
        .ok_or(BlasError::ArithmeticOverflow)
}

pub(crate) fn validate_matrix(
    argument: &'static str,
    rows: usize,
    cols: usize,
    leading_dimension: usize,
    actual: usize,
) -> Result<(), BlasError> {
    let required = required_col_major_len(rows, cols, leading_dimension)?;
    if actual < required {
        return Err(BlasError::InsufficientMatrixStorage {
            argument,
            required,
            actual,
        });
    }
    Ok(())
}

pub(crate) fn gemv_logical_lengths(
    transpose: Transpose,
    rows: usize,
    cols: usize,
) -> (usize, usize) {
    if transpose.is_transposed() {
        (rows, cols)
    } else {
        (cols, rows)
    }
}

pub(crate) fn gemm_stored_shapes(
    trans_a: Transpose,
    trans_b: Transpose,
    m: usize,
    n: usize,
    k: usize,
) -> ((usize, usize), (usize, usize)) {
    let a = if trans_a.is_transposed() {
        (k, m)
    } else {
        (m, k)
    };
    let b = if trans_b.is_transposed() {
        (n, k)
    } else {
        (k, n)
    };
    (a, b)
}

pub(crate) fn triangular_order(side: Side, m: usize, n: usize) -> usize {
    if matches!(side, Side::Left) { m } else { n }
}

pub(crate) fn validate_band_matrix(
    rows: usize,
    cols: usize,
    lower_bandwidth: usize,
    upper_bandwidth: usize,
    leading_dimension: usize,
    actual: usize,
) -> Result<(), BlasError> {
    if (rows > 0 && lower_bandwidth >= rows) || (cols > 0 && upper_bandwidth >= cols) {
        return Err(BlasError::InvalidBandWidth {
            argument: "bandwidth",
            value: lower_bandwidth.max(upper_bandwidth),
        });
    }
    let minimum = lower_bandwidth
        .checked_add(upper_bandwidth)
        .and_then(|value| value.checked_add(1))
        .ok_or(BlasError::ArithmeticOverflow)?;
    validate_leading_dimension("lda", leading_dimension, minimum)?;
    let required = leading_dimension
        .checked_mul(cols)
        .ok_or(BlasError::ArithmeticOverflow)?;
    if actual < required {
        return Err(BlasError::InsufficientMatrixStorage {
            argument: "a",
            required,
            actual,
        });
    }
    Ok(())
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

    #[test]
    fn validates_column_major_storage_with_checked_arithmetic() {
        assert_eq!(super::required_col_major_len(2, 3, 4), Ok(12));
        assert_eq!(super::required_col_major_len(0, 0, 1), Ok(0));
        assert!(matches!(
            super::required_col_major_len(3, 1, 2),
            Err(BlasError::InvalidLeadingDimension { .. })
        ));
        assert!(matches!(
            super::required_col_major_len(1, usize::MAX, usize::MAX),
            Err(BlasError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn derives_transpose_shapes_and_band_storage() {
        assert_eq!(
            super::gemv_logical_lengths(super::Transpose::None, 2, 3),
            (3, 2)
        );
        assert_eq!(
            super::gemv_logical_lengths(super::Transpose::Transpose, 2, 3),
            (2, 3)
        );
        assert_eq!(
            super::gemm_stored_shapes(super::Transpose::Transpose, super::Transpose::None, 2, 3, 4),
            ((4, 2), (4, 3))
        );
        assert_eq!(super::triangular_order(super::Side::Right, 2, 3), 3);
        assert!(super::validate_band_matrix(3, 4, 1, 1, 3, 12).is_ok());
        assert!(matches!(
            super::validate_band_matrix(3, 4, 1, 1, 2, 8),
            Err(BlasError::InvalidLeadingDimension { .. })
        ));
    }
}
