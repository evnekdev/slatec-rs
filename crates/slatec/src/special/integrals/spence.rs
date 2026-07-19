//! Spence's real dilogarithmic integral.
//!
//! # Status: Implemented
//!
//! Enable `special-scalar-expanded` for the reviewed f32 and f64 functions.

#[cfg(feature = "special-scalar-expanded")]
pub use crate::special::scalar_expanded::{spence_integral, spence_integral_f32};
