//! Safe BLAS façades over the selected, validated SLATEC raw FFI profile.

pub mod error;
pub mod selectors;
#[allow(dead_code)]
pub(crate) mod validation;

#[cfg(feature = "blas-level1")]
pub mod level1;

#[cfg(feature = "blas-level2")]
pub mod level2;

#[cfg(feature = "blas-level3")]
pub mod level3;

pub use error::BlasError;
pub use selectors::{Diagonal, Side, Transpose, Triangle};
