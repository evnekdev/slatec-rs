//! Raw GNU Fortran declarations for the reviewed scalar FZERO family.
//!
//! These declarations are specific to `ffi-profile-gnu-mingw-x86_64` and are
//! unsafe because the caller owns callback lifetime, all mutable scalar
//! pointers, and process-global runtime serialization.

use crate::FortranInteger;

/// GNU Fortran scalar function callback used by `DFZERO`.
pub type RootFnF64 = unsafe extern "C" fn(*const f64) -> f64;

/// GNU Fortran scalar function callback used by `FZERO`.
pub type RootFnF32 = unsafe extern "C" fn(*const f32) -> f32;

unsafe extern "C" {
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
