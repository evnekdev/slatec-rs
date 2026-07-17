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
    /// The expert solver's function-evaluation budget is zero or too large.
    InvalidMaximumFunctionEvaluations,
    /// A finite-difference step estimate is negative, NaN, or infinite.
    InvalidFiniteDifferenceStep,
    /// The expert initial step-bound factor is not finite and positive.
    InvalidStepBoundFactor,
    /// A banded Jacobian bandwidth is outside the square system.
    InvalidBandwidth {
        /// Name of the invalid bandwidth.
        argument: &'static str,
        /// Supplied bandwidth.
        value: usize,
        /// System dimension.
        dimension: usize,
    },
    /// User scaling has the wrong number of entries.
    InvalidScalingLength {
        /// Required number of scale factors.
        expected: usize,
        /// Supplied number of scale factors.
        actual: usize,
    },
    /// A user scale factor is non-finite or not strictly positive.
    InvalidScalingValue {
        /// Index of the invalid scale factor.
        index: usize,
    },
    /// Banded finite-difference controls were supplied to a dense user Jacobian.
    AnalyticJacobianRequiresDenseStructure,
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
    /// The user Jacobian callback panicked; the panic was contained before FFI.
    JacobianPanicked,
    /// The user Jacobian callback left a logical entry non-finite or unwritten.
    JacobianReturnedNonFinite {
        /// Zero-based logical row.
        row: usize,
        /// Zero-based logical column.
        column: usize,
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
            Self::InvalidMaximumFunctionEvaluations => write!(
                formatter,
                "maximum function evaluations must be positive and fit Fortran INTEGER"
            ),
            Self::InvalidFiniteDifferenceStep => write!(
                formatter,
                "finite-difference step estimate must be finite and non-negative"
            ),
            Self::InvalidStepBoundFactor => write!(
                formatter,
                "initial step-bound factor must be finite and positive"
            ),
            Self::InvalidBandwidth {
                argument,
                value,
                dimension,
            } => write!(
                formatter,
                "{argument} bandwidth {value} is invalid for dimension {dimension}"
            ),
            Self::InvalidScalingLength { expected, actual } => write!(
                formatter,
                "user scaling length {actual} does not match system dimension {expected}"
            ),
            Self::InvalidScalingValue { index } => write!(
                formatter,
                "user scaling value at index {index} must be finite and positive"
            ),
            Self::AnalyticJacobianRequiresDenseStructure => write!(
                formatter,
                "a user-supplied Jacobian is dense; banded structure applies only to finite differences"
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
            Self::JacobianPanicked => write!(formatter, "nonlinear Jacobian callback panicked"),
            Self::JacobianReturnedNonFinite { row, column } => write!(
                formatter,
                "nonlinear Jacobian callback left a non-finite entry at ({row}, {column})"
            ),
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

#[cfg(feature = "std")]
impl std::error::Error for NonlinearError {}
