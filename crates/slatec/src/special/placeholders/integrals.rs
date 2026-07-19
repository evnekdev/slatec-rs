//! Classical real scalar integral functions.
//!
//! # Status: Partial
//!
//! Enable `special-integrals` for exponential integrals. The logarithmic and
//! Spence leaves become callable with `special-scalar-expanded`.

pub mod exponential {
    //! Exponential integrals.
    //!
    //! # Status: Partial
}

pub mod logarithmic {
    //! The real logarithmic integral.
    //!
    //! # Status: Implemented with `special-scalar-expanded`.

    #[cfg(feature = "special-scalar-expanded")]
    pub use crate::special::scalar_expanded::{logarithmic_integral, logarithmic_integral_f32};
}

pub mod spence {
    //! Spence's real dilogarithmic integral.
    //!
    //! # Status: Implemented with `special-scalar-expanded`.

    #[cfg(feature = "special-scalar-expanded")]
    pub use crate::special::scalar_expanded::{spence_integral, spence_integral_f32};
}

pub mod trigonometric {
    //! Trigonometric integral functions.
    //!
    //! # Status: Planned
}
