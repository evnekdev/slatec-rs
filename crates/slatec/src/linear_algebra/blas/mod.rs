//! BLAS vector and dense matrix operations.
//!
//! # Status: Implemented
//!
//! The scoped modules expose the BLAS family through the canonical
//! linear-algebra namespace.

/// BLAS Level 1 vector operations.
pub mod level1;
/// BLAS Level 2 matrix-vector operations.
pub mod level2;
/// BLAS Level 3 matrix-matrix operations.
pub mod level3;

#[cfg(any(
    feature = "blas-level1",
    feature = "blas-level2",
    feature = "blas-level3"
))]
pub use crate::blas::{BlasError, Diagonal, Side, Transpose, Triangle};
