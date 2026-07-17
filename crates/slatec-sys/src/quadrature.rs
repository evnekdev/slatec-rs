//! Raw GNU Fortran declarations for the reviewed adaptive-quadrature subset.
//!
//! The selected source declarations, compiled symbols, and scalar callback
//! shape are verified for `ffi-profile-gnu-mingw-x86_64`. These functions are
//! still unsafe: callers must uphold callback lifetime, workspace, pointer,
//! status, and process-global runtime invariants.

use crate::FortranInteger;

/// GNU Fortran scalar integrand callback for double precision.
pub type IntegrandF64 = unsafe extern "C" fn(*const f64) -> f64;

/// GNU Fortran scalar integrand callback for single precision.
pub type IntegrandF32 = unsafe extern "C" fn(*const f32) -> f32;

unsafe extern "C" {
    #[link_name = "dqag_"]
    pub fn dqag(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        key: *mut FortranInteger,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[link_name = "qag_"]
    pub fn qag(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        epsabs: *mut f32,
        epsrel: *mut f32,
        key: *mut FortranInteger,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[link_name = "dqags_"]
    pub fn dqags(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[link_name = "qags_"]
    pub fn qags(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[link_name = "dqagi_"]
    pub fn dqagi(
        f: IntegrandF64,
        bound: *mut f64,
        inf: *mut FortranInteger,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[link_name = "qagi_"]
    pub fn qagi(
        f: IntegrandF32,
        bound: *mut f32,
        inf: *mut FortranInteger,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[link_name = "dqawc_"]
    pub fn dqawc(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[link_name = "qawc_"]
    pub fn qawc(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limit: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );
}
