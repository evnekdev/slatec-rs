//! BLAS Level 3 operations on checked column-major dense matrices.
//!
//! # Status: Implemented
//!
//! Enable `blas-level3`. The compatibility path is `crate::blas::level3`.

#[cfg(feature = "blas-level3")]
pub use crate::blas::level3::*;
