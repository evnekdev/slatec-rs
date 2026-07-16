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
    #[link_name = "acosh_"]
    pub fn acosh(x: *mut f32) -> f32;
    #[link_name = "ai_"]
    pub fn ai(x: *mut f32) -> f32;
    #[link_name = "aie_"]
    pub fn aie(x: *mut f32) -> f32;
    #[link_name = "albeta_"]
    pub fn albeta(a: *mut f32, b: *mut f32) -> f32;
    #[link_name = "ali_"]
    pub fn ali(x: *mut f32) -> f32;
    #[link_name = "alngam_"]
    pub fn alngam(x: *mut f32) -> f32;
    #[link_name = "alnrel_"]
    pub fn alnrel(x: *mut f32) -> f32;
    #[link_name = "alog_"]
    pub fn alog(x: *mut f32) -> f32;
    #[link_name = "alog10_"]
    pub fn alog10(x: *mut f32) -> f32;
    #[link_name = "asin_"]
    pub fn asin(x: *mut f32) -> f32;
    #[link_name = "asinh_"]
    pub fn asinh(x: *mut f32) -> f32;
    #[link_name = "atan_"]
    pub fn atan(x: *mut f32) -> f32;
    #[link_name = "atan2_"]
    pub fn atan2(sn: *mut f32, cs: *mut f32) -> f32;
    #[link_name = "atanh_"]
    pub fn atanh(x: *mut f32) -> f32;
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
    #[link_name = "besi0_"]
    pub fn besi0(x: *mut f32) -> f32;
    #[link_name = "besi0e_"]
    pub fn besi0e(x: *mut f32) -> f32;
    #[link_name = "besi1_"]
    pub fn besi1(x: *mut f32) -> f32;
    #[link_name = "besi1e_"]
    pub fn besi1e(x: *mut f32) -> f32;
    #[link_name = "besj0_"]
    pub fn besj0(x: *mut f32) -> f32;
    #[link_name = "besj1_"]
    pub fn besj1(x: *mut f32) -> f32;
    #[link_name = "besk0_"]
    pub fn besk0(x: *mut f32) -> f32;
    #[link_name = "besk0e_"]
    pub fn besk0e(x: *mut f32) -> f32;
    #[link_name = "besk1_"]
    pub fn besk1(x: *mut f32) -> f32;
    #[link_name = "besk1e_"]
    pub fn besk1e(x: *mut f32) -> f32;
    #[link_name = "besy0_"]
    pub fn besy0(x: *mut f32) -> f32;
    #[link_name = "besy1_"]
    pub fn besy1(x: *mut f32) -> f32;
    #[link_name = "beta_"]
    pub fn beta(a: *mut f32, b: *mut f32) -> f32;
    #[link_name = "betai_"]
    pub fn betai(x: *mut f32, pin: *mut f32, qin: *mut f32) -> f32;
    #[link_name = "bi_"]
    pub fn bi(x: *mut f32) -> f32;
    #[link_name = "bie_"]
    pub fn bie(x: *mut f32) -> f32;
    #[link_name = "binom_"]
    pub fn binom(n: *mut FortranInteger, m: *mut FortranInteger) -> f32;
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
    #[link_name = "bvalu_"]
    pub fn bvalu(
        t: *mut f32,
        a: *mut f32,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        ideriv: *mut FortranInteger,
        x: *mut f32,
        inbv: *mut FortranInteger,
        work: *mut f32,
    ) -> f32;
    #[link_name = "cbrt_"]
    pub fn cbrt(x: *mut f32) -> f32;
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
    #[link_name = "chu_"]
    pub fn chu(a: *mut f32, b: *mut f32, x: *mut f32) -> f32;
    #[link_name = "cos_"]
    pub fn cos(x: *mut f32) -> f32;
    #[link_name = "cosdg_"]
    pub fn cosdg(x: *mut f32) -> f32;
    #[link_name = "cosh_"]
    pub fn cosh(x: *mut f32) -> f32;
    #[link_name = "cot_"]
    pub fn cot(x: *mut f32) -> f32;
    #[link_name = "csevl_"]
    pub fn csevl(x: *mut f32, cs: *mut f32, n: *mut FortranInteger) -> f32;
    #[link_name = "cv_"]
    pub fn cv(
        xval: *mut f32,
        ndata: *mut FortranInteger,
        nconst: *mut FortranInteger,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkpt: *mut f32,
        w: *mut f32,
    ) -> f32;
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
    #[link_name = "dacosh_"]
    pub fn dacosh(x: *mut f64) -> f64;
    #[link_name = "dai_"]
    pub fn dai(x: *mut f64) -> f64;
    #[link_name = "daie_"]
    pub fn daie(x: *mut f64) -> f64;
    #[link_name = "dasin_"]
    pub fn dasin(x: *mut f64) -> f64;
    #[link_name = "dasinh_"]
    pub fn dasinh(x: *mut f64) -> f64;
    #[link_name = "dasum_"]
    pub fn dasum(n: *mut FortranInteger, dx: *mut f64, incx: *mut FortranInteger) -> f64;
    #[link_name = "datan_"]
    pub fn datan(x: *mut f64) -> f64;
    #[link_name = "datan2_"]
    pub fn datan2(sn: *mut f64, cs: *mut f64) -> f64;
    #[link_name = "datanh_"]
    pub fn datanh(x: *mut f64) -> f64;
    #[link_name = "daws_"]
    pub fn daws(x: *mut f32) -> f32;
    #[link_name = "dbesi0_"]
    pub fn dbesi0(x: *mut f64) -> f64;
    #[link_name = "dbesi1_"]
    pub fn dbesi1(x: *mut f64) -> f64;
    #[link_name = "dbesj0_"]
    pub fn dbesj0(x: *mut f64) -> f64;
    #[link_name = "dbesj1_"]
    pub fn dbesj1(x: *mut f64) -> f64;
    #[link_name = "dbesk0_"]
    pub fn dbesk0(x: *mut f64) -> f64;
    #[link_name = "dbesk1_"]
    pub fn dbesk1(x: *mut f64) -> f64;
    #[link_name = "dbesy0_"]
    pub fn dbesy0(x: *mut f64) -> f64;
    #[link_name = "dbesy1_"]
    pub fn dbesy1(x: *mut f64) -> f64;
    #[link_name = "dbeta_"]
    pub fn dbeta(a: *mut f64, b: *mut f64) -> f64;
    #[link_name = "dbetai_"]
    pub fn dbetai(x: *mut f64, pin: *mut f64, qin: *mut f64) -> f64;
    #[link_name = "dbi_"]
    pub fn dbi(x: *mut f64) -> f64;
    #[link_name = "dbie_"]
    pub fn dbie(x: *mut f64) -> f64;
    #[link_name = "dbinom_"]
    pub fn dbinom(n: *mut FortranInteger, m: *mut FortranInteger) -> f64;
    #[link_name = "dbsi0e_"]
    pub fn dbsi0e(x: *mut f64) -> f64;
    #[link_name = "dbsi1e_"]
    pub fn dbsi1e(x: *mut f64) -> f64;
    #[link_name = "dbsk0e_"]
    pub fn dbsk0e(x: *mut f64) -> f64;
    #[link_name = "dbsk1e_"]
    pub fn dbsk1e(x: *mut f64) -> f64;
    #[link_name = "dbvalu_"]
    pub fn dbvalu(
        t: *mut f64,
        a: *mut f64,
        n: *mut FortranInteger,
        k: *mut FortranInteger,
        ideriv: *mut FortranInteger,
        x: *mut f64,
        inbv: *mut FortranInteger,
        work: *mut f64,
    ) -> f64;
    #[link_name = "dcbrt_"]
    pub fn dcbrt(x: *mut f64) -> f64;
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
    #[link_name = "dchu_"]
    pub fn dchu(a: *mut f64, b: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dcos_"]
    pub fn dcos(x: *mut f64) -> f64;
    #[link_name = "dcosdg_"]
    pub fn dcosdg(x: *mut f64) -> f64;
    #[link_name = "dcosh_"]
    pub fn dcosh(x: *mut f64) -> f64;
    #[link_name = "dcot_"]
    pub fn dcot(x: *mut f64) -> f64;
    #[link_name = "dcsevl_"]
    pub fn dcsevl(x: *mut f64, cs: *mut f64, n: *mut FortranInteger) -> f64;
    #[link_name = "dcv_"]
    pub fn dcv(
        xval: *mut f64,
        ndata: *mut FortranInteger,
        nconst: *mut FortranInteger,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkpt: *mut f64,
        w: *mut f64,
    ) -> f64;
    #[link_name = "ddanrm_"]
    pub fn ddanrm(
        neq: *mut FortranInteger,
        v: *mut f64,
        wt: *mut f64,
        rpar: *mut f64,
        ipar: *mut FortranInteger,
    ) -> f64;
    #[link_name = "ddaws_"]
    pub fn ddaws(x: *mut f64) -> f64;
    #[link_name = "ddot_"]
    pub fn ddot(
        n: *mut FortranInteger,
        dx: *mut f64,
        incx: *mut FortranInteger,
        dy: *mut f64,
        incy: *mut FortranInteger,
    ) -> f64;
    #[link_name = "de1_"]
    pub fn de1(x: *mut f64) -> f64;
    #[link_name = "dei_"]
    pub fn dei(x: *mut f64) -> f64;
    #[link_name = "denorm_"]
    pub fn denorm(n: *mut FortranInteger, x: *mut f64) -> f64;
    #[link_name = "derf_"]
    pub fn derf(x: *mut f64) -> f64;
    #[link_name = "derfc_"]
    pub fn derfc(x: *mut f64) -> f64;
    #[link_name = "dexp_"]
    pub fn dexp(x: *mut f64) -> f64;
    #[link_name = "dexprl_"]
    pub fn dexprl(x: *mut f64) -> f64;
    #[link_name = "dfac_"]
    pub fn dfac(n: *mut FortranInteger) -> f64;
    #[link_name = "dgami_"]
    pub fn dgami(a: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dgamic_"]
    pub fn dgamic(a: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dgamit_"]
    pub fn dgamit(a: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dgamln_"]
    pub fn dgamln(z: *mut f64, ierr: *mut FortranInteger) -> f64;
    #[link_name = "dgamma_"]
    pub fn dgamma(x: *mut f64) -> f64;
    #[link_name = "dgamr_"]
    pub fn dgamr(x: *mut f64) -> f64;
    #[link_name = "dgamrn_"]
    pub fn dgamrn(x: *mut f64) -> f64;
    #[link_name = "dhvnrm_"]
    pub fn dhvnrm(v: *mut f64, ncomp: *mut FortranInteger) -> f64;
    #[link_name = "dint_"]
    pub fn dint(x: *mut f64) -> f64;
    #[link_name = "dlbeta_"]
    pub fn dlbeta(a: *mut f64, b: *mut f64) -> f64;
    #[link_name = "dli_"]
    pub fn dli(x: *mut f64) -> f64;
    #[link_name = "dlngam_"]
    pub fn dlngam(x: *mut f64) -> f64;
    #[link_name = "dlnrel_"]
    pub fn dlnrel(x: *mut f64) -> f64;
    #[link_name = "dlog_"]
    pub fn dlog(x: *mut f64) -> f64;
    #[link_name = "dlog10_"]
    pub fn dlog10(x: *mut f64) -> f64;
    #[link_name = "dnrm2_"]
    pub fn dnrm2(n: *mut FortranInteger, dx: *mut f64, incx: *mut FortranInteger) -> f64;
    #[link_name = "dpchdf_"]
    pub fn dpchdf(
        k: *mut FortranInteger,
        x: *mut f64,
        s: *mut f64,
        ierr: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dpchst_"]
    pub fn dpchst(arg1: *mut f64, arg2: *mut f64) -> f64;
    #[link_name = "dpoch_"]
    pub fn dpoch(a: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dpoch1_"]
    pub fn dpoch1(a: *mut f64, x: *mut f64) -> f64;
    #[link_name = "dppval_"]
    pub fn dppval(
        ldc: *mut FortranInteger,
        c: *mut f64,
        xi: *mut f64,
        lxi: *mut FortranInteger,
        k: *mut FortranInteger,
        ideriv: *mut FortranInteger,
        x: *mut f64,
        inppv: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dprvec_"]
    pub fn dprvec(m: *mut FortranInteger, u: *mut f64, v: *mut f64) -> f64;
    #[link_name = "dpsi_"]
    pub fn dpsi(x: *mut f64) -> f64;
    #[link_name = "dpsixn_"]
    pub fn dpsixn(n: *mut FortranInteger) -> f64;
    #[link_name = "dqdota_"]
    pub fn dqdota(
        n: *mut FortranInteger,
        db: *mut f64,
        qc: *mut FortranInteger,
        dx: *mut f64,
        incx: *mut FortranInteger,
        dy: *mut f64,
        incy: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dqdoti_"]
    pub fn dqdoti(
        n: *mut FortranInteger,
        db: *mut f64,
        qc: *mut FortranInteger,
        dx: *mut f64,
        incx: *mut FortranInteger,
        dy: *mut f64,
        incy: *mut FortranInteger,
    ) -> f64;
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
    #[link_name = "drc_"]
    pub fn drc(x: *mut f64, y: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "drd_"]
    pub fn drd(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "drf_"]
    pub fn drf(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "drj_"]
    pub fn drj(x: *mut f64, y: *mut f64, z: *mut f64, p: *mut f64, ier: *mut FortranInteger)
    -> f64;
    #[link_name = "dsdot_"]
    pub fn dsdot(
        n: *mut FortranInteger,
        sx: *mut f32,
        incx: *mut FortranInteger,
        sy: *mut f32,
        incy: *mut FortranInteger,
    ) -> f64;
    #[link_name = "dsin_"]
    pub fn dsin(x: *mut f64) -> f64;
    #[link_name = "dsindg_"]
    pub fn dsindg(x: *mut f64) -> f64;
    #[link_name = "dsinh_"]
    pub fn dsinh(x: *mut f64) -> f64;
    #[link_name = "dspenc_"]
    pub fn dspenc(x: *mut f64) -> f64;
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
    #[link_name = "e1_"]
    pub fn e1(x: *mut f32) -> f32;
    #[link_name = "ei_"]
    pub fn ei(x: *mut f32) -> f32;
    #[link_name = "enorm_"]
    pub fn enorm(n: *mut FortranInteger, x: *mut f32) -> f32;
    #[link_name = "erf_"]
    pub fn erf(x: *mut f32) -> f32;
    #[link_name = "erfc_"]
    pub fn erfc(x: *mut f32) -> f32;
    #[link_name = "exp_"]
    pub fn exp(x: *mut f32) -> f32;
    #[link_name = "exprel_"]
    pub fn exprel(x: *mut f32) -> f32;
    #[link_name = "fac_"]
    pub fn fac(n: *mut FortranInteger) -> f32;
    #[link_name = "gami_"]
    pub fn gami(a: *mut f32, x: *mut f32) -> f32;
    #[link_name = "gamic_"]
    pub fn gamic(a: *mut f32, x: *mut f32) -> f32;
    #[link_name = "gamit_"]
    pub fn gamit(a: *mut f32, x: *mut f32) -> f32;
    #[link_name = "gamln_"]
    pub fn gamln(z: *mut f32, ierr: *mut FortranInteger) -> f32;
    #[link_name = "gamma_"]
    pub fn gamma(x: *mut f32) -> f32;
    #[link_name = "gamr_"]
    pub fn gamr(x: *mut f32) -> f32;
    #[link_name = "gamrn_"]
    pub fn gamrn(x: *mut f32) -> f32;
    #[link_name = "hvnrm_"]
    pub fn hvnrm(v: *mut f32, ncomp: *mut FortranInteger) -> f32;
    #[link_name = "idamax_"]
    pub fn idamax(
        n: *mut FortranInteger,
        dx: *mut f64,
        incx: *mut FortranInteger,
    ) -> FortranInteger;
    #[link_name = "idloc_"]
    pub fn idloc(loc: *mut FortranInteger, sx: *mut f64, ix: *mut FortranInteger)
    -> FortranInteger;
    #[link_name = "initds_"]
    pub fn initds(os: *mut f64, nos: *mut FortranInteger, eta: *mut f32) -> FortranInteger;
    #[link_name = "inits_"]
    pub fn inits(os: *mut f32, nos: *mut FortranInteger, eta: *mut f32) -> FortranInteger;
    #[link_name = "iploc_"]
    pub fn iploc(loc: *mut FortranInteger, sx: *mut f32, ix: *mut FortranInteger)
    -> FortranInteger;
    #[link_name = "isamax_"]
    pub fn isamax(
        n: *mut FortranInteger,
        sx: *mut f32,
        incx: *mut FortranInteger,
    ) -> FortranInteger;
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
    #[link_name = "poch_"]
    pub fn poch(a: *mut f32, x: *mut f32) -> f32;
    #[link_name = "poch1_"]
    pub fn poch1(a: *mut f32, x: *mut f32) -> f32;
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
    #[link_name = "ppval_"]
    pub fn ppval(
        ldc: *mut FortranInteger,
        c: *mut f32,
        xi: *mut f32,
        lxi: *mut FortranInteger,
        k: *mut FortranInteger,
        ideriv: *mut FortranInteger,
        x: *mut f32,
        inppv: *mut FortranInteger,
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
    #[link_name = "psi_"]
    pub fn psi(x: *mut f32) -> f32;
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
    #[link_name = "rand_"]
    pub fn rand(r: *mut f32) -> f32;
    #[link_name = "rc_"]
    pub fn rc(x: *mut f32, y: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "rd_"]
    pub fn rd(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "rf_"]
    pub fn rf(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "rgauss_"]
    pub fn rgauss(xmean: *mut f32, sd: *mut f32) -> f32;
    #[link_name = "rj_"]
    pub fn rj(x: *mut f32, y: *mut f32, z: *mut f32, p: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "runif_"]
    pub fn runif(t: *mut f32, n: *mut FortranInteger) -> f32;
    #[link_name = "sasum_"]
    pub fn sasum(n: *mut FortranInteger, sx: *mut f32, incx: *mut FortranInteger) -> f32;
    #[link_name = "sdanrm_"]
    pub fn sdanrm(
        neq: *mut FortranInteger,
        v: *mut f32,
        wt: *mut f32,
        rpar: *mut f32,
        ipar: *mut FortranInteger,
    ) -> f32;
    #[link_name = "sdot_"]
    pub fn sdot(
        n: *mut FortranInteger,
        sx: *mut f32,
        incx: *mut FortranInteger,
        sy: *mut f32,
        incy: *mut FortranInteger,
    ) -> f32;
    #[link_name = "sdsdot_"]
    pub fn sdsdot(
        n: *mut FortranInteger,
        sb: *mut f32,
        sx: *mut f32,
        incx: *mut FortranInteger,
        sy: *mut f32,
        incy: *mut FortranInteger,
    ) -> f32;
    #[link_name = "sin_"]
    pub fn sin(x: *mut f32) -> f32;
    #[link_name = "sindg_"]
    pub fn sindg(x: *mut f32) -> f32;
    #[link_name = "sinh_"]
    pub fn sinh(x: *mut f32) -> f32;
    #[link_name = "snrm2_"]
    pub fn snrm2(n: *mut FortranInteger, sx: *mut f32, incx: *mut FortranInteger) -> f32;
    #[link_name = "spenc_"]
    pub fn spenc(x: *mut f32) -> f32;
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
