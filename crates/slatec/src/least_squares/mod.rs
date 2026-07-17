//! Safe residual-only nonlinear least-squares drivers.
//!
//! This module wraps the original SLATEC Levenberg--Marquardt easy drivers
//! `SNLS1E` and `DNLS1E` in their finite-difference mode. It solves
//! `minimize 1/2 * sum_i r_i(x)^2` for `M` residuals and `N` parameters, with
//! the native precondition `M >= N`. Calls require `std`, allocate the exact
//! documented work arrays internally, and serialize access to the validated
//! GNU Fortran `x86_64-w64-mingw32` runtime. A callback cannot invoke another
//! callback-bearing SLATEC facade.

mod error;
mod solver;

pub use error::LeastSquaresError;
pub use solver::{
    LeastSquaresOptions, LeastSquaresResult, LeastSquaresStatus, least_squares, least_squares_f32,
};
