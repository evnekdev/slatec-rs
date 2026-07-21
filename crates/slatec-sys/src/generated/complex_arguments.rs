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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/caxpy.md"))]
pub use crate::blas::level1::caxpy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ccopy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ccopy.md"))]
pub use crate::blas::level1::ccopy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::crotg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/crotg.md"))]
pub use crate::blas::level1::crotg;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cscal`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cscal.md"))]
pub use crate::blas::level1::cscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::csrot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csrot.md"))]
pub use crate::blas::level1::csrot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::csscal`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csscal.md"))]
pub use crate::blas::level1::csscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::cswap`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cswap.md"))]
pub use crate::blas::level1::cswap;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dcdot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcdot.md"))]
pub use crate::blas::level1::dcdot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::icamax`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/icamax.md"))]
pub use crate::blas::level1::icamax;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scasum`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/scasum.md"))]
pub use crate::blas::level1::scasum;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scnrm2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/scnrm2.md"))]
pub use crate::blas::level1::scnrm2;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgerc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgerc.md"))]
pub use crate::blas::level2::cgerc;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::cgeru`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgeru.md"))]
pub use crate::blas::level2::cgeru;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgbco.md"))]
pub use crate::linear_algebra::banded::complex::cgbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgbdi.md"))]
pub use crate::linear_algebra::banded::complex::cgbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgbfa.md"))]
pub use crate::linear_algebra::banded::complex::cgbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgbsl.md"))]
pub use crate::linear_algebra::banded::complex::cgbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cgtsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgtsl.md"))]
pub use crate::linear_algebra::banded::complex::cgtsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbco.md"))]
pub use crate::linear_algebra::banded::complex::cnbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbdi.md"))]
pub use crate::linear_algebra::banded::complex::cnbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbfa.md"))]
pub use crate::linear_algebra::banded::complex::cnbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbfs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbfs.md"))]
pub use crate::linear_algebra::banded::complex::cnbfs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbir.md"))]
pub use crate::linear_algebra::banded::complex::cnbir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cnbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cnbsl.md"))]
pub use crate::linear_algebra::banded::complex::cnbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpbco.md"))]
pub use crate::linear_algebra::banded::complex::cpbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpbdi.md"))]
pub use crate::linear_algebra::banded::complex::cpbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpbfa.md"))]
pub use crate::linear_algebra::banded::complex::cpbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cpbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpbsl.md"))]
pub use crate::linear_algebra::banded::complex::cpbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::complex::cptsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cptsl.md"))]
pub use crate::linear_algebra::banded::complex::cptsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cchdc.md"))]
pub use crate::linear_algebra::dense::complex::cchdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchdd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cchdd.md"))]
pub use crate::linear_algebra::dense::complex::cchdd;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchex`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cchex.md"))]
pub use crate::linear_algebra::dense::complex::cchex;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cchud`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cchud.md"))]
pub use crate::linear_algebra::dense::complex::cchud;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgeco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgeco.md"))]
pub use crate::linear_algebra::dense::complex::cgeco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgedi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgedi.md"))]
pub use crate::linear_algebra::dense::complex::cgedi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgefa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgefa.md"))]
pub use crate::linear_algebra::dense::complex::cgefa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgefs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgefs.md"))]
pub use crate::linear_algebra::dense::complex::cgefs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgeir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgeir.md"))]
pub use crate::linear_algebra::dense::complex::cgeir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cgesl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgesl.md"))]
pub use crate::linear_algebra::dense::complex::cgesl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chico`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chico.md"))]
pub use crate::linear_algebra::dense::complex::chico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chidi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chidi.md"))]
pub use crate::linear_algebra::dense::complex::chidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chifa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chifa.md"))]
pub use crate::linear_algebra::dense::complex::chifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::chisl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chisl.md"))]
pub use crate::linear_algebra::dense::complex::chisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpoco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpoco.md"))]
pub use crate::linear_algebra::dense::complex::cpoco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpodi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpodi.md"))]
pub use crate::linear_algebra::dense::complex::cpodi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpofa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpofa.md"))]
pub use crate::linear_algebra::dense::complex::cpofa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpofs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpofs.md"))]
pub use crate::linear_algebra::dense::complex::cpofs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cpoir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpoir.md"))]
pub use crate::linear_algebra::dense::complex::cpoir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cposl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cposl.md"))]
pub use crate::linear_algebra::dense::complex::cposl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cqrdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cqrdc.md"))]
pub use crate::linear_algebra::dense::complex::cqrdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::cqrsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cqrsl.md"))]
pub use crate::linear_algebra::dense::complex::cqrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csico`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csico.md"))]
pub use crate::linear_algebra::dense::complex::csico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csidi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csidi.md"))]
pub use crate::linear_algebra::dense::complex::csidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csifa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csifa.md"))]
pub use crate::linear_algebra::dense::complex::csifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csisl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csisl.md"))]
pub use crate::linear_algebra::dense::complex::csisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::csvdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/csvdc.md"))]
pub use crate::linear_algebra::dense::complex::csvdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrco.md"))]
pub use crate::linear_algebra::dense::complex::ctrco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrdi.md"))]
pub use crate::linear_algebra::dense::complex::ctrdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::complex::ctrsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ctrsl.md"))]
pub use crate::linear_algebra::dense::complex::ctrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpco.md"))]
pub use crate::linear_algebra::packed::complex::chpco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpdi.md"))]
pub use crate::linear_algebra::packed::complex::chpdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpfa.md"))]
pub use crate::linear_algebra::packed::complex::chpfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::chpsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chpsl.md"))]
pub use crate::linear_algebra::packed::complex::chpsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cppco.md"))]
pub use crate::linear_algebra::packed::complex::cppco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cppdi.md"))]
pub use crate::linear_algebra::packed::complex::cppdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cppfa.md"))]
pub use crate::linear_algebra::packed::complex::cppfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cppsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cppsl.md"))]
pub use crate::linear_algebra::packed::complex::cppsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cspco.md"))]
pub use crate::linear_algebra::packed::complex::cspco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cspdi.md"))]
pub use crate::linear_algebra::packed::complex::cspdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cspfa.md"))]
pub use crate::linear_algebra::packed::complex::cspfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::complex::cspsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cspsl.md"))]
pub use crate::linear_algebra::packed::complex::cspsl;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::complex::cblktr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cblktr.md"))]
pub use crate::pde::fishpack::complex::cblktr;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::complex::cmgnbn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cmgnbn.md"))]
pub use crate::pde::fishpack::complex::cmgnbn;
#[doc = "Transitional ABI-shaped alias; use `crate::roots::complex::cpqr79`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpqr79.md"))]
pub use crate::roots::complex::cpqr79;
#[doc = "Transitional ABI-shaped alias; use `crate::roots::complex::cpzero`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cpzero.md"))]
pub use crate::roots::complex::cpzero;
#[doc = "Transitional ABI-shaped alias; use `crate::roots::complex::rpqr79`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rpqr79.md"))]
pub use crate::roots::complex::rpqr79;
#[doc = "Transitional ABI-shaped alias; use `crate::roots::complex::rpzero`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rpzero.md"))]
pub use crate::roots::complex::rpzero;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cairy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cairy.md"))]
pub use crate::special::complex::cairy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbesh.md"))]
pub use crate::special::complex::cbesh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbesi.md"))]
pub use crate::special::complex::cbesi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbesj.md"))]
pub use crate::special::complex::cbesj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbesk.md"))]
pub use crate::special::complex::cbesk;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbesy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbesy.md"))]
pub use crate::special::complex::cbesy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::complex::cbiry`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbiry.md"))]
pub use crate::special::complex::cbiry;
// ffi-declaration-aliases:end
