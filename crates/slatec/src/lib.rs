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
#[cfg(all(feature = "least-squares-covariance", not(feature = "std")))]
compile_error!("the `least-squares-covariance` safe API requires the `std` feature");
#[cfg(all(feature = "least-squares-linear-nonnegative", not(feature = "std")))]
compile_error!("the `least-squares-linear-nonnegative` safe API requires the `std` feature");

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
    feature = "least-squares-linear-nonnegative"
))]
pub(crate) mod runtime;

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
#[cfg(feature = "least-squares-linear-nonnegative")]
pub mod linear_least_squares;
