//! BLAS vector and dense matrix operations.
//!
//! # Status: Implemented
//!
//! The scoped modules preserve the feature gates of `crate::blas`.

/// BLAS Level 1 vector operations.
pub mod level1;
/// BLAS Level 2 matrix-vector operations.
pub mod level2;
/// BLAS Level 3 matrix-matrix operations.
pub mod level3;
