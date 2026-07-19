//! Non-adaptive QUADPACK and Newton--Cotes rules.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-nonadaptive`.

#[cfg(feature = "quadrature-nonadaptive")]
pub use crate::quadrature::{
    Nc79IntegrationResult, Nc79Options, NonAdaptiveOptions, integrate_nc79, integrate_nc79_f32,
    integrate_non_adaptive, integrate_non_adaptive_f32,
};
