//! Numerical integration and integral-equation organization.
//!
//! # Status: Partial
//!
//! QUADPACK quadrature is available through `quadrature`. Integral equations
//! remain a planned separate family.

/// Integral-equation methods.
pub mod integral_equations;
/// QUADPACK quadrature families.
pub mod quadrature;
