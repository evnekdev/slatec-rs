use core::fmt;

/// Failure before or during a safe bracketed scalar-root call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RootError {
    InvalidBracket,
    NonFiniteEndpoint { argument: &'static str },
    InvalidTolerance { argument: &'static str },
    InvalidInitialGuess,
    NoSignChange,
    CallbackPanicked,
    CallbackReturnedNonFinite,
    NestedNativeCallback,
    NativeStatus(i32),
    NativeContractViolation { detail: &'static str },
}

impl fmt::Display for RootError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBracket => write!(formatter, "root bracket endpoints must be distinct"),
            Self::NonFiniteEndpoint { argument } => {
                write!(formatter, "root {argument} must be finite")
            }
            Self::InvalidTolerance { argument } => {
                write!(formatter, "root {argument} tolerance is invalid")
            }
            Self::InvalidInitialGuess => {
                write!(
                    formatter,
                    "root initial guess must lie strictly inside the bracket"
                )
            }
            Self::NoSignChange => write!(formatter, "root bracket does not contain a sign change"),
            Self::CallbackPanicked => write!(formatter, "root callback panicked"),
            Self::CallbackReturnedNonFinite => {
                write!(formatter, "root callback returned a non-finite value")
            }
            Self::NestedNativeCallback => {
                write!(
                    formatter,
                    "nested callback-based SLATEC calls are unsupported"
                )
            }
            Self::NativeStatus(status) => write!(formatter, "unknown FZERO status {status}"),
            Self::NativeContractViolation { detail } => {
                write!(formatter, "native root contract was violated: {detail}")
            }
        }
    }
}

impl std::error::Error for RootError {}
