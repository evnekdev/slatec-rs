//! Safe scalar special functions backed by the selected original SLATEC FNLIB.
//!
//! This module is deliberately limited to the explicitly validated GNU
//! Fortran `x86_64-w64-mingw32` profile. The default `bundled` provider
//! checksum-verifies, compiles, and statically links the selected family
//! closure; explicit source, system, and externally managed backends remain
//! available.
//!
//! Calls that enter FNLIB are serialized because its `SAVE`d initialisation
//! and legacy error state are process-global.  The wrappers reject the
//! ordinary fatal-input domains before the native call.  They do not make a
//! cross-platform or general thread-safety claim.

mod error;
pub(crate) mod runtime;

#[cfg(feature = "special-airy")]
pub mod airy;
#[cfg(feature = "special-bessel")]
pub mod bessel;
#[cfg(feature = "special-elementary")]
pub mod elementary;
#[cfg(feature = "special-error")]
pub mod error_functions;
#[cfg(any(feature = "special-gamma", feature = "special-beta"))]
pub mod gamma;
#[cfg(feature = "special-integrals")]
pub mod integrals;

pub use error::SpecialFunctionError;
