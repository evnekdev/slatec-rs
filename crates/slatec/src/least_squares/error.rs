use core::fmt;

/// Failure before or during a safe nonlinear least-squares call.
///
/// This error covers Rust-side validation and contained callback failures. A
/// meaningful numerical completion from `SNLS1E` or `DNLS1E` is returned in a
/// [`super::LeastSquaresResult`] with a [`super::LeastSquaresStatus`] instead
/// of being discarded as an error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeastSquaresError {
    /// The initial parameter vector has no components.
    EmptyParameters,
    /// The requested residual count is zero.
    EmptyResiduals,
    /// The reviewed easy-driver contract requires at least as many residuals
    /// as fitted parameters.
    Underdetermined {
        /// Number of residuals (`M`).
        residuals: usize,
        /// Number of parameters (`N`).
        parameters: usize,
    },
    /// An initial parameter was NaN or infinite.
    NonFiniteInitialValue {
        /// Zero-based parameter index.
        index: usize,
    },
    /// The requested tolerance was negative, NaN, or infinite.
    InvalidTolerance,
    /// A count or workspace length cannot be represented as the selected
    /// GNU Fortran `INTEGER`.
    IntegerOverflow {
        /// Rust argument or internal value that overflowed.
        argument: &'static str,
    },
    /// Checked native workspace-size arithmetic overflowed `usize`.
    WorkspaceOverflow,
    /// The Rust residual callback panicked; the panic was caught before it
    /// could unwind through Fortran.
    CallbackPanicked,
    /// The callback produced a NaN or infinite residual.
    CallbackReturnedNonFinite {
        /// Zero-based residual index, not a parameter index.
        index: usize,
    },
    /// A callback attempted another callback-bearing SLATEC operation.
    NestedNativeCallback,
    /// Native code broke an invariant checked by the safe callback bridge.
    NativeContractViolation {
        /// Stable explanation of the violated invariant.
        detail: &'static str,
    },
    /// The native routine returned a status outside the reviewed `INFO=1..7`
    /// completion contract.
    NativeStatus {
        /// Raw `INFO` value returned by SLATEC.
        status: i32,
    },
}

impl fmt::Display for LeastSquaresError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyParameters => write!(formatter, "least-squares fitting needs parameters"),
            Self::EmptyResiduals => write!(formatter, "least-squares fitting needs residuals"),
            Self::Underdetermined {
                residuals,
                parameters,
            } => write!(
                formatter,
                "least-squares easy drivers require residual count {residuals} to be at least parameter count {parameters}"
            ),
            Self::NonFiniteInitialValue { index } => write!(
                formatter,
                "least-squares initial parameter at index {index} must be finite"
            ),
            Self::InvalidTolerance => write!(
                formatter,
                "least-squares tolerance must be finite and non-negative"
            ),
            Self::IntegerOverflow { argument } => write!(
                formatter,
                "least-squares {argument} does not fit Fortran INTEGER"
            ),
            Self::WorkspaceOverflow => write!(
                formatter,
                "least-squares workspace-size arithmetic overflowed"
            ),
            Self::CallbackPanicked => write!(formatter, "least-squares residual callback panicked"),
            Self::CallbackReturnedNonFinite { index } => write!(
                formatter,
                "least-squares residual callback returned a non-finite value at index {index}"
            ),
            Self::NestedNativeCallback => write!(
                formatter,
                "nested callback-based SLATEC calls are unsupported"
            ),
            Self::NativeContractViolation { detail } => write!(
                formatter,
                "native least-squares contract was violated: {detail}"
            ),
            Self::NativeStatus { status } => write!(
                formatter,
                "unknown least-squares easy-driver status {status}"
            ),
        }
    }
}

impl std::error::Error for LeastSquaresError {}
