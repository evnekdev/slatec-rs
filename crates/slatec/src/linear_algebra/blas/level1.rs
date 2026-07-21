//! BLAS Level 1 operations on contiguous and checked strided vectors.
//!
//! # Status: Implemented
//!
//! Enable `blas-level1`.

#[cfg(feature = "blas-level1")]
pub use crate::blas::level1::*;
