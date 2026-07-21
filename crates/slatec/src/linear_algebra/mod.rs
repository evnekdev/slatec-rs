//! Linear algebra and storage-oriented numerical APIs.
//!
//! # Status: Partial
//!
//! Reviewed BLAS Levels 1--3 and general banded systems are available through
//! their respective modules. Dense, packed, eigenvalue, and sparse families
//! remain reserved for future audits.

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
