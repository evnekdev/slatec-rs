use core::fmt;

/// Validation failure for a safe BLAS Level 1 operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlasError {
    /// A two-vector operation requires equal logical lengths.
    LengthMismatch { x: usize, y: usize },
    /// BLAS increments must not be zero.
    InvalidIncrement {
        argument: &'static str,
        increment: isize,
    },
    /// A backing slice cannot cover the requested logical vector.
    InsufficientStorage {
        argument: &'static str,
        required: usize,
        actual: usize,
    },
    /// A count cannot be represented by the selected Fortran `INTEGER`.
    IntegerOverflow {
        argument: &'static str,
        value: usize,
    },
    /// An increment cannot be represented by the selected Fortran `INTEGER`.
    IncrementOverflow {
        argument: &'static str,
        value: isize,
    },
    /// Checked storage or index arithmetic overflowed.
    ArithmeticOverflow,
    /// The native routine violated a precondition established by its raw ABI.
    NativeContractViolation {
        routine: &'static str,
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

impl std::error::Error for BlasError {}
