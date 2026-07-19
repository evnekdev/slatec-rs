//! Nonlinear, covariance, and linear least-squares APIs.
//!
//! # Status: Partial
//!
//! This permanent namespace has no callable API until a least-squares feature
//! is selected. Exact solver signatures remain at their compatibility paths.

#[path = "least_squares/covariance.rs"]
/// Covariance estimation for nonlinear least-squares fits.
pub mod covariance;
#[path = "least_squares/linear.rs"]
/// Linear least-squares fitting.
pub mod linear;
#[path = "least_squares/nonlinear.rs"]
/// Nonlinear least-squares fitting.
pub mod nonlinear;
