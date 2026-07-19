//! DASSL differential-algebraic equation sessions.
//!
//! # Status: Partial

/// Banded Jacobian sessions.
pub mod banded;
/// Residual-only dense internally differenced Jacobian sessions.
pub mod residual;
/// User-supplied Jacobian sessions.
pub mod user_jacobian;
