//! Safe scalar special functions backed by the selected original SLATEC FNLIB.
//!
//! This module is deliberately limited to the explicitly validated GNU
//! Fortran `x86_64-w64-mingw32` profile.  The native archive is linked only
//! when the application configures `SLATEC_NATIVE_LIB_DIR`; Cargo never
//! downloads or compiles Fortran implicitly.
//!
//! Calls that enter FNLIB are serialized because its `SAVE`d initialisation
//! and legacy error state are process-global.  The wrappers reject the
//! ordinary fatal-input domains before the native call.  They do not make a
//! cross-platform or general thread-safety claim.

mod error;
pub(crate) mod runtime;

pub mod airy;
pub mod bessel;
pub mod elementary;
pub mod error_functions;
pub mod gamma;
pub mod integrals;

pub use error::SpecialFunctionError;
