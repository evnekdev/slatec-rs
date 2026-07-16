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
    #[link_name = "cgbmv_"]
    pub fn cgbmv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        kl: *mut FortranInteger,
        ku: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        beta: *mut Complex32,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "cgemm_"]
    pub fn cgemm(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        beta: *mut Complex32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "cgemv_"]
    pub fn cgemv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        beta: *mut Complex32,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "chbmv_"]
    pub fn chbmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        beta: *mut Complex32,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "chemm_"]
    pub fn chemm(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        beta: *mut Complex32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "chemv_"]
    pub fn chemv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        beta: *mut Complex32,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "cher_"]
    pub fn cher(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "cher2_"]
    pub fn cher2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "cher2k_"]
    pub fn cher2k(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        beta: *mut f32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "cherk_"]
    pub fn cherk(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        beta: *mut f32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "chpmv_"]
    pub fn chpmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        ap: *mut Complex32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        beta: *mut Complex32,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "chpr_"]
    pub fn chpr(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        ap: *mut Complex32,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "chpr2_"]
    pub fn chpr2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        y: *mut Complex32,
        incy: *mut FortranInteger,
        ap: *mut Complex32,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "csymm_"]
    pub fn csymm(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        beta: *mut Complex32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "csyr2k_"]
    pub fn csyr2k(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        beta: *mut Complex32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "csyrk_"]
    pub fn csyrk(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        beta: *mut Complex32,
        c: *mut Complex32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "ctbmv_"]
    pub fn ctbmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "ctbsv_"]
    pub fn ctbsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "ctpmv_"]
    pub fn ctpmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut Complex32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "ctpsv_"]
    pub fn ctpsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut Complex32,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "ctrmm_"]
    pub fn ctrmm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "ctrmv_"]
    pub fn ctrmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "ctrsm_"]
    pub fn ctrsm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut Complex32,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        b: *mut Complex32,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "ctrsv_"]
    pub fn ctrsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut Complex32,
        lda: *mut FortranInteger,
        x: *mut Complex32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dgbmv_"]
    pub fn dgbmv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        kl: *mut FortranInteger,
        ku: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dgemm_"]
    pub fn dgemm(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        b: *mut f64,
        ldb: *mut FortranInteger,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "dgemv_"]
    pub fn dgemv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dsbmv_"]
    pub fn dsbmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dspmv_"]
    pub fn dspmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        ap: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dspr_"]
    pub fn dspr(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        ap: *mut f64,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dspr2_"]
    pub fn dspr2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        y: *mut f64,
        incy: *mut FortranInteger,
        ap: *mut f64,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dsymm_"]
    pub fn dsymm(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        b: *mut f64,
        ldb: *mut FortranInteger,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "dsymv_"]
    pub fn dsymv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dsyr_"]
    pub fn dsyr(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dsyr2_"]
    pub fn dsyr2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        y: *mut f64,
        incy: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "dsyr2k_"]
    pub fn dsyr2k(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        b: *mut f64,
        ldb: *mut FortranInteger,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "dsyrk_"]
    pub fn dsyrk(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "dtbmv_"]
    pub fn dtbmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dtbsv_"]
    pub fn dtbsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dtpmv_"]
    pub fn dtpmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dtpsv_"]
    pub fn dtpsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut f64,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dtrmm_"]
    pub fn dtrmm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        b: *mut f64,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "dtrmv_"]
    pub fn dtrmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "dtrsm_"]
    pub fn dtrsm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut FortranInteger,
        b: *mut f64,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "dtrsv_"]
    pub fn dtrsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        x: *mut f64,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
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
    #[link_name = "sgbmv_"]
    pub fn sgbmv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        kl: *mut FortranInteger,
        ku: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "sgemm_"]
    pub fn sgemm(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        b: *mut f32,
        ldb: *mut FortranInteger,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "sgemv_"]
    pub fn sgemv(
        trans: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "ssbmv_"]
    pub fn ssbmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "sspmv_"]
    pub fn sspmv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        ap: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "sspr_"]
    pub fn sspr(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        ap: *mut f32,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "sspr2_"]
    pub fn sspr2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        y: *mut f32,
        incy: *mut FortranInteger,
        ap: *mut f32,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "ssymm_"]
    pub fn ssymm(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        b: *mut f32,
        ldb: *mut FortranInteger,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "ssymv_"]
    pub fn ssymv(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "ssyr_"]
    pub fn ssyr(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "ssyr2_"]
    pub fn ssyr2(
        uplo: *mut c_char,
        n: *mut FortranInteger,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        y: *mut f32,
        incy: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
    );
    #[link_name = "ssyr2k_"]
    pub fn ssyr2k(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        b: *mut f32,
        ldb: *mut FortranInteger,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "ssyrk_"]
    pub fn ssyrk(
        uplo: *mut c_char,
        trans: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
    );
    #[link_name = "stbmv_"]
    pub fn stbmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "stbsv_"]
    pub fn stbsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "stpmv_"]
    pub fn stpmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "stpsv_"]
    pub fn stpsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        ap: *mut f32,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "strmm_"]
    pub fn strmm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        b: *mut f32,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "strmv_"]
    pub fn strmv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
    #[link_name = "strsm_"]
    pub fn strsm(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut FortranInteger,
        b: *mut f32,
        ldb: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
        character_length_4: FortranCharacterLength,
    );
    #[link_name = "strsv_"]
    pub fn strsv(
        uplo: *mut c_char,
        trans: *mut c_char,
        diag: *mut c_char,
        n: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        x: *mut f32,
        incx: *mut FortranInteger,
        character_length_1: FortranCharacterLength,
        character_length_2: FortranCharacterLength,
        character_length_3: FortranCharacterLength,
    );
}
