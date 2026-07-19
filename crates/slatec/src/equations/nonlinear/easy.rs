//! Easy finite-difference nonlinear-system solving.
//!
//! # Status: Implemented
//!
//! Enable `nonlinear-easy`.

#[cfg(feature = "nonlinear-easy")]
pub use crate::nonlinear::{
    NonlinearError, NonlinearOptions, NonlinearResult, NonlinearStatus, solve_system,
    solve_system_f32,
};
