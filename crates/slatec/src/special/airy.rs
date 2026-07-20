//! Real Airy functions with a conservative overflow-safe input envelope.
//!
//! Each raw call lives in a private operation-sized module.  This preserves the
//! public namespace while allowing a final native link to discard unrelated
//! Airy operations.

mod ai;
#[cfg(feature = "special-f32")]
mod ai_f32;
mod ai_scaled;
#[cfg(feature = "special-f32")]
mod ai_scaled_f32;
mod bi;
#[cfg(feature = "special-f32")]
mod bi_f32;
mod bi_scaled;
#[cfg(feature = "special-f32")]
mod bi_scaled_f32;

pub use ai::airy_ai;
#[cfg(feature = "special-f32")]
pub use ai_f32::airy_ai_f32;
pub use ai_scaled::airy_ai_scaled;
#[cfg(feature = "special-f32")]
pub use ai_scaled_f32::airy_ai_scaled_f32;
pub use bi::airy_bi;
#[cfg(feature = "special-f32")]
pub use bi_f32::airy_bi_f32;
pub use bi_scaled::airy_bi_scaled;
#[cfg(feature = "special-f32")]
pub use bi_scaled_f32::airy_bi_scaled_f32;
