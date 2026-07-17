//! Safe callback-based adapters for a focused SLATEC QUADPACK family.
//!
//! The original QUADPACK Fortran drivers remain the numerical implementations.
//! Calls are serialized for the validated GNU MinGW runtime profile because
//! callback registration and legacy error state are process-global. Integrand
//! panics and non-finite results are contained in the trampoline and reported
//! after Fortran returns.

mod adaptive;
mod callback;
mod error;
mod extended;
mod validation;

pub use adaptive::{
    integrate, integrate_f32, integrate_infinite, integrate_infinite_f32,
    integrate_principal_value, integrate_principal_value_f32, integrate_singular,
    integrate_singular_f32,
};
pub use error::IntegrationError;
pub use extended::{
    integrate_fourier_tail, integrate_fourier_tail_f32, integrate_nc79, integrate_nc79_f32,
    integrate_non_adaptive, integrate_non_adaptive_f32, integrate_oscillatory,
    integrate_oscillatory_f32, integrate_weighted_endpoints, integrate_weighted_endpoints_f32,
    integrate_with_breakpoints, integrate_with_breakpoints_f32,
};

/// Gauss-Kronrod rule used by QAG/DQAG finite-interval integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrationRule {
    Points15,
    Points21,
    Points31,
    Points41,
    Points51,
    Points61,
}

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
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
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
    pub value: T,
    pub estimated_error: T,
    pub evaluations: usize,
    pub intervals: usize,
}

/// Meaning of the endpoint weight applied by QAWS/DQAWS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointWeight {
    Algebraic,
    AlgebraicLogLower,
    AlgebraicLogUpper,
    AlgebraicLogBoth,
}

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
    Cosine,
    Sine,
}

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
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
    pub subdivision_limit: usize,
    /// Upper bound on stored Chebyshev-moment levels. The safe API discards
    /// these moments after the call.
    pub maximum_moments: usize,
}

impl OscillatoryOptions {
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
    pub absolute_tolerance: f64,
    pub subdivision_limit: usize,
    pub cycle_limit: usize,
    pub maximum_moments: usize,
}

impl FourierOptions {
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
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

impl NonAdaptiveOptions {
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
    pub relative_tolerance: f64,
}

impl Nc79Options {
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
    pub value: T,
    pub estimated_error: T,
    pub evaluations: usize,
    pub cycles: usize,
}

/// Summary returned by the historical QNC79/DQNC79 Newton-Cotes drivers.
///
/// QNC79 reports no independent absolute-error estimate, so this result does
/// not manufacture one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Nc79IntegrationResult<T = f64> {
    pub value: T,
    pub evaluations: usize,
}

/// Infinite range accepted by QAGI/DQAGI without passing infinities as bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InfiniteInterval<T = f64> {
    Above(T),
    Below(T),
    WholeLine,
}

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
