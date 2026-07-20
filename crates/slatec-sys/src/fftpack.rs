//! Hand-reviewed raw declarations for selected real FFTPACK routines.
//!
//! These are the single-precision routines present in the selected SLATEC
//! `fishfft` snapshot.  The `N` argument is passed by mutable Fortran
//! reference for ABI compatibility but is an input in the reviewed routines.
//! Mutability of array arguments is declared per routine below; safe plan
//! wrappers own the workspace and validate all slice lengths before calling
//! them.

use crate::FortranInteger;

unsafe extern "C" {
    /// Initializes a periodic real FFT workspace of at least `2*N + 15` words.
    #[link_name = "rffti_"]
    pub fn rffti(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes an in-place packed periodic real forward transform.
    #[link_name = "rfftf_"]
    pub fn rfftf(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);
    /// Computes an in-place packed periodic real backward transform.
    #[link_name = "rfftb_"]
    pub fn rfftb(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);

    /// Initializes an easy real Fourier transform workspace of at least `3*N + 15` words.
    #[link_name = "ezffti_"]
    pub fn ezffti(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes separate easy real Fourier coefficients.
    #[link_name = "ezfftf_"]
    pub fn ezfftf(
        n: *mut FortranInteger,
        values: *const f32,
        azero: *mut f32,
        cosine: *mut f32,
        sine: *mut f32,
        wsave: *mut f32,
    );
    /// Synthesizes a real periodic sequence from easy Fourier coefficients.
    #[link_name = "ezfftb_"]
    pub fn ezfftb(
        n: *mut FortranInteger,
        values: *mut f32,
        azero: *const f32,
        cosine: *const f32,
        sine: *const f32,
        wsave: *mut f32,
    );

    /// Initializes a full sine-transform workspace.
    #[link_name = "sinti_"]
    pub fn sinti(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes the in-place full sine transform.
    #[link_name = "sint_"]
    pub fn sint(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);

    /// Initializes a full cosine-transform workspace.
    #[link_name = "costi_"]
    pub fn costi(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes the in-place full cosine transform.
    #[link_name = "cost_"]
    pub fn cost(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);

    /// Initializes a quarter-wave sine-transform workspace.
    #[link_name = "sinqi_"]
    pub fn sinqi(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes the in-place quarter-wave sine forward transform.
    #[link_name = "sinqf_"]
    pub fn sinqf(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);
    /// Computes the in-place quarter-wave sine backward transform.
    #[link_name = "sinqb_"]
    pub fn sinqb(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);

    /// Initializes a quarter-wave cosine-transform workspace.
    #[link_name = "cosqi_"]
    pub fn cosqi(n: *mut FortranInteger, wsave: *mut f32);
    /// Computes the in-place quarter-wave cosine forward transform.
    #[link_name = "cosqf_"]
    pub fn cosqf(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);
    /// Computes the in-place quarter-wave cosine backward transform.
    #[link_name = "cosqb_"]
    pub fn cosqb(n: *mut FortranInteger, values: *mut f32, wsave: *mut f32);
}

#[cfg(feature = "raw-family-fftpack-extended-real")]
#[path = "batch_a/fftpack.rs"]
mod canonical_bindings;

/// Canonical source-verified FFTPACK declarations.
#[cfg(feature = "raw-family-fftpack-extended-real")]
pub use canonical_bindings::*;
