//! Exponential integrals `E1` and `Ei`.
//!
//! # Status: Partial
//!
//! Enable `special-integrals` for the reviewed real scalar functions.

#[cfg(feature = "special-integrals")]
pub use super::{
    exponential_integral_e1, exponential_integral_e1_f32, exponential_integral_ei,
    exponential_integral_ei_f32,
};
