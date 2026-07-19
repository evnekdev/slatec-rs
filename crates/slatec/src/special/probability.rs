//! Probability-related scalar special functions.
//!
//! # Status: Partial
//!
//! Only the reviewed regularized-beta functions currently belong here. A
//! broader distribution API is not provided.

#[cfg(feature = "special-beta")]
pub use crate::special::gamma::{regularized_beta, regularized_beta_f32};
