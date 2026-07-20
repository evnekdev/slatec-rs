//! Gamma, beta, and related scalar functions with fatal domains rejected
//! before the original FNLIB implementation is entered.
//!
//! The primary `gamma` entry point lives in its own module so calling it does
//! not make other scalar raw references reachable through one broad object.

mod direct;
#[allow(dead_code)]
mod legacy;

pub use direct::gamma;
pub use legacy::*;
