//! Beta and regularized-beta scalar functions.
//!
//! # Status: Partial
//!
//! The existing [`crate::special::gamma`] module remains supported. Enable
//! `special-beta` for the beta-family subset.

#[cfg(feature = "special-beta")]
pub use crate::special::gamma::{
    beta, beta_f32, log_beta, log_beta_f32, regularized_beta, regularized_beta_f32,
};
