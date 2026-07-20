//! Slice-based safe wrappers for selected real BLAS Level 2 routines.
//!
//! Public operations are re-exported from operation-sized modules.  The
//! legacy implementation remains an internal compatibility implementation for
//! operations not yet physically split; it is never a production registry.

mod dgemv;
#[allow(dead_code)]
mod legacy;
mod sgemv;

pub use dgemv::{dgemv, dgemv_contiguous};
pub use legacy::{dger, dsymv, dtrmv, dtrsv, sger, ssymv, strmv, strsv};
pub use sgemv::{sgemv, sgemv_contiguous};
