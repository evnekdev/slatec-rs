//! Nonlinear least-squares fitting.
//!
//! # Status: Implemented with its feature gates.

#[path = "nonlinear/easy.rs"]
/// Easy residual-only fitting.
pub mod easy;
#[path = "nonlinear/expert.rs"]
/// Expert fitting.
pub mod expert;
