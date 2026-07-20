//! Generated raw declarations for `batch_complex_returns`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "c0lgmc_"]
    pub fn c0lgmc(z: *mut Complex32) -> Complex32;
    #[link_name = "c9lgmc_"]
    pub fn c9lgmc(zin: *mut Complex32) -> Complex32;
    #[link_name = "c9ln2r_"]
    pub fn c9ln2r(z: *mut Complex32) -> Complex32;
    #[link_name = "cacos_"]
    pub fn cacos(z: *mut Complex32) -> Complex32;
    #[link_name = "cacosh_"]
    pub fn cacosh(z: *mut Complex32) -> Complex32;
    #[link_name = "casin_"]
    pub fn casin(zinp: *mut Complex32) -> Complex32;
    #[link_name = "casinh_"]
    pub fn casinh(z: *mut Complex32) -> Complex32;
    #[link_name = "catan_"]
    pub fn catan(z: *mut Complex32) -> Complex32;
    #[link_name = "catan2_"]
    pub fn catan2(csn: *mut Complex32, ccs: *mut Complex32) -> Complex32;
    #[link_name = "catanh_"]
    pub fn catanh(z: *mut Complex32) -> Complex32;
    #[link_name = "cbeta_"]
    pub fn cbeta(a: *mut Complex32, b: *mut Complex32) -> Complex32;
    #[link_name = "ccbrt_"]
    pub fn ccbrt(z: *mut Complex32) -> Complex32;
    #[link_name = "ccos_"]
    pub fn ccos(z: *mut Complex32) -> Complex32;
    #[link_name = "ccosh_"]
    pub fn ccosh(z: *mut Complex32) -> Complex32;
    #[link_name = "ccot_"]
    pub fn ccot(z: *mut Complex32) -> Complex32;
    #[link_name = "cdcdot_"]
    pub fn cdcdot(
        n: *mut FortranInteger,
        cb: *mut Complex32,
        cx: *mut Complex32,
        incx: *mut FortranInteger,
        cy: *mut Complex32,
        incy: *mut FortranInteger,
    ) -> Complex32;
    #[link_name = "cdotc_"]
    pub fn cdotc(
        n: *mut FortranInteger,
        cx: *mut Complex32,
        incx: *mut FortranInteger,
        cy: *mut Complex32,
        incy: *mut FortranInteger,
    ) -> Complex32;
    #[link_name = "cdotu_"]
    pub fn cdotu(
        n: *mut FortranInteger,
        cx: *mut Complex32,
        incx: *mut FortranInteger,
        cy: *mut Complex32,
        incy: *mut FortranInteger,
    ) -> Complex32;
    #[link_name = "cexp_"]
    pub fn cexp(z: *mut Complex32) -> Complex32;
    #[link_name = "cexprl_"]
    pub fn cexprl(z: *mut Complex32) -> Complex32;
    #[link_name = "cgamma_"]
    pub fn cgamma(z: *mut Complex32) -> Complex32;
    #[link_name = "cgamr_"]
    pub fn cgamr(z: *mut Complex32) -> Complex32;
    #[link_name = "clbeta_"]
    pub fn clbeta(a: *mut Complex32, b: *mut Complex32) -> Complex32;
    #[link_name = "clngam_"]
    pub fn clngam(zin: *mut Complex32) -> Complex32;
    #[link_name = "clnrel_"]
    pub fn clnrel(z: *mut Complex32) -> Complex32;
    #[link_name = "clog_"]
    pub fn clog(z: *mut Complex32) -> Complex32;
    #[link_name = "clog10_"]
    pub fn clog10(z: *mut Complex32) -> Complex32;
    #[link_name = "cpsi_"]
    pub fn cpsi(zin: *mut Complex32) -> Complex32;
    #[link_name = "csin_"]
    pub fn csin(z: *mut Complex32) -> Complex32;
    #[link_name = "csinh_"]
    pub fn csinh(z: *mut Complex32) -> Complex32;
    #[link_name = "csqrt_"]
    pub fn csqrt(z: *mut Complex32) -> Complex32;
    #[link_name = "ctan_"]
    pub fn ctan(z: *mut Complex32) -> Complex32;
    #[link_name = "ctanh_"]
    pub fn ctanh(z: *mut Complex32) -> Complex32;
}
