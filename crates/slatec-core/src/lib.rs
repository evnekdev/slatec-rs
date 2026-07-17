#![no_std]
#![deny(missing_docs)]

//! Shared safe infrastructure for the supported SLATEC ABI profiles.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub mod fortran;

#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub use fortran::{IntegerRangeError, to_fortran_increment, to_fortran_integer};
