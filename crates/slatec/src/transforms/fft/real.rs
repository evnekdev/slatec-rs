//! Real FFTPACK plan APIs.
//!
//! # Status: Implemented
//!
//! Enable `fftpack-real`. The compatibility path is `crate::fftpack`.

#[cfg(feature = "fftpack-real")]
pub use crate::fftpack::*;
