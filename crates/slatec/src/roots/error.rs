use core::fmt;

/// Failure before or during a safe bracketed scalar-root call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RootError {
    /// The endpoints are equal, so they do not define a bracket.
    InvalidBracket,
    /// A bracket endpoint is NaN or infinite.
    NonFiniteEndpoint {
        /// Rust endpoint argument name.
        argument: &'static str,
    },
    /// A tolerance is negative or non-finite.
    InvalidTolerance {
        /// Rust tolerance argument name.
        argument: &'static str,
    },
    /// The initial suggestion is non-finite or outside the open bracket.
    InvalidInitialGuess,
    /// Endpoint values have the same nonzero sign.
    NoSignChange,
    /// The Rust callback panicked; the panic was contained before FFI.
    CallbackPanicked,
    /// The callback returned NaN or infinity.
    CallbackReturnedNonFinite,
    /// A callback attempted another callback-bearing SLATEC call.
    NestedNativeCallback,
    /// FZERO returned an undocumented status value.
    NativeStatus(i32),
    /// Native outputs violated a checked wrapper invariant.
    NativeContractViolation {
        /// Stable explanation of the violated postcondition.
        detail: &'static str,
    },
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
