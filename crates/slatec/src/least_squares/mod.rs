//! Safe nonlinear least-squares drivers.
//!
//! This module wraps the original SLATEC Levenberg--Marquardt easy drivers
//! `SNLS1E` and `DNLS1E` in their finite-difference mode, plus the opt-in
//! expert `SNLS1` and `DNLS1` finite-difference and dense-Jacobian modes, plus
//! the opt-in `SCOV`/`DCOV` covariance helpers. It solves
//! `minimize 1/2 * sum_i r_i(x)^2` for `M` residuals and `N` parameters, with
//! the native precondition `M >= N`. Calls require `std`, allocate the exact
//! documented work arrays internally, and serialize access to the validated
//! GNU Fortran `x86_64-w64-mingw32` runtime. A callback cannot invoke another
//! callback-bearing SLATEC facade.

#[cfg(feature = "least-squares-covariance")]
pub mod covariance;
mod error;
#[cfg(feature = "least-squares-nonlinear-expert")]
mod expert;
#[cfg(feature = "least-squares-nonlinear-easy")]
mod solver;

/// Linear least-squares organization.
pub mod linear;
/// Nonlinear least-squares organization.
pub mod nonlinear;

#[cfg(feature = "least-squares-covariance")]
pub use covariance::{
    CovarianceEligibility, CovarianceError, CovarianceOptions, CovarianceResult, CovarianceScaling,
    CovarianceStatus, estimate_covariance, estimate_covariance_f32,
    estimate_covariance_finite_difference, estimate_covariance_finite_difference_f32,
};
#[cfg(all(
    feature = "least-squares-covariance",
    feature = "least-squares-nonlinear-expert"
))]
pub use covariance::{covariance_from_expert_fit, covariance_from_expert_fit_f32};
pub use error::LeastSquaresError;
#[cfg(feature = "least-squares-nonlinear-expert")]
pub use expert::{
    ExpertLeastSquaresOptions, ExpertLeastSquaresResult, LeastSquaresScaling, least_squares_expert,
    least_squares_expert_f32, least_squares_with_jacobian, least_squares_with_jacobian_f32,
};
#[cfg(feature = "least-squares-nonlinear-easy")]
pub use solver::{LeastSquaresOptions, LeastSquaresResult, least_squares, least_squares_f32};

/// Meaningful `INFO` completion states from reviewed SLATEC nonlinear
/// least-squares drivers.
///
/// All variants retain final parameters and residuals in the corresponding
/// result. The tolerance-limit states describe a native numerical completion,
/// not a Rust callback or ABI failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeastSquaresStatus {
    /// Both actual and predicted reductions in the residual sum of squares are
    /// small relative to `FTOL` (or easy-driver `TOL`).
    ConvergedResidual,
    /// The relative change in the parameter vector is small relative to
    /// `XTOL` (or easy-driver `TOL`).
    ConvergedParameters,
    /// Both residual-reduction and parameter-change convergence tests passed.
    ConvergedResidualAndParameters,
    /// The residual is orthogonal to the Jacobian columns at `GTOL`.
    ConvergedOrthogonality,
    /// The configured native residual-callback budget was exhausted.
    MaximumEvaluations,
    /// Further reduction in the residual sum of squares is limited by working
    /// precision at the requested `FTOL`.
    ResidualToleranceTooSmall,
    /// Further parameter improvement is limited by working precision at the
    /// requested `XTOL`.
    ParameterToleranceTooSmall,
    /// Further gradient-orthogonality improvement is limited by working
    /// precision at the requested `GTOL`; only expert `SNLS1`/`DNLS1` return
    /// this `INFO = 8` status.
    GradientToleranceTooSmall,
}
