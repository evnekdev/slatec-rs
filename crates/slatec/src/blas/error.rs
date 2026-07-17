use core::fmt;

use super::selectors::Transpose;

/// Validation failure for a safe BLAS Level 1 operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlasError {
    /// A two-vector operation requires equal logical lengths.
    LengthMismatch {
        /// Logical length of the first vector.
        x: usize,
        /// Logical length of the second vector.
        y: usize,
    },
    /// BLAS increments must not be zero.
    InvalidIncrement {
        /// Rust argument name whose increment is invalid.
        argument: &'static str,
        /// Rejected increment.
        increment: isize,
    },
    /// A backing slice cannot cover the requested logical vector.
    InsufficientStorage {
        /// Rust argument name of the backing slice.
        argument: &'static str,
        /// Minimum number of elements required.
        required: usize,
        /// Number of elements supplied.
        actual: usize,
    },
    /// A matrix leading dimension is smaller than its BLAS minimum.
    InvalidLeadingDimension {
        /// Rust name of the leading-dimension argument.
        argument: &'static str,
        /// Leading dimension supplied by the caller.
        provided: usize,
        /// Minimum permitted leading dimension.
        minimum: usize,
    },
    /// A matrix slice cannot cover the declared column-major storage.
    InsufficientMatrixStorage {
        /// Rust argument name of the matrix slice.
        argument: &'static str,
        /// Minimum number of column-major elements required.
        required: usize,
        /// Number of matrix elements supplied.
        actual: usize,
    },
    /// Explicit dimensions do not describe a valid BLAS operation.
    MatrixDimensionMismatch {
        /// Safe operation that rejected the dimensions.
        operation: &'static str,
        /// Stable explanation of the incompatible dimensions.
        detail: &'static str,
    },
    /// A band width is invalid for the requested operation.
    InvalidBandWidth {
        /// Rust band-width argument name.
        argument: &'static str,
        /// Rejected band width.
        value: usize,
    },
    /// A selector has no safe representation for the requested real routine.
    UnsupportedTranspose {
        /// Safe operation that rejected the selector.
        operation: &'static str,
        /// Unsupported transpose mode.
        transpose: Transpose,
    },
    /// A count cannot be represented by the selected Fortran `INTEGER`.
    IntegerOverflow {
        /// Rust argument whose count did not fit.
        argument: &'static str,
        /// Rejected non-negative value.
        value: usize,
    },
    /// An increment cannot be represented by the selected Fortran `INTEGER`.
    IncrementOverflow {
        /// Rust argument whose signed increment did not fit.
        argument: &'static str,
        /// Rejected signed value.
        value: isize,
    },
    /// Checked storage or index arithmetic overflowed.
    ArithmeticOverflow,
    /// The native routine violated a precondition established by its raw ABI.
    NativeContractViolation {
        /// Original SLATEC/BLAS routine name.
        routine: &'static str,
        /// Stable explanation of the violated postcondition.
        detail: &'static str,
    },
}

impl fmt::Display for BlasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { x, y } => {
                write!(formatter, "vector lengths differ: {x} and {y}")
            }
            Self::InvalidIncrement {
                argument,
                increment,
            } => write!(
                formatter,
                "{argument} increment must be nonzero, got {increment}"
            ),
            Self::InsufficientStorage {
                argument,
                required,
                actual,
            } => write!(
                formatter,
                "{argument} needs {required} elements for its logical vector, but has {actual}"
            ),
            Self::InvalidLeadingDimension {
                argument,
                provided,
                minimum,
            } => write!(
                formatter,
                "{argument} leading dimension {provided} is smaller than {minimum}"
            ),
            Self::InsufficientMatrixStorage {
                argument,
                required,
                actual,
            } => write!(
                formatter,
                "{argument} needs {required} column-major elements, but has {actual}"
            ),
            Self::MatrixDimensionMismatch { operation, detail } => {
                write!(
                    formatter,
                    "{operation} has incompatible dimensions: {detail}"
                )
            }
            Self::InvalidBandWidth { argument, value } => {
                write!(formatter, "{argument} band width {value} is invalid")
            }
            Self::UnsupportedTranspose {
                operation,
                transpose,
            } => write!(formatter, "{operation} does not support {transpose:?}"),
            Self::IntegerOverflow { argument, value } => {
                write!(
                    formatter,
                    "{argument} value {value} does not fit Fortran INTEGER"
                )
            }
            Self::IncrementOverflow { argument, value } => {
                write!(
                    formatter,
                    "{argument} increment {value} does not fit Fortran INTEGER"
                )
            }
            Self::ArithmeticOverflow => write!(formatter, "BLAS vector arithmetic overflowed"),
            Self::NativeContractViolation { routine, detail } => {
                write!(
                    formatter,
                    "{routine} violated its native contract: {detail}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlasError {}
