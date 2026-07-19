//! Error functions and complementary error functions.
//!
//! # Status: Partial
//!
//! Existing [`crate::special::error_functions`] remains the compatibility
//! module. Enable `special-error` for callable functions.

#[cfg(feature = "special-error")]
pub use crate::special::error_functions::*;
