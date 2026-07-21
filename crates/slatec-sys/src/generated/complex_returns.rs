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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cdcdot.md"))]
pub use crate::blas::level1::cdcdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cdotc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cdotc.md"))]
pub use crate::blas::level1::cdotc;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cdotu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cdotu.md"))]
pub use crate::blas::level1::cdotu;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::c0lgmc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/c0lgmc.md"))]
pub use crate::special::complex::c0lgmc;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cacos`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cacos.md"))]
pub use crate::special::complex::cacos;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cacosh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cacosh.md"))]
pub use crate::special::complex::cacosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::casin`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/casin.md"))]
pub use crate::special::complex::casin;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::casinh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/casinh.md"))]
pub use crate::special::complex::casinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catan`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/catan.md"))]
pub use crate::special::complex::catan;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catan2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/catan2.md"))]
pub use crate::special::complex::catan2;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::catanh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/catanh.md"))]
pub use crate::special::complex::catanh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbeta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbeta.md"))]
pub use crate::special::complex::cbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccbrt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ccbrt.md"))]
pub use crate::special::complex::ccbrt;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccosh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ccosh.md"))]
pub use crate::special::complex::ccosh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ccot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ccot.md"))]
pub use crate::special::complex::ccot;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cexprl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cexprl.md"))]
pub use crate::special::complex::cexprl;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cgamma`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgamma.md"))]
pub use crate::special::complex::cgamma;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cgamr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgamr.md"))]
pub use crate::special::complex::cgamr;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clbeta`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/clbeta.md"))]
pub use crate::special::complex::clbeta;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clngam`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/clngam.md"))]
pub use crate::special::complex::clngam;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clnrel`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/clnrel.md"))]
pub use crate::special::complex::clnrel;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::clog10`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/clog10.md"))]
pub use crate::special::complex::clog10;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cpsi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpsi.md"))]
pub use crate::special::complex::cpsi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::csinh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csinh.md"))]
pub use crate::special::complex::csinh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ctan`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctan.md"))]
pub use crate::special::complex::ctan;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::ctanh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctanh.md"))]
pub use crate::special::complex::ctanh;
// ffi-declaration-aliases:end
