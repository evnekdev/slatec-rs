//! Canonical generated raw linear-algebra declarations.
//!
//! The namespace separates real and complex dense, banded, packed, sparse,
//! iterative, and eigenproblem interfaces without exposing ABI-generation
//! details.

#[cfg(feature = "raw-family-linear-algebra-real")]
#[path = "batch_a/linear_algebra.rs"]
mod canonical_bindings;

#[cfg(feature = "raw-family-linear-algebra-eigen")]
#[path = "batch_a/eigen.rs"]
mod canonical_eigen_bindings;

#[cfg(feature = "raw-family-linear-algebra-iterative")]
#[path = "batch_b/linear_algebra.rs"]
mod callback_bindings;

/// Dense real and complex linear-algebra interfaces.
#[cfg(any(
    feature = "raw-family-linear-algebra-real",
    feature = "raw-family-linear-algebra-complex"
))]
pub mod dense {
    #[cfg(feature = "raw-family-linear-algebra-real")]
    pub use super::canonical_bindings::dense::*;

    /// Complex LINPACK and related dense-system interfaces.
    #[cfg(feature = "raw-family-linear-algebra-complex")]
    pub mod complex {
        pub use crate::abi_bindings::linear_algebra::dense::*;

        #[deprecated(note = "use `slatec_sys::linear_algebra::banded::complex`")]
        pub use crate::abi_bindings::linear_algebra::banded::*;
        #[deprecated(note = "use `slatec_sys::linear_algebra::packed::complex`")]
        pub use crate::abi_bindings::linear_algebra::packed::*;
    }
}

/// Banded-storage linear systems.
#[cfg(any(
    feature = "raw-family-linear-algebra-real",
    feature = "raw-family-linear-algebra-complex"
))]
pub mod banded {
    #[cfg(feature = "raw-family-linear-algebra-real")]
    pub use super::canonical_bindings::banded::*;

    /// Complex banded-system interfaces.
    #[cfg(feature = "raw-family-linear-algebra-complex")]
    pub mod complex {
        pub use crate::abi_bindings::linear_algebra::banded::*;
    }
}

/// Packed-storage linear systems.
#[cfg(any(
    feature = "raw-family-linear-algebra-real",
    feature = "raw-family-linear-algebra-complex"
))]
pub mod packed {
    #[cfg(feature = "raw-family-linear-algebra-real")]
    pub use super::canonical_bindings::packed::*;

    /// Complex packed-system interfaces.
    #[cfg(feature = "raw-family-linear-algebra-complex")]
    pub mod complex {
        pub use crate::abi_bindings::linear_algebra::packed::*;
    }
}

/// Sparse and callback-driven iterative linear algebra.
#[cfg(any(
    feature = "raw-family-linear-algebra-real",
    feature = "raw-family-linear-algebra-iterative"
))]
pub mod sparse {
    #[cfg(feature = "raw-family-linear-algebra-real")]
    pub use super::canonical_bindings::sparse::*;
    #[cfg(feature = "raw-family-linear-algebra-iterative")]
    pub use super::callback_bindings::sparse::*;
}

/// Eigenvalue and eigenvector routines.
#[cfg(feature = "raw-family-linear-algebra-eigen")]
pub mod eigen {
    pub use super::canonical_eigen_bindings::*;
}
