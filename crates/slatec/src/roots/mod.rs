//! Safe bracketed scalar-root finding over original SLATEC `FZERO` routines.
//!
//! Calls are available only for the validated GNU MinGW x86_64 profile. The
//! callback runtime is shared with quadrature: callback-bearing native calls
//! serialize and any nested callback-based SLATEC call is rejected.

mod error;
mod scalar;

pub use error::RootError;
pub use scalar::{find_root, find_root_f32};

/// A caller-supplied bracket. Reversed endpoints are accepted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootBracket<T = f64> {
    pub lower: T,
    pub upper: T,
}

/// Accuracy controls and an optional interior initial suggestion for FZERO.
///
/// If `initial_guess` is absent, the safe wrapper passes the upper endpoint,
/// following the original routine's documented recommendation when no better
/// interior suggestion is available.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootOptions<T = f64> {
    pub relative_tolerance: T,
    pub absolute_tolerance: T,
    pub initial_guess: Option<T>,
}

impl Default for RootOptions<f64> {
    fn default() -> Self {
        Self {
            relative_tolerance: 1.0e-10,
            absolute_tolerance: 1.0e-12,
            initial_guess: None,
        }
    }
}

impl RootOptions<f32> {
    /// Practical defaults for the single-precision FZERO driver.
    pub const fn single_precision() -> Self {
        Self {
            relative_tolerance: 1.0e-5,
            absolute_tolerance: 1.0e-6,
            initial_guess: None,
        }
    }
}

/// Numerically meaningful FZERO completion statuses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootStatus {
    /// The bracket contracted to the requested tolerance with decreasing
    /// function magnitude.
    Converged,
    /// The caller-provided lower endpoint evaluated to exact zero.
    LowerEndpoint,
    /// The caller-provided upper endpoint evaluated to exact zero.
    UpperEndpoint,
    /// The native driver found an exact zero at its current endpoint.
    EndpointRoot,
    /// The sign-changing interval collapsed while function magnitude grew.
    PossibleSingularity,
    /// The interval collapsed without a sign change during native iteration.
    NoSignChange,
    /// The native driver's fixed evaluation limit was reached.
    MaximumEvaluations,
}

/// Result from a completed FZERO call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootResult<T = f64> {
    pub estimate: T,
    pub bracket: RootBracket<T>,
    /// Counted callback invocations, including safe endpoint validation.
    pub evaluations: usize,
    pub status: RootStatus,
}
