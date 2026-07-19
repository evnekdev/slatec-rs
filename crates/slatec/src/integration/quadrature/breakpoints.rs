//! Breakpoint-aware adaptive integration.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-breakpoints`.

#[cfg(feature = "quadrature-breakpoints")]
pub use crate::quadrature::{integrate_with_breakpoints, integrate_with_breakpoints_f32};
