use core::fmt;

/// Failure reported by a safe adaptive-quadrature call.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegrationError {
    /// Interval bounds or a principal-value pole are invalid.
    InvalidBounds,
    /// Absolute or relative tolerances violate the driver's contract.
    InvalidTolerance,
    /// A requested PP derivative is outside the checked polynomial order.
    InvalidDerivativeOrder,
    /// A breakpoint is non-finite, outside the open interval, or at an endpoint.
    InvalidBreakpoint {
        /// Position in the caller's breakpoint slice.
        index: usize,
        /// Rejected breakpoint value.
        value: f64,
    },
    /// Two caller breakpoint positions contain the same value.
    DuplicateBreakpoint {
        /// First duplicate position.
        first: usize,
        /// Second duplicate position.
        second: usize,
    },
    /// An endpoint-weight exponent is outside the native contract.
    InvalidWeightParameter {
        /// Rust argument name.
        argument: &'static str,
        /// Rejected value.
        value: f64,
    },
    /// An oscillatory frequency is non-finite or otherwise unsupported.
    InvalidFrequency {
        /// Rejected frequency.
        value: f64,
    },
    /// Checked allocation-size arithmetic exceeded supported limits.
    WorkspaceTooLarge,
    /// The requested Chebyshev-moment table is too large.
    MomentWorkspaceTooLarge,
    /// A count cannot be represented by the selected Fortran `INTEGER`.
    IntegerOverflow,
    /// A tabulated overlapping-parabola interval has too few sampled points.
    InsufficientTabulatedPoints {
        /// Number of sample abscissas within the requested closed interval.
        found: usize,
    },
    /// The adaptive driver exhausted its subdivision limit.
    MaximumSubdivisions,
    /// Fourier-tail integration exhausted its cycle limit.
    MaximumCyclesReached,
    /// Native roundoff detection prevented the requested accuracy.
    RoundoffDetected,
    /// QUADPACK detected difficult local integrand behavior.
    BadIntegrandBehavior,
    /// A Fourier cycle reported difficult local behavior.
    BadCycleBehavior,
    /// QNG's fixed rule progression did not reach the requested accuracy.
    NonAdaptiveAccuracyNotReached,
    /// QNC79 rejected a zero-width interval.
    DegenerateInterval,
    /// The native driver could not converge.
    NonConvergence,
    /// The integral appears divergent or too slowly convergent.
    DivergentOrSlowlyConvergent,
    /// The Rust integrand panicked; the panic was contained before FFI.
    CallbackPanicked,
    /// The Rust integrand returned NaN or infinity.
    CallbackFailed,
    /// A callback attempted another callback-bearing SLATEC operation.
    NestedIntegration,
    /// A driver returned an undocumented native status value.
    NativeStatus(i32),
    /// Native outputs violated a checked wrapper invariant.
    NativeContractViolation,
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds => write!(formatter, "integration bounds are invalid"),
            Self::InvalidTolerance => write!(formatter, "integration tolerances are invalid"),
            Self::InvalidDerivativeOrder => {
                write!(
                    formatter,
                    "piecewise-polynomial derivative order is invalid"
                )
            }
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
            Self::InsufficientTabulatedPoints { found } => write!(
                formatter,
                "tabulated interval contains only {found} sample points; at least three are required"
            ),
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
