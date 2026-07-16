//! Safe, opt-in Rust façades over original SLATEC Fortran implementations.
//!
//! The `blas-level1` feature targets the validated
//! `ffi-profile-gnu-mingw-x86_64` raw ABI. It does not download or compile
//! Fortran. Native use requires the explicit link environment documented in
//! [`blas`].

#[cfg(feature = "blas-level1-validation")]
pub mod blas;
