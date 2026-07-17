//! Checked conversions for the supported raw Fortran ABI profile.

use core::fmt;

use slatec_sys::FortranInteger;

/// A value cannot be represented by GNU Fortran's default `INTEGER` in the
/// supported GNU MinGW profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerRangeError {
    /// A Rust slice length or count is too large for a Fortran `INTEGER`.
    Unsigned {
        /// The non-negative Rust value that did not fit.
        value: usize,
    },
    /// A signed stride is too large for a Fortran `INTEGER`.
    Signed {
        /// The signed Rust value that did not fit.
        value: isize,
    },
}

impl fmt::Display for IntegerRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsigned { value } => write!(formatter, "{value} does not fit Fortran INTEGER"),
            Self::Signed { value } => write!(formatter, "{value} does not fit Fortran INTEGER"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IntegerRangeError {}

/// Converts a non-negative Rust count to the selected profile's Fortran
/// `INTEGER` without truncation.
pub fn to_fortran_integer(value: usize) -> Result<FortranInteger, IntegerRangeError> {
    FortranInteger::try_from(value).map_err(|_| IntegerRangeError::Unsigned { value })
}

/// Converts a signed Rust stride to the selected profile's Fortran `INTEGER`
/// without truncation. This is intentionally separate from count conversion.
pub fn to_fortran_increment(value: isize) -> Result<FortranInteger, IntegerRangeError> {
    FortranInteger::try_from(value).map_err(|_| IntegerRangeError::Signed { value })
}

#[cfg(test)]
mod tests {
    use super::{IntegerRangeError, to_fortran_increment, to_fortran_integer};

    #[test]
    fn converts_fortran_integer_boundaries() {
        assert_eq!(to_fortran_integer(0), Ok(0));
        assert_eq!(to_fortran_integer(1), Ok(1));
        assert_eq!(to_fortran_integer(i32::MAX as usize), Ok(i32::MAX));
        assert_eq!(
            to_fortran_integer(i32::MAX as usize + 1),
            Err(IntegerRangeError::Unsigned {
                value: i32::MAX as usize + 1
            })
        );
    }

    #[test]
    fn converts_signed_increments_without_abs_overflow() {
        assert_eq!(to_fortran_increment(-1), Ok(-1));
        assert_eq!(to_fortran_increment(i32::MIN as isize), Ok(i32::MIN));
        assert_eq!(
            to_fortran_increment(isize::MIN),
            Err(IntegerRangeError::Signed { value: isize::MIN })
        );
    }
}
