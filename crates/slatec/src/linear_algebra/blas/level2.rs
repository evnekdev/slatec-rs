//! BLAS Level 2 operations on checked column-major dense matrices.
//!
//! # Status: Implemented
//!
//! Enable `blas-level2`. The compatibility path is `crate::blas::level2`.

#[cfg(feature = "blas-level2")]
pub use crate::blas::level2::*;
