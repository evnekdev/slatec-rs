//! Endpoint-weighted quadrature.
//!
//! # Status: Implemented
//!
//! Enable `quadrature-weighted`.

#[cfg(feature = "quadrature-weighted")]
pub use crate::quadrature::{
    EndpointWeight, integrate_weighted_endpoints, integrate_weighted_endpoints_f32,
};
