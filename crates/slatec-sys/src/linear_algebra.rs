//! Canonical generated raw linear-algebra declarations.
//!
//! Batch A owns the straightforward non-callback declarations. Batch B adds
//! source-verified sparse iterative callback drivers under an explicit callback
//! namespace.
//! Batch C adds compiler-validated complex LINPACK interfaces.

#[cfg(feature = "raw-family-batch-a-linear-algebra")]
#[path = "batch_a/linear_algebra.rs"]
mod batch_a;

#[cfg(feature = "raw-family-batch-b-linear-algebra")]
#[path = "batch_b/linear_algebra.rs"]
mod batch_b;

/// Dense real and complex linear-algebra interfaces.
#[cfg(any(
    feature = "raw-family-batch-a-linear-algebra",
    feature = "raw-family-batch-c-linear-algebra"
))]
pub mod dense {
    #[cfg(feature = "raw-family-batch-a-linear-algebra")]
    pub use super::batch_a::dense::*;

    /// Complex LINPACK and related dense-system interfaces promoted by Batch C.
    #[cfg(feature = "raw-family-batch-c-linear-algebra")]
    pub mod complex {
        pub use crate::batch_c::linear_algebra::*;
    }
}

#[cfg(feature = "raw-family-batch-b-linear-algebra")]
pub use batch_b::sparse;
