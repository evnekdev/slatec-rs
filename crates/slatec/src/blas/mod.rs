//! Safe BLAS façades over the selected, validated SLATEC raw FFI profile.

/// Validation and native-contract errors returned by safe BLAS wrappers.
pub mod error;
/// Type-safe replacements for BLAS character selector arguments.
pub mod selectors;
#[allow(dead_code)]
pub(crate) mod validation;

#[cfg(feature = "blas-level1")]
/// Safe real-valued BLAS Level 1 operations on caller-owned slices.
pub mod level1;

#[cfg(feature = "blas-level2")]
/// Safe real-valued BLAS Level 2 operations on column-major slices.
pub mod level2;

#[cfg(feature = "blas-level3")]
/// Safe real-valued BLAS Level 3 operations on column-major slices.
pub mod level3;

pub use error::BlasError;
pub use selectors::{Diagonal, Side, Transpose, Triangle};
