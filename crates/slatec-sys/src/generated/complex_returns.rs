//! Generated raw declarations for `batch_complex_returns`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "c9lgmc_"]
    pub fn c9lgmc(zin: *mut Complex32) -> Complex32;
    #[link_name = "c9ln2r_"]
    pub fn c9ln2r(z: *mut Complex32) -> Complex32;
    #[link_name = "ccos_"]
    pub fn ccos(z: *mut Complex32) -> Complex32;
    #[link_name = "cexp_"]
    pub fn cexp(z: *mut Complex32) -> Complex32;
    #[link_name = "clog_"]
    pub fn clog(z: *mut Complex32) -> Complex32;
    #[link_name = "csin_"]
    pub fn csin(z: *mut Complex32) -> Complex32;
    #[link_name = "csqrt_"]
    pub fn csqrt(z: *mut Complex32) -> Complex32;
}

// ffi-declaration-aliases:start
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cdcdot`."]
pub use crate::blas::level1::cdcdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cdotc`."]
pub use crate::blas::level1::cdotc;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cdotu`."]
pub use crate::blas::level1::cdotu;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::c0lgmc`."]
pub use crate::special::complex::c0lgmc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cacos`."]
pub use crate::special::complex::cacos;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cacosh`."]
pub use crate::special::complex::cacosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::casin`."]
pub use crate::special::complex::casin;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::casinh`."]
pub use crate::special::complex::casinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catan`."]
pub use crate::special::complex::catan;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catan2`."]
pub use crate::special::complex::catan2;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catanh`."]
pub use crate::special::complex::catanh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbeta`."]
pub use crate::special::complex::cbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccbrt`."]
pub use crate::special::complex::ccbrt;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccosh`."]
pub use crate::special::complex::ccosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccot`."]
pub use crate::special::complex::ccot;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cexprl`."]
pub use crate::special::complex::cexprl;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cgamma`."]
pub use crate::special::complex::cgamma;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cgamr`."]
pub use crate::special::complex::cgamr;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clbeta`."]
pub use crate::special::complex::clbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clngam`."]
pub use crate::special::complex::clngam;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clnrel`."]
pub use crate::special::complex::clnrel;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clog10`."]
pub use crate::special::complex::clog10;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cpsi`."]
pub use crate::special::complex::cpsi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::csinh`."]
pub use crate::special::complex::csinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ctan`."]
pub use crate::special::complex::ctan;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ctanh`."]
pub use crate::special::complex::ctanh;
// ffi-declaration-aliases:end
