#![no_std]
#![deny(missing_docs)]

//! Safe, opt-in Rust facades over original SLATEC Fortran implementations.
//!
//! Numerical families target the validated GNU MinGW x86_64 ABI and require an
//! explicit backend. `source-build` consumes a separately acquired verified
//! cache; `system` and `external-backend` provide integration escape hatches.
//! `prebuilt` is unavailable while historical redistribution rights remain
//! unresolved; see
//! [`docs/api/family-features-and-backends.md`](https://github.com/evnekdev/slatec-rs/blob/master/docs/api/family-features-and-backends.md).

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "nonlinear-easy", not(feature = "std")))]
compile_error!("the `nonlinear-easy` safe API requires the `std` feature");
#[cfg(all(feature = "nonlinear-expert", not(feature = "std")))]
compile_error!("the `nonlinear-expert` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-nonlinear-easy", not(feature = "std")))]
compile_error!("the `least-squares-nonlinear-easy` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-nonlinear-expert", not(feature = "std")))]
compile_error!("the `least-squares-nonlinear-expert` safe API requires the `std` feature");
#[cfg(all(feature = "ode-sdrive-expert", not(feature = "std")))]
compile_error!("the `ode-sdrive-expert` safe API requires the `std` feature");
#[cfg(all(
    feature = "optimization-linear-programming-in-memory",
    not(feature = "std")
))]
compile_error!(
    "the `optimization-linear-programming-in-memory` safe API requires the `std` feature"
);
#[cfg(all(feature = "least-squares-covariance", not(feature = "std")))]
compile_error!("the `least-squares-covariance` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-linear-nonnegative", not(feature = "std")))]
compile_error!("the `least-squares-linear-nonnegative` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-linear-bounded", not(feature = "std")))]
compile_error!("the `least-squares-linear-bounded` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-linear-constrained", not(feature = "std")))]
compile_error!("the `least-squares-linear-constrained` safe API requires the `std` feature");
#[cfg(all(
    feature = "least-squares-linear-bounded-constrained",
    not(feature = "std")
))]
compile_error!(
    "the `least-squares-linear-bounded-constrained` safe API requires the `std` feature"
);

// Keep the selected provider crate, and therefore its native link directives,
// in final artifacts without exposing provider mechanics in the safe API.
#[used]
static SLATEC_IMPLEMENTATION_PROVIDER: fn() = slatec_src::ensure_linked;

#[cfg(any(
    feature = "blas-level1",
    feature = "blas-level2",
    feature = "blas-level3"
))]
pub mod blas;

/// Runtime-validated scalar facades over selected original SLATEC FNLIB
/// routines for the GNU MinGW x86_64 profile.
#[cfg(any(
    feature = "special-elementary",
    feature = "special-gamma",
    feature = "special-beta",
    feature = "special-error",
    feature = "special-airy",
    feature = "special-bessel",
    feature = "special-integrals"
))]
pub mod special;

/// Scalar polynomial helpers whose storage contracts are independently
/// validated before the raw call.
#[cfg(feature = "special-polynomials")]
pub mod polynomials;

#[cfg(any(
    feature = "special-elementary",
    feature = "special-gamma",
    feature = "special-beta",
    feature = "special-error",
    feature = "special-airy",
    feature = "special-bessel",
    feature = "special-integrals",
    feature = "quadrature-basic",
    feature = "quadrature-breakpoints",
    feature = "quadrature-weighted",
    feature = "quadrature-oscillatory",
    feature = "quadrature-fourier",
    feature = "quadrature-nonadaptive",
    feature = "roots-scalar",
    feature = "nonlinear-easy",
    feature = "nonlinear-expert",
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance",
    feature = "least-squares-linear-nonnegative",
    feature = "least-squares-linear-bounded",
    feature = "least-squares-linear-constrained",
    feature = "least-squares-linear-bounded-constrained",
    feature = "ode-sdrive-expert",
    feature = "optimization-linear-programming-in-memory"
))]
pub(crate) mod runtime;

/// Test-only observations of the hosted process-wide native runtime lock.
///
/// This module is available only with `native-serialization-tests`; it does
/// not alter lock scope or advertise native parallel execution.
#[cfg(feature = "native-serialization-tests")]
#[doc(hidden)]
pub mod native_serialization_test_support {
    /// A point-in-time observation of hosted native lock activity.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Snapshot {
        /// Hosted outermost native scopes currently active.
        pub active: usize,
        /// Maximum outermost scopes observed since reset.
        pub maximum_active: usize,
        /// Same-thread nested lock entries observed since reset.
        pub nested_same_thread: usize,
        /// Whether the process-wide lock currently has an owner.
        pub owner_present: bool,
        /// Debug-form Rust thread identifier of the current owner, if any.
        pub owner_thread: Option<std::string::String>,
    }

    /// Resets test counters while holding the same process-wide runtime lock.
    pub fn reset() {
        crate::runtime::reset_hosted_native_call_audit();
    }

    /// Returns the current test-only serialization observation.
    #[must_use]
    pub fn snapshot() -> Snapshot {
        let (active, maximum_active, nested_same_thread, owner_present, owner_thread) =
            crate::runtime::hosted_native_call_audit();
        Snapshot {
            active,
            maximum_active,
            nested_same_thread,
            owner_present,
            owner_thread,
        }
    }
}

/// Test-only observations surrounding exact qualified BLAS Level 1 FFI calls.
///
/// This module is available only to the reviewed source-backend concurrency
/// test profile. It does not alter production dispatch or provider claims.
#[cfg(feature = "blas-level1-concurrency-native-tests")]
#[doc(hidden)]
pub mod blas1_concurrency_test_support {
    /// A point-in-time observation of candidate native-call overlap.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Snapshot {
        /// Candidate BLAS native calls currently active.
        pub active: usize,
        /// Maximum simultaneous candidate BLAS native calls since reset.
        pub maximum_active: usize,
        /// Candidate entries observed while a hosted exclusive scope was active.
        pub hosted_overlaps: usize,
    }

    /// Resets candidate native-call counters while no hosted solver can enter.
    pub fn reset() {
        crate::runtime::reset_blas1_native_call_audit();
    }

    /// Returns the current test-only candidate overlap observation.
    #[must_use]
    pub fn snapshot() -> Snapshot {
        let (active, maximum_active, hosted_overlaps) = crate::runtime::blas1_native_call_audit();
        Snapshot {
            active,
            maximum_active,
            hosted_overlaps,
        }
    }
}

#[cfg(any(
    feature = "quadrature-basic",
    feature = "quadrature-breakpoints",
    feature = "quadrature-weighted",
    feature = "quadrature-oscillatory",
    feature = "quadrature-fourier",
    feature = "quadrature-nonadaptive",
    feature = "roots-scalar",
    feature = "nonlinear-easy",
    feature = "nonlinear-expert",
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance"
))]
mod callback_runtime;

/// Panic-contained closure adapters for the reviewed SLATEC QUADPACK drivers.
#[cfg(any(
    feature = "quadrature-basic",
    feature = "quadrature-breakpoints",
    feature = "quadrature-weighted",
    feature = "quadrature-oscillatory",
    feature = "quadrature-fourier",
    feature = "quadrature-nonadaptive"
))]
pub mod quadrature;

/// Safe bracketed scalar-root adapters over the original FZERO routines.
#[cfg(feature = "roots-scalar")]
pub mod roots;

/// Safe nonlinear-system solvers and Jacobian checks over original SLATEC
/// `SNSQE`, `DNSQE`, `SNSQ`, `DNSQ`, `CHKDER`, and `DCKDER` implementations.
#[cfg(any(
    feature = "nonlinear-easy",
    feature = "nonlinear-expert",
    feature = "nonlinear-jacobian-check",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance"
))]
pub mod nonlinear;

/// Safe nonlinear least-squares drivers over the original SLATEC
/// `SNLS1E`, `DNLS1E`, `SNLS1`, and `DNLS1` implementations.
///
/// This hosted, allocating module minimizes one half of the residual sum of
/// squares. The easy feature accepts residual-only closures and asks the
/// original driver to form finite-difference Jacobians; the separate expert
/// feature also exposes checked controls, scaling, and dense analytic
/// Jacobian closures. It is not a nonlinear equation-solver API.
#[cfg(any(
    feature = "least-squares-nonlinear-easy",
    feature = "least-squares-nonlinear-expert",
    feature = "least-squares-covariance"
))]
pub mod least_squares;

/// Safe constrained linear least-squares facades over original SLATEC
/// `WNNLS` and `DWNNLS` implementations.
#[cfg(any(
    feature = "least-squares-linear-nonnegative",
    feature = "least-squares-linear-bounded",
    feature = "least-squares-linear-constrained",
    feature = "least-squares-linear-bounded-constrained"
))]
pub mod linear_least_squares;

/// Safe dense bounded linear least-squares facades over original SLATEC
/// `SBOLS` and `DBOLS` implementations.
#[cfg(feature = "least-squares-linear-bounded")]
pub mod bounded_least_squares;

/// Safe dense equality/inequality constrained linear least-squares facades
/// over original SLATEC `LSEI` and `DLSEI` implementations.
#[cfg(feature = "least-squares-linear-constrained")]
pub mod constrained_least_squares;

/// Safe dense bounded constrained linear least-squares facades over original
/// SLATEC `SBOCLS` and `DBOCLS` implementations.
#[cfg(feature = "least-squares-linear-bounded-constrained")]
pub mod bounded_constrained_least_squares;

/// Safe owned sessions for restricted explicit ODE initial-value problems over
/// the original SLATEC `SDRIV3` and `DDRIV3` drivers.
#[cfg(feature = "ode-sdrive-expert")]
pub mod ode;

/// Safe sparse linear programming over original `SPLP` and `DSPLP`, limited
/// to problems proved to remain entirely in native high-speed memory.
///
/// The linear objective is distinct from every least-squares family. Paging,
/// Fortran-unit management, save/restore, native printing, and filesystem use
/// are not exposed.
#[cfg(feature = "optimization-linear-programming-in-memory")]
pub mod linear_programming;
