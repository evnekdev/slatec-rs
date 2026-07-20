//! Generated raw declarations for `batch_complex_arguments`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "c1merg_"]
    pub fn c1merg(
        tcos: *mut Complex32,
        i1: *mut FortranInteger,
        m1: *mut FortranInteger,
        i2: *mut FortranInteger,
        m2: *mut FortranInteger,
        i3: *mut FortranInteger,
    );
    #[link_name = "cabs_"]
    pub fn cabs(z: *mut Complex32) -> f32;
    #[link_name = "cacai_"]
    pub fn cacai(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        rl: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cacon_"]
    pub fn cacon(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        rl: *mut f32,
        fnul: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "carg_"]
    pub fn carg(z: *mut Complex32) -> f32;
    #[link_name = "casyi_"]
    pub fn casyi(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        rl: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cbinu_"]
    pub fn cbinu(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        cy: *mut Complex32,
        nz: *mut FortranInteger,
        rl: *mut f32,
        fnul: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cbknu_"]
    pub fn cbknu(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cblkt1_"]
    pub fn cblkt1(
        n: *mut FortranInteger,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        m: *mut FortranInteger,
        am: *mut Complex32,
        bm: *mut Complex32,
        cm: *mut Complex32,
        idimy: *mut FortranInteger,
        y: *mut Complex32,
        b: *mut f32,
        w1: *mut Complex32,
        w2: *mut Complex32,
        w3: *mut Complex32,
        wd: *mut Complex32,
        ww: *mut Complex32,
        wu: *mut Complex32,
        prdct: *mut f32,
        cprdct: *mut f32,
    );
    #[link_name = "cbuni_"]
    pub fn cbuni(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        nui: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cbunk_"]
    pub fn cbunk(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cdcor_"]
    pub fn cdcor(
        dfdy: *mut Complex32,
        el: *mut f32,
        fa: *mut f32,
        h: *mut f32,
        ierror: *mut FortranInteger,
        r#impl: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        matdim: *mut FortranInteger,
        miter: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        n: *mut FortranInteger,
        nde: *mut FortranInteger,
        nq: *mut FortranInteger,
        t: *mut f32,
        users: *mut f32,
        y: *mut Complex32,
        yh: *mut Complex32,
        ywt: *mut Complex32,
        evalfa: *mut FortranLogical,
        save1: *mut Complex32,
        save2: *mut Complex32,
        a: *mut Complex32,
        d: *mut f32,
        jstate: *mut FortranInteger,
    );
    #[link_name = "cdntl_"]
    pub fn cdntl(
        eps: *mut f32,
        f: *mut f32,
        fa: *mut f32,
        hmax: *mut f32,
        hold: *mut f32,
        r#impl: *mut FortranInteger,
        jtask: *mut FortranInteger,
        matdim: *mut FortranInteger,
        maxord: *mut FortranInteger,
        mint: *mut FortranInteger,
        miter: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        n: *mut FortranInteger,
        nde: *mut FortranInteger,
        save1: *mut Complex32,
        t: *mut f32,
        uround: *mut f32,
        users: *mut f32,
        y: *mut Complex32,
        ywt: *mut Complex32,
        h: *mut f32,
        mntold: *mut FortranInteger,
        mtrold: *mut FortranInteger,
        nfe: *mut FortranInteger,
        rc: *mut f32,
        yh: *mut Complex32,
        a: *mut Complex32,
        convrg: *mut FortranLogical,
        el: *mut f32,
        fac: *mut Complex32,
        ier: *mut FortranLogical,
        ipvt: *mut FortranInteger,
        nq: *mut FortranInteger,
        nwait: *mut FortranInteger,
        rh: *mut f32,
        rmax: *mut f32,
        save2: *mut Complex32,
        tq: *mut f32,
        trend: *mut f32,
        iswflg: *mut FortranInteger,
        jstate: *mut FortranInteger,
    );
    #[link_name = "cdntp_"]
    pub fn cdntp(
        h: *mut f32,
        k: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        t: *mut f32,
        tout: *mut f32,
        yh: *mut Complex32,
        y: *mut Complex32,
    );
    #[link_name = "cdpsc_"]
    pub fn cdpsc(
        ksgn: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        yh: *mut Complex32,
    );
    #[link_name = "cdpst_"]
    pub fn cdpst(
        el: *mut f32,
        f: *mut f32,
        fa: *mut f32,
        h: *mut f32,
        r#impl: *mut FortranInteger,
        jacobn: *mut FortranInteger,
        matdim: *mut FortranInteger,
        miter: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        n: *mut FortranInteger,
        nde: *mut FortranInteger,
        nq: *mut FortranInteger,
        save2: *mut Complex32,
        t: *mut f32,
        users: *mut f32,
        y: *mut Complex32,
        yh: *mut Complex32,
        ywt: *mut Complex32,
        uround: *mut f32,
        nfe: *mut FortranInteger,
        nje: *mut FortranInteger,
        a: *mut Complex32,
        dfdy: *mut Complex32,
        fac: *mut Complex32,
        ier: *mut FortranLogical,
        ipvt: *mut FortranInteger,
        save1: *mut Complex32,
        iswflg: *mut FortranInteger,
        bnd: *mut f32,
        jstate: *mut FortranInteger,
    );
    #[link_name = "cdscl_"]
    pub fn cdscl(
        hmax: *mut f32,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        rmax: *mut f32,
        h: *mut f32,
        rc: *mut f32,
        rh: *mut f32,
        yh: *mut Complex32,
    );
    #[link_name = "cdzro_"]
    pub fn cdzro(
        ae: *mut f32,
        f: *mut f32,
        h: *mut f32,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        iroot: *mut FortranInteger,
        re: *mut f32,
        t: *mut f32,
        yh: *mut Complex32,
        uround: *mut f32,
        b: *mut f32,
        c: *mut f32,
        fb: *mut f32,
        fc: *mut f32,
        y: *mut Complex32,
    );
    #[link_name = "cfftb_"]
    pub fn cfftb(n: *mut FortranInteger, c: *mut Complex32, wsave: *mut f32);
    #[link_name = "cfftf_"]
    pub fn cfftf(n: *mut FortranInteger, c: *mut Complex32, wsave: *mut f32);
    #[link_name = "ckscl_"]
    pub fn ckscl(
        zr: *mut Complex32,
        fnu: *mut f32,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        rz: *mut Complex32,
        ascle: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
    );
    #[link_name = "cmlri_"]
    pub fn cmlri(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
    );
    #[link_name = "cmpcsg_"]
    pub fn cmpcsg(
        n: *mut FortranInteger,
        ijump: *mut FortranInteger,
        fnum: *mut f32,
        fden: *mut f32,
        a: *mut Complex32,
    );
    #[link_name = "cmposd_"]
    pub fn cmposd(
        mr: *mut FortranInteger,
        nr: *mut FortranInteger,
        istag: *mut FortranInteger,
        ba: *mut Complex32,
        bb: *mut Complex32,
        bc: *mut Complex32,
        q: *mut Complex32,
        idimq: *mut FortranInteger,
        b: *mut Complex32,
        w: *mut Complex32,
        d: *mut Complex32,
        tcos: *mut Complex32,
        p: *mut Complex32,
    );
    #[link_name = "cmposn_"]
    pub fn cmposn(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        istag: *mut FortranInteger,
        mixbnd: *mut FortranInteger,
        a: *mut Complex32,
        bb: *mut Complex32,
        c: *mut Complex32,
        q: *mut Complex32,
        idimq: *mut FortranInteger,
        b: *mut Complex32,
        b2: *mut Complex32,
        b3: *mut Complex32,
        w: *mut Complex32,
        w2: *mut Complex32,
        w3: *mut Complex32,
        d: *mut Complex32,
        tcos: *mut Complex32,
        p: *mut Complex32,
    );
    #[link_name = "cmposp_"]
    pub fn cmposp(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut Complex32,
        bb: *mut Complex32,
        c: *mut Complex32,
        q: *mut Complex32,
        idimq: *mut FortranInteger,
        b: *mut Complex32,
        b2: *mut Complex32,
        b3: *mut Complex32,
        w: *mut Complex32,
        w2: *mut Complex32,
        w3: *mut Complex32,
        d: *mut Complex32,
        tcos: *mut Complex32,
        p: *mut Complex32,
    );
    #[link_name = "cmptr3_"]
    pub fn cmptr3(
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        k: *mut FortranInteger,
        y1: *mut Complex32,
        y2: *mut Complex32,
        y3: *mut Complex32,
        tcos: *mut Complex32,
        d: *mut Complex32,
        w1: *mut Complex32,
        w2: *mut Complex32,
        w3: *mut Complex32,
    );
    #[link_name = "cmptrx_"]
    pub fn cmptrx(
        idegbr: *mut FortranInteger,
        idegcr: *mut FortranInteger,
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        y: *mut Complex32,
        tcos: *mut Complex32,
        d: *mut Complex32,
        w: *mut Complex32,
    );
    #[link_name = "cpadd_"]
    pub fn cpadd(
        n: *mut FortranInteger,
        ierror: *mut FortranInteger,
        a: *mut f32,
        c: *mut f32,
        cbp: *mut Complex32,
        bp: *mut f32,
        bh: *mut f32,
    );
    #[link_name = "cpevl_"]
    pub fn cpevl(
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        a: *mut Complex32,
        z: *mut Complex32,
        c: *mut Complex32,
        b: *mut Complex32,
        kbd: *mut FortranLogical,
    );
    #[link_name = "cproc_"]
    pub fn cproc(
        nd: *mut FortranInteger,
        bd: *mut Complex32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut Complex32,
        y: *mut Complex32,
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        d: *mut Complex32,
        w: *mut Complex32,
        yy: *mut f32,
    );
    #[link_name = "cprocp_"]
    pub fn cprocp(
        nd: *mut FortranInteger,
        bd: *mut Complex32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut Complex32,
        y: *mut Complex32,
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        d: *mut Complex32,
        u: *mut Complex32,
        yy: *mut f32,
    );
    #[link_name = "cprod_"]
    pub fn cprod(
        nd: *mut FortranInteger,
        bd: *mut Complex32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut f32,
        yy: *mut f32,
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut Complex32,
        w: *mut Complex32,
        y: *mut Complex32,
    );
    #[link_name = "cprodp_"]
    pub fn cprodp(
        nd: *mut FortranInteger,
        bd: *mut Complex32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut f32,
        yy: *mut f32,
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut Complex32,
        u: *mut Complex32,
        y: *mut Complex32,
    );
    #[link_name = "crati_"]
    pub fn crati(
        z: *mut Complex32,
        fnu: *mut f32,
        n: *mut FortranInteger,
        cy: *mut Complex32,
        tol: *mut f32,
    );
    #[link_name = "cs1s2_"]
    pub fn cs1s2(
        zr: *mut Complex32,
        s1: *mut Complex32,
        s2: *mut Complex32,
        nz: *mut FortranInteger,
        ascle: *mut f32,
        alim: *mut f32,
        iuf: *mut FortranInteger,
    );
    #[link_name = "cseri_"]
    pub fn cseri(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cshch_"]
    pub fn cshch(z: *mut Complex32, csh: *mut Complex32, cch: *mut Complex32);
    #[link_name = "cuchk_"]
    pub fn cuchk(y: *mut Complex32, nz: *mut FortranInteger, ascle: *mut f32, tol: *mut f32);
    #[link_name = "cunhj_"]
    pub fn cunhj(
        z: *mut Complex32,
        fnu: *mut f32,
        ipmtr: *mut FortranInteger,
        tol: *mut f32,
        phi: *mut Complex32,
        arg: *mut Complex32,
        zeta1: *mut Complex32,
        zeta2: *mut Complex32,
        asum: *mut Complex32,
        bsum: *mut Complex32,
    );
    #[link_name = "cuni1_"]
    pub fn cuni1(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cuni2_"]
    pub fn cuni2(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cunik_"]
    pub fn cunik(
        zr: *mut Complex32,
        fnu: *mut f32,
        ikflg: *mut FortranInteger,
        ipmtr: *mut FortranInteger,
        tol: *mut f32,
        init: *mut FortranInteger,
        phi: *mut Complex32,
        zeta1: *mut Complex32,
        zeta2: *mut Complex32,
        sum: *mut Complex32,
        cwrk: *mut Complex32,
    );
    #[link_name = "cunk1_"]
    pub fn cunk1(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cunk2_"]
    pub fn cunk2(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cuoik_"]
    pub fn cuoik(
        z: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        ikflg: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nuf: *mut FortranInteger,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "cwrsk_"]
    pub fn cwrsk(
        zr: *mut Complex32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut Complex32,
        nz: *mut FortranInteger,
        cw: *mut Complex32,
        tol: *mut f32,
        elim: *mut f32,
        alim: *mut f32,
    );
    #[link_name = "ppadd_"]
    pub fn ppadd(
        n: *mut FortranInteger,
        ierror: *mut FortranInteger,
        a: *mut f32,
        c: *mut f32,
        cbp: *mut Complex32,
        bp: *mut f32,
        bh: *mut f32,
    );
    #[link_name = "proc_"]
    pub fn proc(
        nd: *mut FortranInteger,
        bd: *mut f32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut Complex32,
        y: *mut Complex32,
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        d: *mut Complex32,
        w: *mut Complex32,
        u: *mut Complex32,
    );
    #[link_name = "procp_"]
    pub fn procp(
        nd: *mut FortranInteger,
        bd: *mut f32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut Complex32,
        y: *mut Complex32,
        m: *mut FortranInteger,
        a: *mut Complex32,
        b: *mut Complex32,
        c: *mut Complex32,
        d: *mut Complex32,
        u: *mut Complex32,
        w: *mut Complex32,
    );
}

// ffi-declaration-aliases:start
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::caxpy`."]
pub use crate::blas::level1::caxpy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ccopy`."]
pub use crate::blas::level1::ccopy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::crotg`."]
pub use crate::blas::level1::crotg;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cscal`."]
pub use crate::blas::level1::cscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::csrot`."]
pub use crate::blas::level1::csrot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::csscal`."]
pub use crate::blas::level1::csscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cswap`."]
pub use crate::blas::level1::cswap;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dcdot`."]
pub use crate::blas::level1::dcdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::icamax`."]
pub use crate::blas::level1::icamax;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scasum`."]
pub use crate::blas::level1::scasum;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scnrm2`."]
pub use crate::blas::level1::scnrm2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgerc`."]
pub use crate::blas::level2::cgerc;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgeru`."]
pub use crate::blas::level2::cgeru;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbco`."]
pub use crate::linear_algebra::banded::complex::cgbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbdi`."]
pub use crate::linear_algebra::banded::complex::cgbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbfa`."]
pub use crate::linear_algebra::banded::complex::cgbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbsl`."]
pub use crate::linear_algebra::banded::complex::cgbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgtsl`."]
pub use crate::linear_algebra::banded::complex::cgtsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbco`."]
pub use crate::linear_algebra::banded::complex::cnbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbdi`."]
pub use crate::linear_algebra::banded::complex::cnbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbfa`."]
pub use crate::linear_algebra::banded::complex::cnbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbfs`."]
pub use crate::linear_algebra::banded::complex::cnbfs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbir`."]
pub use crate::linear_algebra::banded::complex::cnbir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbsl`."]
pub use crate::linear_algebra::banded::complex::cnbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbco`."]
pub use crate::linear_algebra::banded::complex::cpbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbdi`."]
pub use crate::linear_algebra::banded::complex::cpbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbfa`."]
pub use crate::linear_algebra::banded::complex::cpbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbsl`."]
pub use crate::linear_algebra::banded::complex::cpbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cptsl`."]
pub use crate::linear_algebra::banded::complex::cptsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchdc`."]
pub use crate::linear_algebra::dense::complex::cchdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchdd`."]
pub use crate::linear_algebra::dense::complex::cchdd;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchex`."]
pub use crate::linear_algebra::dense::complex::cchex;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchud`."]
pub use crate::linear_algebra::dense::complex::cchud;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgeco`."]
pub use crate::linear_algebra::dense::complex::cgeco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgedi`."]
pub use crate::linear_algebra::dense::complex::cgedi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgefa`."]
pub use crate::linear_algebra::dense::complex::cgefa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgefs`."]
pub use crate::linear_algebra::dense::complex::cgefs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgeir`."]
pub use crate::linear_algebra::dense::complex::cgeir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgesl`."]
pub use crate::linear_algebra::dense::complex::cgesl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chico`."]
pub use crate::linear_algebra::dense::complex::chico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chidi`."]
pub use crate::linear_algebra::dense::complex::chidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chifa`."]
pub use crate::linear_algebra::dense::complex::chifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chisl`."]
pub use crate::linear_algebra::dense::complex::chisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpoco`."]
pub use crate::linear_algebra::dense::complex::cpoco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpodi`."]
pub use crate::linear_algebra::dense::complex::cpodi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpofa`."]
pub use crate::linear_algebra::dense::complex::cpofa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpofs`."]
pub use crate::linear_algebra::dense::complex::cpofs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpoir`."]
pub use crate::linear_algebra::dense::complex::cpoir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cposl`."]
pub use crate::linear_algebra::dense::complex::cposl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cqrdc`."]
pub use crate::linear_algebra::dense::complex::cqrdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cqrsl`."]
pub use crate::linear_algebra::dense::complex::cqrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csico`."]
pub use crate::linear_algebra::dense::complex::csico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csidi`."]
pub use crate::linear_algebra::dense::complex::csidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csifa`."]
pub use crate::linear_algebra::dense::complex::csifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csisl`."]
pub use crate::linear_algebra::dense::complex::csisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csvdc`."]
pub use crate::linear_algebra::dense::complex::csvdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrco`."]
pub use crate::linear_algebra::dense::complex::ctrco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrdi`."]
pub use crate::linear_algebra::dense::complex::ctrdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrsl`."]
pub use crate::linear_algebra::dense::complex::ctrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpco`."]
pub use crate::linear_algebra::packed::complex::chpco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpdi`."]
pub use crate::linear_algebra::packed::complex::chpdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpfa`."]
pub use crate::linear_algebra::packed::complex::chpfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpsl`."]
pub use crate::linear_algebra::packed::complex::chpsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppco`."]
pub use crate::linear_algebra::packed::complex::cppco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppdi`."]
pub use crate::linear_algebra::packed::complex::cppdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppfa`."]
pub use crate::linear_algebra::packed::complex::cppfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppsl`."]
pub use crate::linear_algebra::packed::complex::cppsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspco`."]
pub use crate::linear_algebra::packed::complex::cspco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspdi`."]
pub use crate::linear_algebra::packed::complex::cspdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspfa`."]
pub use crate::linear_algebra::packed::complex::cspfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspsl`."]
pub use crate::linear_algebra::packed::complex::cspsl;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::complex::cpqr79`."]
pub use crate::nonlinear::complex::cpqr79;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::complex::cpzero`."]
pub use crate::nonlinear::complex::cpzero;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::complex::rpqr79`."]
pub use crate::nonlinear::complex::rpqr79;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::complex::rpzero`."]
pub use crate::nonlinear::complex::rpzero;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::complex::cblktr`."]
pub use crate::pde::fishpack::complex::cblktr;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::complex::cmgnbn`."]
pub use crate::pde::fishpack::complex::cmgnbn;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cairy`."]
pub use crate::special::complex::cairy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesh`."]
pub use crate::special::complex::cbesh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesi`."]
pub use crate::special::complex::cbesi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesj`."]
pub use crate::special::complex::cbesj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesk`."]
pub use crate::special::complex::cbesk;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesy`."]
pub use crate::special::complex::cbesy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbiry`."]
pub use crate::special::complex::cbiry;
// ffi-declaration-aliases:end
