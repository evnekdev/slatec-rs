//! Sparse linear algebra.
//!
//! # Status: Planned
//!
//! Sparse matrix ownership, indexing, and callback contracts require separate
//! audits before callable APIs are introduced.

/// Sparse direct methods.
pub mod direct;
/// Sparse iterative methods.
pub mod iterative;
/// Sparse operator representations.
pub mod operators;
