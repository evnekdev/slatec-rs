//! In-memory-only sparse linear programming.
//!
//! # Status: Implemented
//!
//! Enable `optimization-linear-programming-in-memory`. The compatibility path
//! is `crate::linear_programming`. Paging and Fortran-unit operation remain
//! deliberately unavailable.

#[cfg(feature = "optimization-linear-programming-in-memory")]
pub use crate::linear_programming::*;
