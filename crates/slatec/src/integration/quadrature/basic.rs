//! Basic adaptive QUADPACK integration.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-basic`.

#[cfg(feature = "quadrature-basic")]
pub use crate::quadrature::{
    InfiniteInterval, IntegrationOptions, IntegrationResult, IntegrationRule, integrate,
    integrate_f32, integrate_infinite, integrate_infinite_f32, integrate_principal_value,
    integrate_principal_value_f32, integrate_singular, integrate_singular_f32,
};
