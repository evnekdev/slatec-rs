//! Easy residual-only nonlinear least-squares fitting.
//!
//! # Status: Implemented
//!
//! Enable `least-squares-nonlinear-easy`.

#[cfg(feature = "least-squares-nonlinear-easy")]
pub use crate::least_squares::{
    LeastSquaresError, LeastSquaresOptions, LeastSquaresResult, LeastSquaresStatus, least_squares,
    least_squares_f32,
};
