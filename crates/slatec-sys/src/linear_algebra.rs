//! Canonical generated raw linear-algebra declarations.
//!
//! Batch A owns the straightforward non-callback declarations. Batch B adds
//! source-verified sparse iterative callback drivers under an explicit callback
//! namespace.

#[cfg(feature = "raw-family-batch-a-linear-algebra")]
#[path = "batch_a/linear_algebra.rs"]
mod batch_a;

#[cfg(feature = "raw-family-batch-b-linear-algebra")]
#[path = "batch_b/linear_algebra.rs"]
mod batch_b;

#[cfg(feature = "raw-family-batch-a-linear-algebra")]
pub use batch_a::dense;

#[cfg(feature = "raw-family-batch-b-linear-algebra")]
pub use batch_b::sparse;
