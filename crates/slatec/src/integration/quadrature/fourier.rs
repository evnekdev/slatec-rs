//! Infinite Fourier-tail quadrature.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-fourier`.

#[cfg(feature = "quadrature-fourier")]
pub use crate::quadrature::{
    FourierIntegrationResult, FourierOptions, integrate_fourier_tail, integrate_fourier_tail_f32,
};
