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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqagp.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qagp.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqaws.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qaws.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqawo.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qawo.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqawf.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qawf.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqng.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qng.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqnc79.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qnc79.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqag.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qag.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqags.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qags.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqagi.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qagi.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqawc.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qawc.md"))]
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

#[cfg(feature = "raw-family-quadrature-direct")]
#[path = "batch_a/quadrature.rs"]
mod canonical_bindings;

/// Canonical source-verified non-callback quadrature declarations.
#[cfg(feature = "raw-family-quadrature-direct")]
pub use canonical_bindings::*;

#[cfg(feature = "raw-family-quadrature-callbacks")]
#[path = "batch_b/quadrature.rs"]
mod callback_bindings;

/// Canonical source-verified callback-bearing quadrature declarations.
#[cfg(feature = "raw-family-quadrature-callbacks")]
pub use callback_bindings::callbacks;
