//! Safe, opt-in Rust façades over original SLATEC Fortran implementations.
//!
//! The `blas-level1` feature targets the validated
//! `ffi-profile-gnu-mingw-x86_64` raw ABI. It does not download or compile
//! Fortran. Native use requires the explicit link environment documented in
//! [`blas`].

#[cfg(feature = "blas-level1-validation")]
pub mod blas;

/// Runtime-validated scalar façades over selected original SLATEC FNLIB
/// routines for the GNU MinGW x86_64 profile.
#[cfg(feature = "special-functions")]
pub mod special;

/// Scalar polynomial helpers whose storage contracts are independently
/// validated before the raw call.
#[cfg(feature = "special-functions-polynomials")]
pub mod polynomials;

#[cfg(any(
    feature = "special-functions",
    feature = "quadrature",
    feature = "roots"
))]
pub(crate) mod runtime;

#[cfg(any(feature = "quadrature", feature = "roots"))]
mod callback_runtime;

/// Panic-contained closure adapters for the reviewed SLATEC QUADPACK drivers.
#[cfg(feature = "quadrature")]
pub mod quadrature;

/// Safe bracketed scalar-root adapters over the original FZERO routines.
#[cfg(feature = "roots")]
pub mod roots;
