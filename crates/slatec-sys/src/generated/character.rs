//! Generated raw declarations for `batch_character`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "aaaaaa_"]
    pub fn aaaaaa(ver: *mut c_char, character_length_1: FortranCharacterLength);
    #[link_name = "hpperm_"]
    pub fn hpperm(
        hx: *mut c_char,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        work: *mut c_char,
        ier: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "hpsort_"]
    pub fn hpsort(
        hx: *mut c_char,
        n: *mut FortranInteger,
        strbeg: *mut FortranInteger,
        strend: *mut FortranInteger,
        iperm: *mut FortranInteger,
        kflag: *mut FortranInteger,
        work: *mut c_char,
        ier: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
}

// ffi-declaration-aliases:start
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgbmv`."]
pub use crate::blas::level2::cgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgemv`."]
pub use crate::blas::level2::cgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chbmv`."]
pub use crate::blas::level2::chbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chemv`."]
pub use crate::blas::level2::chemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cher`."]
pub use crate::blas::level2::cher;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cher2`."]
pub use crate::blas::level2::cher2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpmv`."]
pub use crate::blas::level2::chpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpr`."]
pub use crate::blas::level2::chpr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpr2`."]
pub use crate::blas::level2::chpr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctbmv`."]
pub use crate::blas::level2::ctbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctbsv`."]
pub use crate::blas::level2::ctbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctpmv`."]
pub use crate::blas::level2::ctpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctpsv`."]
pub use crate::blas::level2::ctpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctrmv`."]
pub use crate::blas::level2::ctrmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctrsv`."]
pub use crate::blas::level2::ctrsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dgbmv`."]
pub use crate::blas::level2::dgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dgemv`."]
pub use crate::blas::level2::dgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsbmv`."]
pub use crate::blas::level2::dsbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspmv`."]
pub use crate::blas::level2::dspmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspr`."]
pub use crate::blas::level2::dspr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspr2`."]
pub use crate::blas::level2::dspr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsymv`."]
pub use crate::blas::level2::dsymv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsyr`."]
pub use crate::blas::level2::dsyr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsyr2`."]
pub use crate::blas::level2::dsyr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtbmv`."]
pub use crate::blas::level2::dtbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtbsv`."]
pub use crate::blas::level2::dtbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtpmv`."]
pub use crate::blas::level2::dtpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtpsv`."]
pub use crate::blas::level2::dtpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtrmv`."]
pub use crate::blas::level2::dtrmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtrsv`."]
pub use crate::blas::level2::dtrsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sgbmv`."]
pub use crate::blas::level2::sgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sgemv`."]
pub use crate::blas::level2::sgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssbmv`."]
pub use crate::blas::level2::ssbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspmv`."]
pub use crate::blas::level2::sspmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspr`."]
pub use crate::blas::level2::sspr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspr2`."]
pub use crate::blas::level2::sspr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssymv`."]
pub use crate::blas::level2::ssymv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssyr`."]
pub use crate::blas::level2::ssyr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssyr2`."]
pub use crate::blas::level2::ssyr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stbmv`."]
pub use crate::blas::level2::stbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stbsv`."]
pub use crate::blas::level2::stbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stpmv`."]
pub use crate::blas::level2::stpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stpsv`."]
pub use crate::blas::level2::stpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::strmv`."]
pub use crate::blas::level2::strmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::strsv`."]
pub use crate::blas::level2::strsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cgemm`."]
pub use crate::blas::level3::cgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::chemm`."]
pub use crate::blas::level3::chemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cher2k`."]
pub use crate::blas::level3::cher2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cherk`."]
pub use crate::blas::level3::cherk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csymm`."]
pub use crate::blas::level3::csymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csyr2k`."]
pub use crate::blas::level3::csyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csyrk`."]
pub use crate::blas::level3::csyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ctrmm`."]
pub use crate::blas::level3::ctrmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ctrsm`."]
pub use crate::blas::level3::ctrsm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dgemm`."]
pub use crate::blas::level3::dgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsymm`."]
pub use crate::blas::level3::dsymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsyr2k`."]
pub use crate::blas::level3::dsyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsyrk`."]
pub use crate::blas::level3::dsyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dtrmm`."]
pub use crate::blas::level3::dtrmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dtrsm`."]
pub use crate::blas::level3::dtrsm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::sgemm`."]
pub use crate::blas::level3::sgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssymm`."]
pub use crate::blas::level3::ssymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssyr2k`."]
pub use crate::blas::level3::ssyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssyrk`."]
pub use crate::blas::level3::ssyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::strmm`."]
pub use crate::blas::level3::strmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::strsm`."]
pub use crate::blas::level3::strsm;
// ffi-declaration-aliases:end
