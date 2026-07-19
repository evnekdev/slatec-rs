//! Expert nonlinear least-squares fitting.
//!
//! # Status: Implemented
//!
//! Enable `least-squares-nonlinear-expert`.

#[cfg(feature = "least-squares-nonlinear-expert")]
pub use crate::least_squares::{
    ExpertLeastSquaresOptions, ExpertLeastSquaresResult, LeastSquaresError, LeastSquaresScaling,
    LeastSquaresStatus, least_squares_expert, least_squares_expert_f32,
    least_squares_with_jacobian, least_squares_with_jacobian_f32,
};
