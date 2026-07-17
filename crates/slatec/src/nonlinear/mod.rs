//! Safe finite-difference nonlinear-system drivers over original SLATEC code.
//!
//! This module wraps `SNSQE` and `DNSQE`, the simple drivers for the SLATEC
//! Powell-hybrid nonlinear-system solver. It fixes their `IOPT` argument to
//! finite-difference Jacobian mode, allocates the documented workspace, and
//! contains a Rust residual callback behind the shared native-runtime guard.
//!
//! The API requires `std`, `alloc`, the `nonlinear-easy` feature, and the
//! validated GNU Fortran x86_64 MinGW native profile. Calls are serialized
//! because the selected SLATEC runtime contains process-global state. A
//! callback cannot begin another callback-based SLATEC operation.

mod error;
mod solver;

pub use error::NonlinearError;
pub use solver::{solve_system, solve_system_f32};

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
