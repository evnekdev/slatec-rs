//! Interpolation and approximation organization.
//!
//! # Status: Partial
//!
//! PCHIP is available under `pchip`; owned B-spline and PP representations are
//! available under `bspline` and `piecewise_polynomial`. Other interpolation
//! families remain reserved until their storage and numerical contracts are
//! audited.

/// Approximation methods.
pub mod approximation;
/// B-spline interpolation.
pub mod bspline;
/// Chebyshev interpolation.
pub mod chebyshev;
/// Divided-difference interpolation.
pub mod divided_differences;
/// Piecewise-cubic Hermite interpolation.
pub mod pchip;
/// Piecewise-polynomial interpolation.
pub mod piecewise_polynomial;
