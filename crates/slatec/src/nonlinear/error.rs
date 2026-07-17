use core::fmt;

/// Failure before or during a finite-difference SLATEC nonlinear easy-driver call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NonlinearError {
    /// The initial vector has no components.
    EmptySystem,
    /// An initial component is NaN or infinite.
    NonFiniteInitialValue {
        /// Index of the invalid component.
        index: usize,
    },
    /// The requested solver tolerance is negative, NaN, or infinite.
    InvalidTolerance,
    /// A value cannot be represented by the selected Fortran `INTEGER` ABI.
    IntegerOverflow {
        /// Name of the Rust argument or internal allocation.
        argument: &'static str,
    },
    /// The documented native workspace formula overflowed Rust `usize`.
    WorkspaceOverflow,
    /// The Rust residual callback panicked; the panic was contained before FFI.
    CallbackPanicked,
    /// The callback produced a NaN or infinite residual component.
    CallbackReturnedNonFinite {
        /// Index of the invalid residual component.
        index: usize,
    },
    /// A callback attempted another callback-bearing SLATEC operation.
    NestedNativeCallback,
    /// Native code violated a checked wrapper invariant.
    NativeContractViolation {
        /// Stable explanation of the failed invariant.
        detail: &'static str,
    },
    /// The native easy driver returned an undocumented completion code.
    NativeStatus {
        /// Raw SLATEC `INFO` value.
        status: i32,
    },
}

impl fmt::Display for NonlinearError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySystem => write!(
                formatter,
                "nonlinear systems must have at least one variable"
            ),
            Self::NonFiniteInitialValue { index } => {
                write!(
                    formatter,
                    "nonlinear initial value at index {index} must be finite"
                )
            }
            Self::InvalidTolerance => write!(
                formatter,
                "nonlinear tolerance must be finite and non-negative"
            ),
            Self::IntegerOverflow { argument } => {
                write!(
                    formatter,
                    "nonlinear {argument} does not fit Fortran INTEGER"
                )
            }
            Self::WorkspaceOverflow => {
                write!(formatter, "nonlinear workspace-size arithmetic overflowed")
            }
            Self::CallbackPanicked => write!(formatter, "nonlinear residual callback panicked"),
            Self::CallbackReturnedNonFinite { index } => {
                write!(
                    formatter,
                    "nonlinear residual callback returned a non-finite value at index {index}"
                )
            }
            Self::NestedNativeCallback => write!(
                formatter,
                "nested callback-based SLATEC calls are unsupported"
            ),
            Self::NativeContractViolation { detail } => {
                write!(
                    formatter,
                    "native nonlinear contract was violated: {detail}"
                )
            }
            Self::NativeStatus { status } => {
                write!(formatter, "unknown nonlinear easy-driver status {status}")
            }
        }
    }
}

impl std::error::Error for NonlinearError {}
