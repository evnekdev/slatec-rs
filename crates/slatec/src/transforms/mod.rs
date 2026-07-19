//! Numerical transform organization.
//!
//! # Status: Partial
//!
//! Real and selected complex FFTPACK plans are available in `fft` behind their
//! separate feature flags. Multidimensional APIs remain a separate source and
//! ABI milestone.

/// Fourier transforms.
pub mod fft;
