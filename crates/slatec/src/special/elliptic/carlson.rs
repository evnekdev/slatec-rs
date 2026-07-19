//! Carlson's real symmetric elliptic integrals `R_C`, `R_F`, `R_D`, and `R_J`.
//!
//! # Status: Implemented
//!
//! Enable `special-scalar-expanded` for the f32 and f64 functions. Their
//! documented native range statuses remain explicit Rust errors.

#[cfg(feature = "special-scalar-expanded")]
pub use crate::special::scalar_expanded::{
    carlson_rc, carlson_rc_f32, carlson_rd, carlson_rd_f32, carlson_rf, carlson_rf_f32, carlson_rj,
    carlson_rj_f32,
};
