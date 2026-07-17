use core::fmt;

/// Failure reported by a safe adaptive-quadrature call.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegrationError {
    InvalidBounds,
    InvalidTolerance,
    InvalidBreakpoint { index: usize, value: f64 },
    DuplicateBreakpoint { first: usize, second: usize },
    InvalidWeightParameter { argument: &'static str, value: f64 },
    InvalidFrequency { value: f64 },
    WorkspaceTooLarge,
    MomentWorkspaceTooLarge,
    IntegerOverflow,
    MaximumSubdivisions,
    MaximumCyclesReached,
    RoundoffDetected,
    BadIntegrandBehavior,
    BadCycleBehavior,
    NonAdaptiveAccuracyNotReached,
    DegenerateInterval,
    NonConvergence,
    DivergentOrSlowlyConvergent,
    CallbackPanicked,
    CallbackFailed,
    NestedIntegration,
    NativeStatus(i32),
    NativeContractViolation,
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds => write!(formatter, "integration bounds are invalid"),
            Self::InvalidTolerance => write!(formatter, "integration tolerances are invalid"),
            Self::InvalidBreakpoint { index, value } => {
                write!(formatter, "breakpoint {index} is invalid: {value}")
            }
            Self::DuplicateBreakpoint { first, second } => {
                write!(formatter, "breakpoints {first} and {second} are duplicates")
            }
            Self::InvalidWeightParameter { argument, value } => {
                write!(formatter, "invalid weight parameter {argument}: {value}")
            }
            Self::InvalidFrequency { value } => write!(formatter, "invalid frequency: {value}"),
            Self::WorkspaceTooLarge => write!(formatter, "quadrature workspace is too large"),
            Self::MomentWorkspaceTooLarge => {
                write!(formatter, "quadrature moment workspace is too large")
            }
            Self::IntegerOverflow => write!(formatter, "value does not fit Fortran INTEGER"),
            Self::MaximumSubdivisions => write!(formatter, "maximum subdivisions reached"),
            Self::MaximumCyclesReached => write!(formatter, "maximum Fourier cycles reached"),
            Self::RoundoffDetected => write!(formatter, "roundoff prevented requested accuracy"),
            Self::BadIntegrandBehavior => write!(formatter, "bad integrand behavior detected"),
            Self::BadCycleBehavior => {
                write!(formatter, "bad integrand behavior on a Fourier cycle")
            }
            Self::NonAdaptiveAccuracyNotReached => {
                write!(
                    formatter,
                    "non-adaptive rule progression did not reach requested accuracy"
                )
            }
            Self::DegenerateInterval => write!(formatter, "integration interval is degenerate"),
            Self::NonConvergence => write!(formatter, "quadrature did not converge"),
            Self::DivergentOrSlowlyConvergent => {
                write!(formatter, "integral is divergent or slowly convergent")
            }
            Self::CallbackPanicked => write!(formatter, "integrand callback panicked"),
            Self::CallbackFailed => write!(formatter, "integrand returned a non-finite value"),
            Self::NestedIntegration => {
                write!(
                    formatter,
                    "nested callback-based integration is unsupported"
                )
            }
            Self::NativeStatus(status) => write!(formatter, "unknown native status {status}"),
            Self::NativeContractViolation => write!(formatter, "native contract was violated"),
        }
    }
}

impl std::error::Error for IntegrationError {}
