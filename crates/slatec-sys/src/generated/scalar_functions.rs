//! Generated raw declarations for `batch_scalar_functions`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "acos_"]
    pub fn acos(x: *mut f32) -> f32;
    #[link_name = "alog_"]
    pub fn alog(x: *mut f32) -> f32;
    #[link_name = "alog10_"]
    pub fn alog10(x: *mut f32) -> f32;
    #[link_name = "asin_"]
    pub fn asin(x: *mut f32) -> f32;
    #[link_name = "atan_"]
    pub fn atan(x: *mut f32) -> f32;
    #[link_name = "atan2_"]
    pub fn atan2(sn: *mut f32, cs: *mut f32) -> f32;
    #[link_name = "bcrh_"]
    pub fn bcrh(
        xll: *mut f32,
        xrr: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
        f: *mut f32,
        sgn: *mut f32,
    ) -> f32;
    #[link_name = "bsrh_"]
    pub fn bsrh(
        xll: *mut f32,
        xrr: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
        f: *mut f32,
        sgn: *mut f32,
    ) -> f32;
    #[link_name = "chfcm_"]
    pub fn chfcm(d1: *mut f32, d2: *mut f32, delta: *mut f32) -> FortranInteger;
    #[link_name = "chfie_"]
    pub fn chfie(
        x1: *mut f32,
        x2: *mut f32,
        f1: *mut f32,
        f2: *mut f32,
        d1: *mut f32,
        d2: *mut f32,
        a: *mut f32,
        b: *mut f32,
    ) -> f32;
    #[link_name = "cos_"]
    pub fn cos(x: *mut f32) -> f32;
    #[link_name = "cosh_"]
    pub fn cosh(x: *mut f32) -> f32;
    #[link_name = "d9atn1_"]
    pub fn d9atn1(x: *mut f64) -> f64;
    #[link_name = "d9chu_"]
    pub fn d9chu(a: *mut f64, b: *mut f64, z: *mut f64) -> f64;
    #[link_name = "d9gmic_"]
    pub fn d9gmic(a: *mut f64, x: *mut f64, alx: *mut f64) -> f64;
    #[link_name = "d9gmit_"]
    pub fn d9gmit(
        a: *mut f64,
        x: *mut f64,
        algap1: *mut f64,
        sgngam: *mut f64,
        alx: *mut f64,
    ) -> f64;
    #[link_name = "d9lgic_"]
    pub fn d9lgic(a: *mut f64, x: *mut f64, alx: *mut f64) -> f64;
    #[link_name = "d9lgit_"]
    pub fn d9lgit(a: *mut f64, x: *mut f64, algap1: *mut f64) -> f64;
    #[link_name = "d9lgmc_"]
    pub fn d9lgmc(x: *mut f64) -> f64;
    #[link_name = "d9ln2r_"]
    pub fn d9ln2r(x: *mut f64) -> f64;
    #[link_name = "d9pak_"]
    pub fn d9pak(y: *mut f64, n: *mut FortranInteger) -> f64;
    #[link_name = "dacos_"]
    pub fn dacos(x: *mut f64) -> f64;
    #[link_name = "dasin_"]
    pub fn dasin(x: *mut f64) -> f64;
    #[link_name = "datan_"]
    pub fn datan(x: *mut f64) -> f64;
    #[link_name = "datan2_"]
    pub fn datan2(sn: *mut f64, cs: *mut f64) -> f64;
    #[link_name = "dchfcm_"]
    pub fn dchfcm(d1: *mut f64, d2: *mut f64, delta: *mut f64) -> FortranInteger;
    #[link_name = "dchfie_"]
    pub fn dchfie(
        x1: *mut f64,
        x2: *mut f64,
        f1: *mut f64,
        f2: *mut f64,
        d1: *mut f64,
        d2: *mut f64,
        a: *mut f64,
        b: *mut f64,
    ) -> f64;
    #[link_name = "dcos_"]
    pub fn dcos(x: *mut f64) -> f64;
    #[link_name = "dcosh_"]
    pub fn dcosh(x: *mut f64) -> f64;
    #[link_name = "ddanrm_"]
    pub fn ddanrm(
        neq: *mut FortranInteger,
        v: *mut f64,
        wt: *mut f64,
        rpar: *mut f64,
        ipar: *mut FortranInteger,
    ) -> f64;
    #[link_name = "denorm_"]
    pub fn denorm(n: *mut FortranInteger, x: *mut f64) -> f64;
    #[link_name = "dexp_"]
    pub fn dexp(x: *mut f64) -> f64;
    #[link_name = "dgamln_"]
    pub fn dgamln(z: *mut f64, ierr: *mut FortranInteger) -> f64;
    #[link_name = "dgamrn_"]
    pub fn dgamrn(x: *mut f64) -> f64;
    #[link_name = "dhvnrm_"]
    pub fn dhvnrm(v: *mut f64, ncomp: *mut FortranInteger) -> f64;
    #[link_name = "dint_"]
    pub fn dint(x: *mut f64) -> f64;
    #[link_name = "dlog_"]
    pub fn dlog(x: *mut f64) -> f64;
    #[link_name = "dlog10_"]
    pub fn dlog10(x: *mut f64) -> f64;
    #[link_name = "dpchdf_"]
    pub fn dpchdf(
        k: *mut FortranInteger,
        x: *mut f64,
        s: *mut f64,
        ierr: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dpchst_"]
    pub fn dpchst(arg1: *mut f64, arg2: *mut f64) -> f64;
    #[link_name = "dprvec_"]
    pub fn dprvec(m: *mut FortranInteger, u: *mut f64, v: *mut f64) -> f64;
    #[link_name = "dpsixn_"]
    pub fn dpsixn(n: *mut FortranInteger) -> f64;
    #[link_name = "dqwgtc_"]
    pub fn dqwgtc(
        x: *mut f64,
        c: *mut f64,
        p2: *mut f64,
        p3: *mut f64,
        p4: *mut f64,
        kp: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dqwgtf_"]
    pub fn dqwgtf(
        x: *mut f64,
        omega: *mut f64,
        p2: *mut f64,
        p3: *mut f64,
        p4: *mut f64,
        integr: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dqwgts_"]
    pub fn dqwgts(
        x: *mut f64,
        a: *mut f64,
        b: *mut f64,
        alfa: *mut f64,
        beta: *mut f64,
        integr: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dsin_"]
    pub fn dsin(x: *mut f64) -> f64;
    #[link_name = "dsinh_"]
    pub fn dsinh(x: *mut f64) -> f64;
    #[link_name = "dsqrt_"]
    pub fn dsqrt(x: *mut f64) -> f64;
    #[link_name = "dtan_"]
    pub fn dtan(x: *mut f64) -> f64;
    #[link_name = "dtanh_"]
    pub fn dtanh(x: *mut f64) -> f64;
    #[link_name = "dvnrms_"]
    pub fn dvnrms(n: *mut FortranInteger, v: *mut f64, w: *mut f64) -> f64;
    #[link_name = "dxpsi_"]
    pub fn dxpsi(a: *mut f64, ipsik: *mut FortranInteger, ipsix: *mut FortranInteger) -> f64;
    #[link_name = "enorm_"]
    pub fn enorm(n: *mut FortranInteger, x: *mut f32) -> f32;
    #[link_name = "exp_"]
    pub fn exp(x: *mut f32) -> f32;
    #[link_name = "gamln_"]
    pub fn gamln(z: *mut f32, ierr: *mut FortranInteger) -> f32;
    #[link_name = "gamrn_"]
    pub fn gamrn(x: *mut f32) -> f32;
    #[link_name = "hvnrm_"]
    pub fn hvnrm(v: *mut f32, ncomp: *mut FortranInteger) -> f32;
    #[link_name = "idloc_"]
    pub fn idloc(loc: *mut FortranInteger, sx: *mut f64, ix: *mut FortranInteger)
    -> FortranInteger;
    #[link_name = "iploc_"]
    pub fn iploc(loc: *mut FortranInteger, sx: *mut f32, ix: *mut FortranInteger)
    -> FortranInteger;
    #[link_name = "numxer_"]
    pub fn numxer(nerr: *mut FortranInteger) -> FortranInteger;
    #[link_name = "pchdf_"]
    pub fn pchdf(
        k: *mut FortranInteger,
        x: *mut f32,
        s: *mut f32,
        ierr: *mut FortranInteger,
    ) -> f32;
    #[link_name = "pchst_"]
    pub fn pchst(arg1: *mut f32, arg2: *mut f32) -> f32;
    #[link_name = "pgsf_"]
    pub fn pgsf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "pimach_"]
    pub fn pimach(dum: *mut f32) -> f32;
    #[link_name = "ppgsf_"]
    pub fn ppgsf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "pppsf_"]
    pub fn pppsf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "ppsgf_"]
    pub fn ppsgf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "ppspf_"]
    pub fn ppspf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "prvec_"]
    pub fn prvec(m: *mut FortranInteger, u: *mut f32, v: *mut f32) -> f32;
    #[link_name = "psgf_"]
    pub fn psgf(
        x: *mut f32,
        iz: *mut FortranInteger,
        c: *mut f32,
        a: *mut f32,
        bh: *mut f32,
    ) -> f32;
    #[link_name = "psixn_"]
    pub fn psixn(n: *mut FortranInteger) -> f32;
    #[link_name = "pythag_"]
    pub fn pythag(a: *mut f32, b: *mut f32) -> f32;
    #[link_name = "qwgtc_"]
    pub fn qwgtc(
        x: *mut f32,
        c: *mut f32,
        p2: *mut f32,
        p3: *mut f32,
        p4: *mut f32,
        kp: *mut FortranInteger,
    ) -> f32;
    #[link_name = "qwgtf_"]
    pub fn qwgtf(
        x: *mut f32,
        omega: *mut f32,
        p2: *mut f32,
        p3: *mut f32,
        p4: *mut f32,
        integr: *mut FortranInteger,
    ) -> f32;
    #[link_name = "qwgts_"]
    pub fn qwgts(
        x: *mut f32,
        a: *mut f32,
        b: *mut f32,
        alfa: *mut f32,
        beta: *mut f32,
        integr: *mut FortranInteger,
    ) -> f32;
    #[link_name = "r9atn1_"]
    pub fn r9atn1(x: *mut f32) -> f32;
    #[link_name = "r9chu_"]
    pub fn r9chu(a: *mut f32, b: *mut f32, z: *mut f32) -> f32;
    #[link_name = "r9gmic_"]
    pub fn r9gmic(a: *mut f32, x: *mut f32, alx: *mut f32) -> f32;
    #[link_name = "r9gmit_"]
    pub fn r9gmit(
        a: *mut f32,
        x: *mut f32,
        algap1: *mut f32,
        sgngam: *mut f32,
        alx: *mut f32,
    ) -> f32;
    #[link_name = "r9lgic_"]
    pub fn r9lgic(a: *mut f32, x: *mut f32, alx: *mut f32) -> f32;
    #[link_name = "r9lgit_"]
    pub fn r9lgit(a: *mut f32, x: *mut f32, algap1: *mut f32) -> f32;
    #[link_name = "r9lgmc_"]
    pub fn r9lgmc(x: *mut f32) -> f32;
    #[link_name = "r9ln2r_"]
    pub fn r9ln2r(x: *mut f32) -> f32;
    #[link_name = "r9pak_"]
    pub fn r9pak(y: *mut f32, n: *mut FortranInteger) -> f32;
    #[link_name = "sdanrm_"]
    pub fn sdanrm(
        neq: *mut FortranInteger,
        v: *mut f32,
        wt: *mut f32,
        rpar: *mut f32,
        ipar: *mut FortranInteger,
    ) -> f32;
    #[link_name = "sin_"]
    pub fn sin(x: *mut f32) -> f32;
    #[link_name = "sinh_"]
    pub fn sinh(x: *mut f32) -> f32;
    #[link_name = "sqrt_"]
    pub fn sqrt(x: *mut f32) -> f32;
    #[link_name = "tan_"]
    pub fn tan(x: *mut f32) -> f32;
    #[link_name = "tanh_"]
    pub fn tanh(x: *mut f32) -> f32;
    #[link_name = "vnwrms_"]
    pub fn vnwrms(n: *mut FortranInteger, v: *mut f32, w: *mut f32) -> f32;
    #[link_name = "xpsi_"]
    pub fn xpsi(a: *mut f32, ipsik: *mut FortranInteger, ipsix: *mut FortranInteger) -> f32;
    #[link_name = "zabs_"]
    pub fn zabs(zr: *mut f64, zi: *mut f64) -> f64;
}

// ffi-declaration-aliases:start
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dasum`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dasum.md"))]
pub use crate::blas::level1::dasum;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ddot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddot.md"))]
pub use crate::blas::level1::ddot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dnrm2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnrm2.md"))]
pub use crate::blas::level1::dnrm2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dqdota`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqdota.md"))]
pub use crate::blas::level1::dqdota;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dqdoti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqdoti.md"))]
pub use crate::blas::level1::dqdoti;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dsdot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdot.md"))]
pub use crate::blas::level1::dsdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::idamax`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/idamax.md"))]
pub use crate::blas::level1::idamax;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::isamax`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/isamax.md"))]
pub use crate::blas::level1::isamax;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::sasum`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sasum.md"))]
pub use crate::blas::level1::sasum;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::sdot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdot.md"))]
pub use crate::blas::level1::sdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::sdsdot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdsdot.md"))]
pub use crate::blas::level1::sdsdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::snrm2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snrm2.md"))]
pub use crate::blas::level1::snrm2;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bvalu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bvalu.md"))]
pub use crate::interpolation::bvalu;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbvalu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbvalu.md"))]
pub use crate::interpolation::dbvalu;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dppval`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppval.md"))]
pub use crate::interpolation::dppval;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::ppval`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ppval.md"))]
pub use crate::interpolation::ppval;
#[doc = "Transitional ABI-shaped alias; use `crate::special::acosh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/acosh.md"))]
pub use crate::special::acosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::ai`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ai.md"))]
pub use crate::special::airy::ai;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::aie`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/aie.md"))]
pub use crate::special::airy::aie;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::bi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bi.md"))]
pub use crate::special::airy::bi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::bie`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bie.md"))]
pub use crate::special::airy::bie;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::dai`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dai.md"))]
pub use crate::special::airy::dai;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::daie`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/daie.md"))]
pub use crate::special::airy::daie;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::dbi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbi.md"))]
pub use crate::special::airy::dbi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::airy::dbie`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbie.md"))]
pub use crate::special::airy::dbie;
#[doc = "Transitional ABI-shaped alias; use `crate::special::ali`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ali.md"))]
pub use crate::special::ali;
#[doc = "Transitional ABI-shaped alias; use `crate::special::asinh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/asinh.md"))]
pub use crate::special::asinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::atanh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/atanh.md"))]
pub use crate::special::atanh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besi0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi0.md"))]
pub use crate::special::bessel::besi0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besi0e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi0e.md"))]
pub use crate::special::bessel::besi0e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besi1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi1.md"))]
pub use crate::special::bessel::besi1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besi1e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi1e.md"))]
pub use crate::special::bessel::besi1e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besj0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besj0.md"))]
pub use crate::special::bessel::besj0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besj1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besj1.md"))]
pub use crate::special::bessel::besj1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besk0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk0.md"))]
pub use crate::special::bessel::besk0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besk0e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk0e.md"))]
pub use crate::special::bessel::besk0e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besk1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk1.md"))]
pub use crate::special::bessel::besk1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besk1e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk1e.md"))]
pub use crate::special::bessel::besk1e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besy0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besy0.md"))]
pub use crate::special::bessel::besy0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besy1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besy1.md"))]
pub use crate::special::bessel::besy1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesi0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesi0.md"))]
pub use crate::special::bessel::dbesi0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesi1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesi1.md"))]
pub use crate::special::bessel::dbesi1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesj0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesj0.md"))]
pub use crate::special::bessel::dbesj0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesj1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesj1.md"))]
pub use crate::special::bessel::dbesj1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesk0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesk0.md"))]
pub use crate::special::bessel::dbesk0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesk1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesk1.md"))]
pub use crate::special::bessel::dbesk1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesy0`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesy0.md"))]
pub use crate::special::bessel::dbesy0;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesy1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesy1.md"))]
pub use crate::special::bessel::dbesy1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::albeta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/albeta.md"))]
pub use crate::special::beta::albeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::beta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/beta.md"))]
pub use crate::special::beta::beta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::betai`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/betai.md"))]
pub use crate::special::beta::betai;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::dbeta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbeta.md"))]
pub use crate::special::beta::dbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::dbetai`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbetai.md"))]
pub use crate::special::beta::dbetai;
#[doc = "Transitional ABI-shaped alias; use `crate::special::beta::dlbeta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlbeta.md"))]
pub use crate::special::beta::dlbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::chu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chu.md"))]
pub use crate::special::chu;
#[doc = "Transitional ABI-shaped alias; use `crate::special::cot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cot.md"))]
pub use crate::special::cot;
#[doc = "Transitional ABI-shaped alias; use `crate::special::csevl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csevl.md"))]
pub use crate::special::csevl;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dacosh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dacosh.md"))]
pub use crate::special::dacosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dasinh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dasinh.md"))]
pub use crate::special::dasinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::datanh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/datanh.md"))]
pub use crate::special::datanh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbsi0e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsi0e.md"))]
pub use crate::special::dbsi0e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbsi1e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsi1e.md"))]
pub use crate::special::dbsi1e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbsk0e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsk0e.md"))]
pub use crate::special::dbsk0e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbsk1e`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsk1e.md"))]
pub use crate::special::dbsk1e;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dchu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchu.md"))]
pub use crate::special::dchu;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dcot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcot.md"))]
pub use crate::special::dcot;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dcsevl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcsevl.md"))]
pub use crate::special::dcsevl;
#[doc = "Transitional ABI-shaped alias; use `crate::special::de1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/de1.md"))]
pub use crate::special::de1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dei`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dei.md"))]
pub use crate::special::dei;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dli`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dli.md"))]
pub use crate::special::dli;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dpoch`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpoch.md"))]
pub use crate::special::dpoch;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dpoch1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpoch1.md"))]
pub use crate::special::dpoch1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drc.md"))]
pub use crate::special::drc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drd.md"))]
pub use crate::special::drd;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drf.md"))]
pub use crate::special::drf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drj.md"))]
pub use crate::special::drj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dspenc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspenc.md"))]
pub use crate::special::dspenc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::e1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/e1.md"))]
pub use crate::special::e1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::ei`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ei.md"))]
pub use crate::special::ei;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::alnrel`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/alnrel.md"))]
pub use crate::special::elementary::alnrel;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::cbrt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbrt.md"))]
pub use crate::special::elementary::cbrt;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::cosdg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cosdg.md"))]
pub use crate::special::elementary::cosdg;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::daws`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/daws.md"))]
pub use crate::special::elementary::daws;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::dcbrt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcbrt.md"))]
pub use crate::special::elementary::dcbrt;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::dcosdg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcosdg.md"))]
pub use crate::special::elementary::dcosdg;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::ddaws`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddaws.md"))]
pub use crate::special::elementary::ddaws;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::dexprl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dexprl.md"))]
pub use crate::special::elementary::dexprl;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::dlnrel`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlnrel.md"))]
pub use crate::special::elementary::dlnrel;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::dsindg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsindg.md"))]
pub use crate::special::elementary::dsindg;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::exprel`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/exprel.md"))]
pub use crate::special::elementary::exprel;
#[doc = "Transitional ABI-shaped alias; use `crate::special::elementary::sindg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sindg.md"))]
pub use crate::special::elementary::sindg;
#[doc = "Transitional ABI-shaped alias; use `crate::special::error::derf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/derf.md"))]
pub use crate::special::error::derf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::error::derfc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/derfc.md"))]
pub use crate::special::error::derfc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::error::erf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/erf.md"))]
pub use crate::special::error::erf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::error::erfc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/erfc.md"))]
pub use crate::special::error::erfc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::alngam`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/alngam.md"))]
pub use crate::special::gamma::alngam;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::binom`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/binom.md"))]
pub use crate::special::gamma::binom;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dbinom`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbinom.md"))]
pub use crate::special::gamma::dbinom;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dfac`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dfac.md"))]
pub use crate::special::gamma::dfac;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dgami`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgami.md"))]
pub use crate::special::gamma::dgami;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dgamic`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamic.md"))]
pub use crate::special::gamma::dgamic;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dgamit`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamit.md"))]
pub use crate::special::gamma::dgamit;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dgamma`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamma.md"))]
pub use crate::special::gamma::dgamma;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dgamr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgamr.md"))]
pub use crate::special::gamma::dgamr;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dlngam`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlngam.md"))]
pub use crate::special::gamma::dlngam;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::dpsi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpsi.md"))]
pub use crate::special::gamma::dpsi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::fac`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/fac.md"))]
pub use crate::special::gamma::fac;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::gami`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gami.md"))]
pub use crate::special::gamma::gami;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::gamic`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamic.md"))]
pub use crate::special::gamma::gamic;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::gamit`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamit.md"))]
pub use crate::special::gamma::gamit;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::gamma`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamma.md"))]
pub use crate::special::gamma::gamma;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::gamr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/gamr.md"))]
pub use crate::special::gamma::gamr;
#[doc = "Transitional ABI-shaped alias; use `crate::special::gamma::psi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/psi.md"))]
pub use crate::special::gamma::psi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::initds`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/initds.md"))]
pub use crate::special::initds;
#[doc = "Transitional ABI-shaped alias; use `crate::special::inits`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/inits.md"))]
pub use crate::special::inits;
#[doc = "Transitional ABI-shaped alias; use `crate::special::poch`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/poch.md"))]
pub use crate::special::poch;
#[doc = "Transitional ABI-shaped alias; use `crate::special::poch1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/poch1.md"))]
pub use crate::special::poch1;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rc.md"))]
pub use crate::special::rc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rd.md"))]
pub use crate::special::rd;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rf.md"))]
pub use crate::special::rf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rj.md"))]
pub use crate::special::rj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::spenc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spenc.md"))]
pub use crate::special::spenc;
#[doc = "Transitional ABI-shaped alias; use `crate::statistics::cv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cv.md"))]
pub use crate::statistics::cv;
#[doc = "Transitional ABI-shaped alias; use `crate::statistics::dcv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcv.md"))]
pub use crate::statistics::dcv;
#[doc = "Transitional ABI-shaped alias; use `crate::statistics::rand`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rand.md"))]
pub use crate::statistics::rand;
#[doc = "Transitional ABI-shaped alias; use `crate::statistics::rgauss`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rgauss.md"))]
pub use crate::statistics::rgauss;
#[doc = "Transitional ABI-shaped alias; use `crate::statistics::runif`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/runif.md"))]
pub use crate::statistics::runif;
// ffi-declaration-aliases:end
