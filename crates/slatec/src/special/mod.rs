//! Safe scalar special functions backed by the selected original SLATEC FNLIB.
//!
//! This module is deliberately limited to the explicitly validated GNU
//! Fortran `x86_64-w64-mingw32` profile. Native execution requires one explicit
//! provider. The offline `source-build` mode verifies and compiles the selected
//! family closure; system and externally managed backends remain available.
//!
//! Calls that enter FNLIB are serialized because its `SAVE`d initialisation
//! and legacy error state are process-global.  The wrappers reject the
//! ordinary fatal-input domains before the native call.  They do not make a
//! cross-platform or general thread-safety claim.
//!
//! The mathematical organization includes legacy scalar FNLIB families,
//! exponential, logarithmic, and Spence integrals, and Carlson symmetric
//! elliptic integrals. Scaled functions remain explicitly named. Complex,
//! sequence, and workspace-oriented special-function APIs remain deferred.

#[cfg(any(
    feature = "special-elementary",
    feature = "special-gamma",
    feature = "special-beta",
    feature = "special-error",
    feature = "special-airy",
    feature = "special-bessel",
    feature = "special-integrals",
    feature = "special-scalar-expanded"
))]
pub(crate) mod runtime;
#[path = "error.rs"]
mod special_error;

#[cfg(feature = "special-airy")]
pub mod airy;
#[cfg(not(feature = "special-airy"))]
#[path = "placeholders/airy.rs"]
pub mod airy;
#[cfg(feature = "special-bessel")]
pub mod bessel;
#[cfg(not(feature = "special-bessel"))]
#[path = "placeholders/bessel.rs"]
pub mod bessel;
/// Beta-function compatibility view.
pub mod beta;
#[cfg(feature = "special-elementary")]
pub mod elementary;
#[cfg(not(feature = "special-elementary"))]
#[path = "placeholders/elementary.rs"]
pub mod elementary;
/// Carlson symmetric elliptic-integral organization.
pub mod elliptic;
/// Error-function compatibility view.
#[path = "error_family.rs"]
pub mod error;
#[cfg(feature = "special-error")]
pub mod error_functions;
#[cfg(not(feature = "special-error"))]
#[path = "placeholders/error.rs"]
pub mod error_functions;
#[cfg(any(feature = "special-gamma", feature = "special-beta"))]
pub mod gamma;
#[cfg(not(any(feature = "special-gamma", feature = "special-beta")))]
#[path = "placeholders/gamma.rs"]
pub mod gamma;
#[cfg(feature = "special-integrals")]
pub mod integrals;
#[cfg(not(feature = "special-integrals"))]
#[path = "placeholders/integrals.rs"]
pub mod integrals;
/// Compatibility view of scalar orthogonal-polynomial functions.
pub mod polynomials;
/// Probability-related scalar functions.
pub mod probability;
#[cfg(feature = "special-scalar-expanded")]
#[doc(hidden)]
pub mod scalar_expanded;

pub use special_error::SpecialFunctionError;
