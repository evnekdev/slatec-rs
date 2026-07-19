//! Hand-reviewed raw declarations for the standard real-array complex FFTPACK API.
//!
//! `CFFTI1`, `CFFTF1`, and `CFFTB1` are the selected snapshot's standard
//! Fortran-77 entry points. Their complex sequence is declared as a real
//! `DIMENSION C(*)` array holding adjacent real then imaginary `f32` words.
//! The safe wrapper supplies that interleaved pointer from a verified
//! `num_complex::Complex32` slice; raw callers remain responsible for all
//! layout, workspace, and lifetime requirements.

use crate::FortranInteger;

unsafe extern "C" {
    /// Initializes `WA[2*N]` and `IFAC[15]` for the CFFTF1/CFFTB1 pair.
    #[link_name = "cffti1_"]
    pub fn cffti1(n: *mut FortranInteger, wa: *mut f32, ifac: *mut FortranInteger);

    /// Computes the unnormalised forward complex transform in-place.
    ///
    /// `c` and `ch` each contain `2*N` interleaved real words, `wa` is the
    /// initialized `2*N` word table, and `ifac` has 15 integer entries.
    #[link_name = "cfftf1_"]
    pub fn cfftf1(
        n: *mut FortranInteger,
        c: *mut f32,
        ch: *mut f32,
        wa: *mut f32,
        ifac: *mut FortranInteger,
    );

    /// Computes the unnormalised backward complex transform in-place.
    ///
    /// Buffer requirements match [`cfftf1`].
    #[link_name = "cfftb1_"]
    pub fn cfftb1(
        n: *mut FortranInteger,
        c: *mut f32,
        ch: *mut f32,
        wa: *mut f32,
        ifac: *mut FortranInteger,
    );
}
