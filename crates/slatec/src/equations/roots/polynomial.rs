//! Owned single-precision polynomial roots.
//!
//! # Status: Implemented
//!
//! Enable `roots-polynomial`. The canonical safe API is
//! `crate::roots::polynomial`; this module is its mathematical-organization
//! re-export.

#[cfg(feature = "roots-polynomial")]
pub use crate::roots::polynomial::*;
