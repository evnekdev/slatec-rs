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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgbmv.md"))]
pub use crate::blas::level2::cgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgemv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgemv.md"))]
pub use crate::blas::level2::cgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chbmv.md"))]
pub use crate::blas::level2::chbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chemv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chemv.md"))]
pub use crate::blas::level2::chemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cher`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cher.md"))]
pub use crate::blas::level2::cher;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cher2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cher2.md"))]
pub use crate::blas::level2::cher2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpmv.md"))]
pub use crate::blas::level2::chpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpr.md"))]
pub use crate::blas::level2::chpr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::chpr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpr2.md"))]
pub use crate::blas::level2::chpr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctbmv.md"))]
pub use crate::blas::level2::ctbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctbsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctbsv.md"))]
pub use crate::blas::level2::ctbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctpmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctpmv.md"))]
pub use crate::blas::level2::ctpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctpsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctpsv.md"))]
pub use crate::blas::level2::ctpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctrmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrmv.md"))]
pub use crate::blas::level2::ctrmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ctrsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrsv.md"))]
pub use crate::blas::level2::ctrsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dgbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbmv.md"))]
pub use crate::blas::level2::dgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dgemv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgemv.md"))]
pub use crate::blas::level2::dgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsbmv.md"))]
pub use crate::blas::level2::dsbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspmv.md"))]
pub use crate::blas::level2::dspmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspr.md"))]
pub use crate::blas::level2::dspr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dspr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspr2.md"))]
pub use crate::blas::level2::dspr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsymv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsymv.md"))]
pub use crate::blas::level2::dsymv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsyr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsyr.md"))]
pub use crate::blas::level2::dsyr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dsyr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsyr2.md"))]
pub use crate::blas::level2::dsyr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtbmv.md"))]
pub use crate::blas::level2::dtbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtbsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtbsv.md"))]
pub use crate::blas::level2::dtbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtpmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtpmv.md"))]
pub use crate::blas::level2::dtpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtpsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtpsv.md"))]
pub use crate::blas::level2::dtpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtrmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrmv.md"))]
pub use crate::blas::level2::dtrmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dtrsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrsv.md"))]
pub use crate::blas::level2::dtrsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sgbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbmv.md"))]
pub use crate::blas::level2::sgbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sgemv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgemv.md"))]
pub use crate::blas::level2::sgemv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssbmv.md"))]
pub use crate::blas::level2::ssbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspmv.md"))]
pub use crate::blas::level2::sspmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspr.md"))]
pub use crate::blas::level2::sspr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sspr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspr2.md"))]
pub use crate::blas::level2::sspr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssymv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssymv.md"))]
pub use crate::blas::level2::ssymv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssyr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssyr.md"))]
pub use crate::blas::level2::ssyr;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::ssyr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssyr2.md"))]
pub use crate::blas::level2::ssyr2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stbmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/stbmv.md"))]
pub use crate::blas::level2::stbmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stbsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/stbsv.md"))]
pub use crate::blas::level2::stbsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stpmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/stpmv.md"))]
pub use crate::blas::level2::stpmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::stpsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/stpsv.md"))]
pub use crate::blas::level2::stpsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::strmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strmv.md"))]
pub use crate::blas::level2::strmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::strsv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strsv.md"))]
pub use crate::blas::level2::strsv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cgemm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgemm.md"))]
pub use crate::blas::level3::cgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::chemm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chemm.md"))]
pub use crate::blas::level3::chemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cher2k`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cher2k.md"))]
pub use crate::blas::level3::cher2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::cherk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cherk.md"))]
pub use crate::blas::level3::cherk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csymm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csymm.md"))]
pub use crate::blas::level3::csymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csyr2k`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csyr2k.md"))]
pub use crate::blas::level3::csyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::csyrk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csyrk.md"))]
pub use crate::blas::level3::csyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ctrmm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrmm.md"))]
pub use crate::blas::level3::ctrmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ctrsm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrsm.md"))]
pub use crate::blas::level3::ctrsm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dgemm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgemm.md"))]
pub use crate::blas::level3::dgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsymm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsymm.md"))]
pub use crate::blas::level3::dsymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsyr2k`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsyr2k.md"))]
pub use crate::blas::level3::dsyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dsyrk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsyrk.md"))]
pub use crate::blas::level3::dsyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dtrmm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrmm.md"))]
pub use crate::blas::level3::dtrmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::dtrsm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrsm.md"))]
pub use crate::blas::level3::dtrsm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::sgemm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgemm.md"))]
pub use crate::blas::level3::sgemm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssymm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssymm.md"))]
pub use crate::blas::level3::ssymm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssyr2k`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssyr2k.md"))]
pub use crate::blas::level3::ssyr2k;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::ssyrk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssyrk.md"))]
pub use crate::blas::level3::ssyrk;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::strmm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strmm.md"))]
pub use crate::blas::level3::strmm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level3::strsm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strsm.md"))]
pub use crate::blas::level3::strsm;
// ffi-declaration-aliases:end
