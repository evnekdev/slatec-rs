//! Safe BLAS façades over the selected, validated SLATEC raw FFI profile.

pub mod error;
#[allow(dead_code)]
pub(crate) mod validation;

#[cfg(feature = "blas-level1")]
pub mod level1;

pub use error::BlasError;
