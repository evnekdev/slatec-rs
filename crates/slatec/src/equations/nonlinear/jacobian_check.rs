//! Jacobian consistency checking.
//!
//! # Status: Implemented
//!
//! Enable `nonlinear-jacobian-check`.

#[cfg(feature = "nonlinear-jacobian-check")]
pub use crate::nonlinear::{
    JacobianCheckError, JacobianCheckResult, check_jacobian, check_jacobian_f32,
};
