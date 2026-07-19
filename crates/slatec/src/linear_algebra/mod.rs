//! Linear algebra and storage-oriented numerical APIs.
//!
//! # Status: Partial
//!
//! Reviewed BLAS Levels 1--3 are available through `blas`. Dense, banded,
//! packed, eigenvalue, and sparse families are reserved for future audits.
//! Existing `crate::blas` paths remain supported compatibility paths.

/// Banded linear algebra.
pub mod banded;
/// Basic Linear Algebra Subprograms.
pub mod blas;
/// Dense linear algebra.
pub mod dense;
/// Eigenvalue and eigenvector algorithms.
pub mod eigen;
/// Packed-storage linear algebra.
pub mod packed;
/// Sparse linear algebra.
pub mod sparse;
