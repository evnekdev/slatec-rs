//! Private ABI-shaped declaration forwarding.
//!
//! This module never owns an FFI declaration; it only forwards items from the
//! private declaration layer for internal canonical modules.

#![allow(unused_imports)]

#[cfg(any(
    feature = "raw-family-blas-level1",
    feature = "raw-family-blas-level2",
    feature = "raw-family-blas-level3"
))]
pub mod blas {
    pub use crate::generated_ffi::blas::*;
}

#[cfg(feature = "raw-ffi-callbacks")]
pub mod callbacks {
    pub use crate::generated_ffi::callbacks::*;
}

#[cfg(feature = "raw-ffi-character")]
pub mod character {
    pub use crate::generated_ffi::character::*;
}

#[cfg(feature = "raw-ffi-complex-arguments")]
pub mod complex_arguments {
    pub use crate::generated_ffi::complex_arguments::*;
}

#[cfg(feature = "raw-ffi-complex-returns")]
pub mod complex_returns {
    pub use crate::generated_ffi::complex_returns::*;
}

#[cfg(feature = "raw-ffi-infrastructure")]
pub mod infrastructure {
    pub use crate::generated_ffi::infrastructure::*;
}

#[cfg(feature = "raw-ffi-logical")]
pub mod logical {
    pub use crate::generated_ffi::logical::*;
}

#[cfg(feature = "raw-ffi-numeric-array-subroutines")]
pub mod numeric_array_subroutines {
    pub use crate::generated_ffi::numeric_array_subroutines::*;
}

#[cfg(feature = "raw-ffi-numeric-scalar-subroutines")]
pub mod numeric_scalar_subroutines {
    pub use crate::generated_ffi::numeric_scalar_subroutines::*;
}

#[cfg(feature = "raw-ffi-scalar-functions")]
pub mod scalar_functions {
    pub use crate::generated_ffi::scalar_functions::*;
}
