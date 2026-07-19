//! Linear least-squares fitting.
//!
//! # Status: Implemented with its feature gates.

#[path = "linear/bounded.rs"]
/// Bounded linear least squares.
pub mod bounded;
#[path = "linear/bounded_constrained.rs"]
/// Bounded constrained linear least squares.
pub mod bounded_constrained;
#[path = "linear/constrained.rs"]
/// Constrained linear least squares.
pub mod constrained;
#[path = "linear/nonnegative.rs"]
/// Nonnegative linear least squares.
pub mod nonnegative;
