//! Ordinary and differential-algebraic equations.
//!
//! # Status: Partial
//!
//! SDRIVE and residual-only DASSL sessions are available under their feature
//! gates. Existing `crate::ode` and `crate::dassl` paths remain supported.
//! Hosted native entry remains process-wide serialized; sessions own their
//! continuation workspace and callback state.

/// Boundary-value problems.
pub mod boundary_value;
/// Differential-algebraic equations.
pub mod dae;
/// Ordinary differential equations.
pub mod ode;
/// Partial differential equations.
pub mod pde;
