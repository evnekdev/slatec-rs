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
    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-breakpoints"
    ))]
    #[link_name = "dqagp_"]
    pub fn dqagp(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        npts2: *mut FortranInteger,
        points: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        leniw: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-breakpoints"
    ))]
    #[link_name = "qagp_"]
    pub fn qagp(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        npts2: *mut FortranInteger,
        points: *mut f32,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        leniw: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-weighted"
    ))]
    #[link_name = "dqaws_"]
    pub fn dqaws(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        alfa: *mut f64,
        beta: *mut f64,
        integr: *mut FortranInteger,
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-weighted"
    ))]
    #[link_name = "qaws_"]
    pub fn qaws(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        alfa: *mut f32,
        beta: *mut f32,
        integr: *mut FortranInteger,
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-oscillatory"
    ))]
    #[link_name = "dqawo_"]
    pub fn dqawo(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        omega: *mut f64,
        integr: *mut FortranInteger,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        leniw: *mut FortranInteger,
        maxp1: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-oscillatory"
    ))]
    #[link_name = "qawo_"]
    pub fn qawo(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        omega: *mut f32,
        integr: *mut FortranInteger,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        leniw: *mut FortranInteger,
        maxp1: *mut FortranInteger,
        lenw: *mut FortranInteger,
        last: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-fourier"
    ))]
    #[link_name = "dqawf_"]
    pub fn dqawf(
        f: IntegrandF64,
        a: *mut f64,
        omega: *mut f64,
        integr: *mut FortranInteger,
        epsabs: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limlst: *mut FortranInteger,
        lst: *mut FortranInteger,
        leniw: *mut FortranInteger,
        maxp1: *mut FortranInteger,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f64,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-fourier"
    ))]
    #[link_name = "qawf_"]
    pub fn qawf(
        f: IntegrandF32,
        a: *mut f32,
        omega: *mut f32,
        integr: *mut FortranInteger,
        epsabs: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
        limlst: *mut FortranInteger,
        lst: *mut FortranInteger,
        leniw: *mut FortranInteger,
        maxp1: *mut FortranInteger,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        work: *mut f32,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-nonadaptive"
    ))]
    #[link_name = "dqng_"]
    pub fn dqng(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        epsabs: *mut f64,
        epsrel: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-nonadaptive"
    ))]
    #[link_name = "qng_"]
    pub fn qng(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        epsabs: *mut f32,
        epsrel: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        neval: *mut FortranInteger,
        ier: *mut FortranInteger,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-nonadaptive"
    ))]
    #[link_name = "dqnc79_"]
    pub fn dqnc79(
        f: IntegrandF64,
        a: *mut f64,
        b: *mut f64,
        err: *mut f64,
        ans: *mut f64,
        ierr: *mut FortranInteger,
        evaluations: *mut FortranInteger,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-nonadaptive"
    ))]
    #[link_name = "qnc79_"]
    pub fn qnc79(
        f: IntegrandF32,
        a: *mut f32,
        b: *mut f32,
        err: *mut f32,
        ans: *mut f32,
        ierr: *mut FortranInteger,
        evaluations: *mut FortranInteger,
    );

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

    #[cfg(any(
        feature = "raw-ffi-quadrature",
        feature = "raw-family-quadrature-basic"
    ))]
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

#[cfg(feature = "raw-family-batch-a-quadrature")]
#[path = "batch_a/quadrature.rs"]
mod batch_a;

/// Canonical source-verified Batch A non-callback quadrature declarations.
#[cfg(feature = "raw-family-batch-a-quadrature")]
pub use batch_a::numerical;

#[cfg(feature = "raw-family-batch-b-quadrature")]
#[path = "batch_b/quadrature.rs"]
mod batch_b;

/// Canonical source-verified Batch B callback-bearing quadrature declarations.
#[cfg(feature = "raw-family-batch-b-quadrature")]
pub use batch_b::callbacks;
