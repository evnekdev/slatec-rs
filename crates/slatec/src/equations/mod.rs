//! Root finding and nonlinear-equation solving.
//!
//! # Status: Partial
//!
//! Scalar roots and reviewed nonlinear systems are available behind their
//! existing features. `crate::roots` and `crate::nonlinear` remain
//! supported compatibility paths.

/// Nonlinear systems of equations.
pub mod nonlinear;
/// Scalar and polynomial root finding.
pub mod roots;
