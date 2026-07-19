//! Numerical transform organization.
//!
//! # Status: Partial
//!
//! Real FFTPACK plans are available in `fft`. Complex and multidimensional
//! APIs remain separate source and ABI milestones.

/// Fourier transforms.
pub mod fft;
