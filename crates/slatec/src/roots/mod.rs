//! Safe root finding over reviewed original SLATEC routines.
//!
//! Scalar roots use the callback-bearing `FZERO` routines. Polynomial roots
//! use owned buffers with the single-precision complex `CPZERO`/`RPZERO` and
//! `CPQR79`/`RPQR79` drivers. Every native call is serialized under the
//! validated GNU MinGW x86_64 profile.

#[cfg(feature = "roots-scalar")]
mod error;
#[cfg(feature = "roots-polynomial")]
pub mod polynomial;
#[cfg(feature = "roots-scalar")]
mod scalar;

#[cfg(feature = "roots-scalar")]
pub use error::RootError;
#[cfg(feature = "roots-polynomial")]
pub use polynomial::{
    PolynomialRootError, PolynomialRootMethod, PolynomialRootStatus, PolynomialRoots,
    complex_polynomial_roots, complex_polynomial_roots_with_method, real_polynomial_roots,
    real_polynomial_roots_with_method,
};
#[cfg(feature = "roots-scalar")]
pub use scalar::{find_root, find_root_f32};

/// A caller-supplied bracket. Reversed endpoints are accepted.
#[cfg(feature = "roots-scalar")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootBracket<T = f64> {
    /// First endpoint passed as Fortran argument `B`.
    pub lower: T,
    /// Second endpoint passed as Fortran argument `C`.
    pub upper: T,
}

/// Accuracy controls and an optional interior initial suggestion for FZERO.
///
/// If `initial_guess` is absent, the safe wrapper passes the upper endpoint,
/// following the original routine's documented recommendation when no better
/// interior suggestion is available.
#[cfg(feature = "roots-scalar")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootOptions<T = f64> {
    /// Relative bracket-width tolerance (`RE`).
    pub relative_tolerance: T,
    /// Absolute bracket-width tolerance (`AE`).
    pub absolute_tolerance: T,
    /// Optional native suggestion (`R`); `None` uses the upper endpoint.
    pub initial_guess: Option<T>,
}

#[cfg(feature = "roots-scalar")]
impl Default for RootOptions<f64> {
    fn default() -> Self {
        Self {
            relative_tolerance: 1.0e-10,
            absolute_tolerance: 1.0e-12,
            initial_guess: None,
        }
    }
}

#[cfg(feature = "roots-scalar")]
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
#[cfg(feature = "roots-scalar")]
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
#[cfg(feature = "roots-scalar")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootResult<T = f64> {
    /// Best root estimate returned by FZERO.
    pub estimate: T,
    /// Final pair of native bracket endpoints.
    pub bracket: RootBracket<T>,
    /// Counted callback invocations, including safe endpoint validation.
    pub evaluations: usize,
    /// Interpreted native `IFLAG` completion status.
    pub status: RootStatus,
}
