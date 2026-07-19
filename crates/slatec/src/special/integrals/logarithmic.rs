//! The real logarithmic integral.
//!
//! # Status: Implemented
//!
//! Enable `special-scalar-expanded` for the reviewed f32 and f64 functions.

#[cfg(feature = "special-scalar-expanded")]
pub use crate::special::scalar_expanded::{logarithmic_integral, logarithmic_integral_f32};
