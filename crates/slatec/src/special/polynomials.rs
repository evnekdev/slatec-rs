//! Scalar orthogonal-polynomial functions.
//!
//! # Status: Partial
//!
//! `crate::polynomials` remains the compatibility path. Enable
//! `special-polynomials` for callable functions.

#[cfg(feature = "special-polynomials")]
pub use crate::polynomials::*;
