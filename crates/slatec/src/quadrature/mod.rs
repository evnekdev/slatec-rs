//! Safe callback-based adapters for a focused SLATEC QUADPACK family.
//!
//! The original QUADPACK Fortran drivers remain the numerical implementations.
//! Calls are serialized for the validated GNU MinGW runtime profile because
//! callback registration and legacy error state are process-global. Integrand
//! panics and non-finite results are contained in the trampoline and reported
//! after Fortran returns.

#[cfg(feature = "quadrature-basic")]
mod adaptive;
mod callback;
mod error;
#[cfg(any(
    feature = "quadrature-breakpoints",
    feature = "quadrature-weighted",
    feature = "quadrature-oscillatory",
    feature = "quadrature-fourier",
    feature = "quadrature-nonadaptive"
))]
mod extended;
mod validation;

#[cfg(feature = "quadrature-basic")]
pub use adaptive::{
    integrate, integrate_f32, integrate_infinite, integrate_infinite_f32,
    integrate_principal_value, integrate_principal_value_f32, integrate_singular,
    integrate_singular_f32,
};
pub use error::IntegrationError;
#[cfg(feature = "quadrature-fourier")]
pub use extended::{integrate_fourier_tail, integrate_fourier_tail_f32};
#[cfg(feature = "quadrature-nonadaptive")]
pub use extended::{
    integrate_nc79, integrate_nc79_f32, integrate_non_adaptive, integrate_non_adaptive_f32,
};
#[cfg(feature = "quadrature-oscillatory")]
pub use extended::{integrate_oscillatory, integrate_oscillatory_f32};
#[cfg(feature = "quadrature-weighted")]
pub use extended::{integrate_weighted_endpoints, integrate_weighted_endpoints_f32};
#[cfg(feature = "quadrature-breakpoints")]
pub use extended::{integrate_with_breakpoints, integrate_with_breakpoints_f32};

/// Gauss-Kronrod rule used by QAG/DQAG finite-interval integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrationRule {
    /// 15-point Gauss-Kronrod pair.
    Points15,
    /// 21-point Gauss-Kronrod pair.
    Points21,
    /// 31-point Gauss-Kronrod pair.
    Points31,
    /// 41-point Gauss-Kronrod pair.
    Points41,
    /// 51-point Gauss-Kronrod pair.
    Points51,
    /// 61-point Gauss-Kronrod pair.
    Points61,
}

#[cfg(feature = "quadrature-basic")]
impl IntegrationRule {
    pub(crate) const fn key(self) -> i32 {
        match self {
            Self::Points15 => 1,
            Self::Points21 => 2,
            Self::Points31 => 3,
            Self::Points41 => 4,
            Self::Points51 => 5,
            Self::Points61 => 6,
        }
    }
}

/// Resource and accuracy controls for the adaptive drivers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegrationOptions {
    /// Requested absolute error bound (`EPSABS`).
    pub absolute_tolerance: f64,
    /// Requested relative error bound (`EPSREL`).
    pub relative_tolerance: f64,
    /// Maximum number of adaptive subintervals (`LIMIT`).
    pub limit: usize,
    /// Used by `integrate`/`integrate_f32`; extrapolating and weighted drivers
    /// select their own local rules.
    pub rule: IntegrationRule,
}

impl IntegrationOptions {
    /// A practical default for the single-precision QAG-family drivers.
    pub const fn single_precision() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-5,
            limit: 100,
            rule: IntegrationRule::Points21,
        }
    }
}

impl Default for IntegrationOptions {
    fn default() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-10,
            limit: 100,
            rule: IntegrationRule::Points21,
        }
    }
}

/// A completed quadrature estimate and its native diagnostics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegrationResult<T = f64> {
    /// Approximation to the integral (`RESULT`).
    pub value: T,
    /// Native absolute-error estimate (`ABSERR`).
    pub estimated_error: T,
    /// Number of integrand evaluations (`NEVAL`).
    pub evaluations: usize,
    /// Number of subintervals retained (`LAST`).
    pub intervals: usize,
}

/// Meaning of the endpoint weight applied by QAWS/DQAWS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointWeight {
    /// `(x-a)^alpha * (b-x)^beta`.
    Algebraic,
    /// Algebraic weight multiplied by `log(x-a)`.
    AlgebraicLogLower,
    /// Algebraic weight multiplied by `log(b-x)`.
    AlgebraicLogUpper,
    /// Algebraic weight multiplied by both endpoint logarithms.
    AlgebraicLogBoth,
}

#[cfg(feature = "quadrature-weighted")]
impl EndpointWeight {
    pub(crate) const fn native_selector(self) -> i32 {
        match self {
            Self::Algebraic => 1,
            Self::AlgebraicLogLower => 2,
            Self::AlgebraicLogUpper => 3,
            Self::AlgebraicLogBoth => 4,
        }
    }
}

/// Trigonometric weight applied by QAWO/DQAWO and QAWF/DQAWF.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OscillatoryWeight {
    /// Cosine weight (`INTEGR = 1`).
    Cosine,
    /// Sine weight (`INTEGR = 2`).
    Sine,
}

#[cfg(any(feature = "quadrature-oscillatory", feature = "quadrature-fourier"))]
impl OscillatoryWeight {
    pub(crate) const fn native_selector(self) -> i32 {
        match self {
            Self::Cosine => 1,
            Self::Sine => 2,
        }
    }
}

/// Resource and accuracy controls for finite oscillatory integration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OscillatoryOptions {
    /// Requested absolute error bound (`EPSABS`).
    pub absolute_tolerance: f64,
    /// Requested relative error bound (`EPSREL`).
    pub relative_tolerance: f64,
    /// Maximum adaptive subinterval count (`LIMIT`).
    pub subdivision_limit: usize,
    /// Upper bound on stored Chebyshev-moment levels. The safe API discards
    /// these moments after the call.
    pub maximum_moments: usize,
}

impl OscillatoryOptions {
    /// Returns practical defaults for the single-precision QAWO driver.
    pub const fn single_precision() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-5,
            subdivision_limit: 100,
            maximum_moments: 25,
        }
    }
}

impl Default for OscillatoryOptions {
    fn default() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-10,
            subdivision_limit: 100,
            maximum_moments: 25,
        }
    }
}

/// Resource and accuracy controls for infinite Fourier-tail integration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FourierOptions {
    /// Requested absolute error bound (`EPSABS`).
    pub absolute_tolerance: f64,
    /// Subinterval limit used within each Fourier cycle (`LIMIT`).
    pub subdivision_limit: usize,
    /// Maximum number of Fourier cycles (`LIMLST`).
    pub cycle_limit: usize,
    /// Maximum Chebyshev-moment levels (`MAXP1`).
    pub maximum_moments: usize,
}

impl FourierOptions {
    /// Returns practical defaults for the single-precision QAWF driver.
    pub const fn single_precision() -> Self {
        Self {
            absolute_tolerance: 1.0e-5,
            subdivision_limit: 100,
            cycle_limit: 50,
            maximum_moments: 25,
        }
    }
}

impl Default for FourierOptions {
    fn default() -> Self {
        Self {
            absolute_tolerance: 1.0e-10,
            subdivision_limit: 100,
            cycle_limit: 50,
            maximum_moments: 25,
        }
    }
}

/// Accuracy controls for QNG/DQNG's fixed nested-rule progression.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonAdaptiveOptions {
    /// Requested absolute error bound (`EPSABS`).
    pub absolute_tolerance: f64,
    /// Requested relative error bound (`EPSREL`).
    pub relative_tolerance: f64,
}

impl NonAdaptiveOptions {
    /// Returns practical defaults for the single-precision QNG driver.
    pub const fn single_precision() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-5,
        }
    }
}

impl Default for NonAdaptiveOptions {
    fn default() -> Self {
        Self {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-10,
        }
    }
}

/// Relative accuracy request used by the historical Newton-Cotes QNC79 pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nc79Options {
    /// Requested relative accuracy (`ERR`).
    pub relative_tolerance: f64,
}

impl Nc79Options {
    /// Returns practical defaults for the single-precision QNC79 driver.
    pub const fn single_precision() -> Self {
        Self {
            relative_tolerance: 1.0e-5,
        }
    }
}

impl Default for Nc79Options {
    fn default() -> Self {
        Self {
            relative_tolerance: 1.0e-10,
        }
    }
}

/// Summary returned by QAWF/DQAWF.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FourierIntegrationResult<T = f64> {
    /// Approximation to the Fourier-tail integral.
    pub value: T,
    /// Native absolute-error estimate.
    pub estimated_error: T,
    /// Total integrand evaluations.
    pub evaluations: usize,
    /// Fourier cycles used.
    pub cycles: usize,
}

/// Summary returned by the historical QNC79/DQNC79 Newton-Cotes drivers.
///
/// QNC79 reports no independent absolute-error estimate, so this result does
/// not manufacture one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nc79IntegrationResult<T = f64> {
    /// Approximation to the integral.
    pub value: T,
    /// Number of integrand evaluations.
    pub evaluations: usize,
}

/// Infinite range accepted by QAGI/DQAGI without passing infinities as bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InfiniteInterval<T = f64> {
    /// Integrate from the finite bound to positive infinity.
    Above(T),
    /// Integrate from negative infinity to the finite bound.
    Below(T),
    /// Integrate over the whole real line.
    WholeLine,
}

#[cfg(feature = "quadrature-basic")]
impl InfiniteInterval<f64> {
    pub(crate) fn native_parameters(self) -> Result<(f64, i32), IntegrationError> {
        match self {
            Self::Above(bound) if bound.is_finite() => Ok((bound, 1)),
            Self::Below(bound) if bound.is_finite() => Ok((bound, -1)),
            Self::WholeLine => Ok((0.0, 2)),
            Self::Above(_) | Self::Below(_) => Err(IntegrationError::InvalidBounds),
        }
    }
}

#[cfg(feature = "quadrature-basic")]
impl InfiniteInterval<f32> {
    pub(crate) fn native_parameters(self) -> Result<(f32, i32), IntegrationError> {
        match self {
            Self::Above(bound) if bound.is_finite() => Ok((bound, 1)),
            Self::Below(bound) if bound.is_finite() => Ok((bound, -1)),
            Self::WholeLine => Ok((0.0, 2)),
            Self::Above(_) | Self::Below(_) => Err(IntegrationError::InvalidBounds),
        }
    }
}
