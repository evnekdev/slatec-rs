//! Feature-gated QUADPACK quadrature views.
//!
//! # Status: Implemented
//!
//! The compatibility path is `crate::quadrature`.

/// Basic adaptive finite and infinite integration.
pub mod basic;
/// Integration with explicit breakpoints.
pub mod breakpoints;
/// Infinite Fourier-tail integration.
pub mod fourier;
/// Non-adaptive integration rules.
pub mod nonadaptive;
/// Finite oscillatory integration.
pub mod oscillatory;
/// Endpoint-weighted integration.
pub mod weighted;
