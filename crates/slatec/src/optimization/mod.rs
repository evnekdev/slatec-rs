//! Optimization organization.
//!
//! # Status: Partial
//!
//! In-memory sparse linear programming is implemented. Other optimization
//! families await independent safety and native-state audits. The LP API never
//! enables paging, filesystem, Fortran-unit, or save/restore operation.

/// Constrained optimization.
pub mod constrained;
/// In-memory sparse linear programming.
pub mod linear_programming;
/// Nonlinear programming.
pub mod nonlinear;
/// Unconstrained minimization.
pub mod unconstrained;
