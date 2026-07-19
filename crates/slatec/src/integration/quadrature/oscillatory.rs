//! Finite oscillatory quadrature.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-oscillatory`.

#[cfg(feature = "quadrature-oscillatory")]
pub use crate::quadrature::{
    OscillatoryOptions, OscillatoryWeight, integrate_oscillatory, integrate_oscillatory_f32,
};
