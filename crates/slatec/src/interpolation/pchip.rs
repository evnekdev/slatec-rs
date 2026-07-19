//! Piecewise-cubic Hermite interpolation with reviewed PCHIP semantics.
//!
//! # Status: Implemented
//!
//! Enable `pchip`. The compatibility path is `crate::pchip`.

#[cfg(feature = "pchip")]
pub use crate::pchip::*;
