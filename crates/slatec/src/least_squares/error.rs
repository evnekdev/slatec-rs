use core::fmt;

/// Failure before or during a safe nonlinear least-squares call.
///
/// This error covers Rust-side validation and contained callback failures. A
/// meaningful numerical completion from `SNLS1E`, `DNLS1E`, `SNLS1`, or
/// `DNLS1` is returned in a result with a [`super::LeastSquaresStatus`] rather
/// than being discarded as an error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeastSquaresError {
    /// The initial parameter vector has no components.
    EmptyParameters,
    /// The requested residual count is zero.
    EmptyResiduals,
    /// The reviewed SLATEC least-squares driver contract requires at least as
    /// many residuals as fitted parameters.
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
    /// An expert residual-reduction tolerance (`FTOL`) was invalid.
    InvalidFunctionTolerance,
    /// An expert parameter-step tolerance (`XTOL`) was invalid.
    InvalidParameterTolerance,
    /// An expert gradient-orthogonality tolerance (`GTOL`) was invalid.
    InvalidGradientTolerance,
    /// An expert `MAXFEV` control was zero.
    InvalidMaximumFunctionEvaluations,
    /// An expert forward-difference relative-error estimate (`EPSFCN`) was
    /// negative, NaN, or infinite.
    InvalidFiniteDifferenceStep,
    /// An expert initial step-bound multiplier (`FACTOR`) was invalid.
    InvalidStepBoundFactor,
    /// User scaling (`DIAG`) did not have one value per parameter.
    InvalidScalingLength {
        /// Expected parameter count.
        expected: usize,
        /// Actual number of supplied scaling entries.
        actual: usize,
    },
    /// A user scaling entry was non-finite or non-positive.
    InvalidScalingValue {
        /// Zero-based parameter/scaling index.
        index: usize,
    },
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
    /// The Rust dense-Jacobian callback panicked before returning to Fortran.
    JacobianPanicked,
    /// A dense-Jacobian callback left a logical derivative non-finite or
    /// unwritten.
    JacobianReturnedNonFinite {
        /// Zero-based residual row.
        row: usize,
        /// Zero-based parameter column.
        column: usize,
    },
    /// A callback attempted another callback-bearing SLATEC operation.
    NestedNativeCallback,
    /// Native code broke an invariant checked by the safe callback bridge.
    NativeContractViolation {
        /// Stable explanation of the violated invariant.
        detail: &'static str,
    },
    /// The native routine returned a status outside the reviewed `INFO=1..8`
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
                "least-squares drivers require residual count {residuals} to be at least parameter count {parameters}"
            ),
            Self::NonFiniteInitialValue { index } => write!(
                formatter,
                "least-squares initial parameter at index {index} must be finite"
            ),
            Self::InvalidTolerance => write!(
                formatter,
                "least-squares tolerance must be finite and non-negative"
            ),
            Self::InvalidFunctionTolerance => write!(
                formatter,
                "least-squares function tolerance must be finite and non-negative"
            ),
            Self::InvalidParameterTolerance => write!(
                formatter,
                "least-squares parameter tolerance must be finite and non-negative"
            ),
            Self::InvalidGradientTolerance => write!(
                formatter,
                "least-squares gradient tolerance must be finite and non-negative"
            ),
            Self::InvalidMaximumFunctionEvaluations => write!(
                formatter,
                "least-squares maximum function evaluations must be positive"
            ),
            Self::InvalidFiniteDifferenceStep => write!(
                formatter,
                "least-squares finite-difference step estimate must be finite and non-negative"
            ),
            Self::InvalidStepBoundFactor => write!(
                formatter,
                "least-squares step-bound factor must be finite and positive"
            ),
            Self::InvalidScalingLength { expected, actual } => write!(
                formatter,
                "least-squares user scaling needs {expected} values, received {actual}"
            ),
            Self::InvalidScalingValue { index } => write!(
                formatter,
                "least-squares user scaling entry at index {index} must be finite and positive"
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
            Self::JacobianPanicked => write!(formatter, "least-squares Jacobian callback panicked"),
            Self::JacobianReturnedNonFinite { row, column } => write!(
                formatter,
                "least-squares Jacobian callback left entry ({row}, {column}) non-finite or unwritten"
            ),
            Self::NestedNativeCallback => write!(
                formatter,
                "nested callback-based SLATEC calls are unsupported"
            ),
            Self::NativeContractViolation { detail } => write!(
                formatter,
                "native least-squares contract was violated: {detail}"
            ),
            Self::NativeStatus { status } => {
                write!(formatter, "unknown least-squares native status {status}")
            }
        }
    }
}

impl std::error::Error for LeastSquaresError {}
