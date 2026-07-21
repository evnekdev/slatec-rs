//! Raw GNU Fortran declarations for the reviewed scalar FZERO family.
//!
//! These declarations are specific to `ffi-profile-gnu-mingw-x86_64` and are
//! unsafe because the caller owns callback lifetime, all mutable scalar
//! pointers, and process-global runtime serialization.

#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
use crate::FortranInteger;

/// GNU Fortran scalar function callback used by `DFZERO`.
#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
pub type RootFnF64 = unsafe extern "C" fn(*const f64) -> f64;

/// GNU Fortran scalar function callback used by `FZERO`.
#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
pub type RootFnF32 = unsafe extern "C" fn(*const f32) -> f32;

#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
unsafe extern "C" {
    /// Finds a zero of the double-precision callback `F`.
    ///
    /// Original SLATEC routine: `DFZERO`; supported ABI profile:
    /// `ffi-profile-gnu-mingw-x86_64`; native symbol: `dfzero_`. Source:
    /// <https://www.netlib.org/slatec/src/dfzero.f>.
    ///
    /// # Arguments
    ///
    /// - `F` is a non-null input callback. It is invoked synchronously with a
    ///   pointer to a readable `f64`; native code does not retain it.
    /// - `B` and `C` are non-null in/out pointers to the initial bracket
    ///   endpoints and returned enclosing interval.
    /// - `R` is a non-null output pointer for the computed root estimate.
    /// - `RE` and `AE` are non-null input pointers to relative and absolute
    ///   error tolerances.
    /// - `IFLAG` is a non-null output pointer for the historical completion
    ///   code: `1` is normal bracketed-root completion; `2` means `B` was
    ///   exactly zero; `3` reports a possible singular point; `4` reports no
    ///   sign change after interval collapse; and `5` reports more than 500
    ///   callback evaluations. Callers must inspect this value after the call.
    ///
    /// # Safety
    ///
    /// `F` must follow the GNU Fortran callback ABI, remain valid throughout
    /// the call, and never unwind into Fortran. Every scalar pointer must be
    /// non-null, correctly aligned, and valid for the documented access; no
    /// pointer may alias mutable storage in a way that violates Rust's aliasing
    /// rules. The caller must serialize use of legacy SLATEC runtime state.
    #[link_name = "dfzero_"]
    pub fn dfzero(
        function: RootFnF64,
        b: *mut f64,
        c: *mut f64,
        r: *mut f64,
        relative_error: *mut f64,
        absolute_error: *mut f64,
        iflag: *mut FortranInteger,
    );

    /// Finds a zero of the single-precision callback `F`.
    ///
    /// Original SLATEC routine: `FZERO`; supported ABI profile:
    /// `ffi-profile-gnu-mingw-x86_64`; native symbol: `fzero_`. Source:
    /// <https://www.netlib.org/slatec/src/fzero.f>.
    ///
    /// # Arguments
    ///
    /// - `F` is a non-null input callback. It is invoked synchronously with a
    ///   pointer to a readable `f32`; native code does not retain it.
    /// - `B` and `C` are non-null in/out pointers to the initial bracket
    ///   endpoints and returned enclosing interval.
    /// - `R` is a non-null output pointer for the computed root estimate.
    /// - `RE` and `AE` are non-null input pointers to relative and absolute
    ///   error tolerances.
    /// - `IFLAG` is a non-null output pointer for the historical completion
    ///   code: `1` is normal bracketed-root completion; `2` means `B` was
    ///   exactly zero; `3` reports a possible singular point; `4` reports no
    ///   sign change after interval collapse; and `5` reports more than 500
    ///   callback evaluations. Callers must inspect this value after the call.
    ///
    /// # Safety
    ///
    /// `F` must follow the GNU Fortran callback ABI, remain valid throughout
    /// the call, and never unwind into Fortran. Every scalar pointer must be
    /// non-null, correctly aligned, and valid for the documented access; no
    /// pointer may alias mutable storage in a way that violates Rust's aliasing
    /// rules. The caller must serialize use of legacy SLATEC runtime state.
    #[link_name = "fzero_"]
    pub fn fzero(
        function: RootFnF32,
        b: *mut f32,
        c: *mut f32,
        r: *mut f32,
        relative_error: *mut f32,
        absolute_error: *mut f32,
        iflag: *mut FortranInteger,
    );
}

/// Canonical scalar root-finding namespace.
///
/// No additional `extern` declarations live here.
#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
pub mod scalar {
    pub use super::{RootFnF32, RootFnF64, dfzero, fzero};
}

/// Canonical complex polynomial and scalar-root interfaces.
///
/// These routines solve polynomial or complex scalar zero problems and
/// re-export the one authoritative FFI declaration.
#[cfg(feature = "raw-family-nonlinear-complex")]
pub mod complex {
    pub use crate::abi_bindings::nonlinear::{cpqr79, cpzero, rpqr79, rpzero};
}
