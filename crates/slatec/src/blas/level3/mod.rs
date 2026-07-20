//! Slice-based safe wrappers for selected real BLAS Level 3 routines.
//!
//! Matrix multiplication lives in precision-specific modules so a call does
//! not pull the rest of Level 3 merely through a broad generated wrapper.

mod dgemm;
#[allow(dead_code)]
mod legacy;
mod sgemm;

pub use dgemm::{dgemm, dgemm_contiguous};
pub use legacy::{dsyrk, dtrmm, dtrsm, ssyrk, strmm, strsm};
pub use sgemm::{sgemm, sgemm_contiguous};
