//! Safe callback-based adapters for a focused SLATEC QUADPACK family.
//!
//! The original QAG, QAGS, QAGI, and QAWC Fortran routines remain the
//! numerical implementations. Calls are serialized for the validated GNU
//! MinGW runtime profile because callback registration and legacy error state
//! are process-global. Integrand panics and non-finite results are contained
//! in the trampoline and reported after Fortran returns.

mod adaptive;
mod callback;
mod error;
mod validation;

pub use adaptive::{
    integrate, integrate_f32, integrate_infinite, integrate_infinite_f32,
    integrate_principal_value, integrate_principal_value_f32, integrate_singular,
    integrate_singular_f32,
};
pub use error::IntegrationError;

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
