//! Shared safe infrastructure for the supported SLATEC ABI profiles.

#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub mod fortran;

#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub use fortran::{IntegerRangeError, to_fortran_increment, to_fortran_integer};
