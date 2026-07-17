use core::fmt;

/// Failure reported by a safe adaptive-quadrature call.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegrationError {
    InvalidBounds,
    InvalidTolerance,
    WorkspaceTooLarge,
    IntegerOverflow,
    MaximumSubdivisions,
    RoundoffDetected,
    BadIntegrandBehavior,
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
            Self::WorkspaceTooLarge => write!(formatter, "quadrature workspace is too large"),
            Self::IntegerOverflow => write!(formatter, "value does not fit Fortran INTEGER"),
            Self::MaximumSubdivisions => write!(formatter, "maximum subdivisions reached"),
            Self::RoundoffDetected => write!(formatter, "roundoff prevented requested accuracy"),
            Self::BadIntegrandBehavior => write!(formatter, "bad integrand behavior detected"),
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
