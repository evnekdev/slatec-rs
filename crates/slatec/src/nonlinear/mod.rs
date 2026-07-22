//! Safe nonlinear-system drivers and Jacobian checks over original SLATEC code.
//!
//! The `nonlinear-easy` feature wraps `SNSQE` and `DNSQE`. The
//! `nonlinear-expert` feature wraps `SNSQ` and `DNSQ` with checked
//! finite-difference and dense user-Jacobian modes. The alloc-only
//! `nonlinear-jacobian-check` feature wraps the two-stage `CHKDER` and `DCKDER`
//! consistency checks. Observer callbacks and callback cancellation remain
//! deliberately unavailable.
//!
//! The `nonlinear-systems` feature additionally exposes scalar-equation
//! `SOS`/`DSOS` drivers with typed termination reports. Solver APIs require
//! `std` and the validated GNU Fortran x86_64 MinGW native profile. Calls are
//! serialized because the selected SLATEC runtime contains process-global
//! state, and a callback cannot begin another callback-based SLATEC operation.
//! Jacobian checking uses `alloc` but does not require the hosted callback
//! runtime.

#[cfg(feature = "nonlinear-jacobian-check")]
mod checker;
mod error;
#[cfg(feature = "nonlinear-expert")]
mod expert;
#[cfg(any(
    feature = "nonlinear-expert",
    feature = "nonlinear-jacobian-check",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance"
))]
mod jacobian;
#[cfg(feature = "nonlinear-easy")]
mod solver;
#[cfg(feature = "nonlinear-systems")]
mod systems;

#[cfg(feature = "nonlinear-jacobian-check")]
pub use checker::{JacobianCheckError, JacobianCheckResult, check_jacobian, check_jacobian_f32};
pub use error::NonlinearError;
#[cfg(feature = "nonlinear-expert")]
pub use expert::{
    ExpertNonlinearOptions, ExpertNonlinearResult, JacobianStructure, VariableScaling,
    solve_system_expert, solve_system_expert_f32, solve_system_with_jacobian,
    solve_system_with_jacobian_f32,
};
#[cfg(any(
    feature = "nonlinear-expert",
    feature = "nonlinear-jacobian-check",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance"
))]
pub use jacobian::{JacobianIndexError, JacobianMut};
#[cfg(feature = "nonlinear-easy")]
pub use solver::{solve_system, solve_system_f32};
#[cfg(feature = "nonlinear-systems")]
pub use systems::{
    SystemOptions, SystemReport, SystemTermination, solve_scalar_equations,
    solve_scalar_equations_f32,
};

/// Easy-driver controls accepted by `SNSQE` and `DNSQE`.
///
/// The easy drivers expose only a relative solution tolerance. Their native
/// maximum callback budget is fixed at `200 * (n + 1)` in finite-difference
/// mode; this API deliberately does not invent a separate limit control.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonlinearOptions<T = f64> {
    /// Relative solution tolerance passed as the Fortran `TOL` argument.
    pub tolerance: T,
}

impl Default for NonlinearOptions<f64> {
    fn default() -> Self {
        Self { tolerance: 1.0e-10 }
    }
}

impl NonlinearOptions<f32> {
    /// Returns practical single-precision defaults for `SNSQE`.
    pub const fn single_precision() -> Self {
        Self { tolerance: 1.0e-5 }
    }
}

/// Meaningful successful or warning completion states from `SNSQE` and `DNSQE`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonlinearStatus {
    /// The relative error between successive iterates is at most `TOL`.
    Converged,
    /// The native callback budget was exhausted before convergence.
    MaximumFunctionEvaluations,
    /// The requested tolerance is smaller than the working precision permits.
    ToleranceTooSmall,
    /// Native progress stalled over the driver's recent Jacobian evaluations.
    SlowProgress,
    /// Progress stalled across five consecutive Jacobian evaluations.
    SlowProgressJacobianEvaluations,
    /// Progress stalled across ten consecutive solver iterations.
    SlowProgressIterations,
}

/// Result of a completed nonlinear easy-driver call.
///
/// `solution` is the final Fortran `X` vector; `residual` is the final native
/// `FVEC` vector. `residual_norm` is a Rust-recomputed Euclidean norm of that
/// returned residual. `function_evaluations` counts every Rust residual
/// callback invoked by the native driver.
#[derive(Clone, Debug, PartialEq)]
pub struct NonlinearResult<T = f64> {
    /// Final iterate returned through Fortran argument `X`.
    pub solution: alloc::vec::Vec<T>,
    /// Final residual vector returned through Fortran argument `FVEC`.
    pub residual: alloc::vec::Vec<T>,
    /// Euclidean norm of `residual`, recomputed by the safe wrapper.
    pub residual_norm: T,
    /// Number of contained Rust callback invocations.
    pub function_evaluations: usize,
    /// Interpreted native `INFO` completion state.
    pub status: NonlinearStatus,
}
