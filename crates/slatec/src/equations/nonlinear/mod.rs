//! Nonlinear systems of equations.
//!
//! # Status: Implemented
//!
//! The compatibility path is `crate::nonlinear`.

/// Easy finite-difference nonlinear systems.
pub mod easy;
/// Expert finite-difference and analytic-Jacobian nonlinear systems.
pub mod expert;
/// Jacobian consistency checking.
pub mod jacobian_check;
