//! Safe univariate B-spline evaluation and integration.
//!
//! # Status: Partial
//!
//! This module exposes `BSpline`, an owned scalar B-spline represented by
//! its knot sequence, coefficients, and order. It evaluates values and
//! derivatives with SLATEC `BVALU`/`DBVALU` and integrates with
//! `BSQAD`/`DBSQAD`. Construction from interpolation data and basis-function
//! APIs remain deliberately deferred.

#[cfg(feature = "bspline")]
pub use crate::bspline::*;
