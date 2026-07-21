//! Interpolation and approximation organization.
//!
//! # Status: Partial
//!
//! PCHIP is available under `pchip`; owned B-spline, PP, and tabulated global
//! polynomial representations are available under `bspline`,
//! `piecewise_polynomial`, and `tabulated`. Other interpolation families
//! remain reserved until their storage and numerical contracts are audited.

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
/// Checked tabulated samples and global polynomial interpolation.
#[cfg(feature = "tabulated-data")]
pub mod tabulated;
