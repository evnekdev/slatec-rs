//! Generated raw declarations for `batch_numeric_scalar_subroutines`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "algams_"]
    pub fn algams(x: *mut f32, algam: *mut f32, sgngam: *mut f32);
    #[link_name = "bkisr_"]
    pub fn bkisr(x: *mut f32, n: *mut FortranInteger, sum: *mut f32, ierr: *mut FortranInteger);
    #[link_name = "bspdoc_"]
    pub fn bspdoc();
    #[link_name = "cdiv_"]
    pub fn cdiv(ar: *mut f32, ai: *mut f32, br: *mut f32, bi: *mut f32, cr: *mut f32, ci: *mut f32);
    #[link_name = "csroot_"]
    pub fn csroot(xr: *mut f32, xi: *mut f32, yr: *mut f32, yi: *mut f32);
    #[link_name = "d9aimp_"]
    pub fn d9aimp(x: *mut f64, ampl: *mut f64, theta: *mut f64);
    #[link_name = "d9b0mp_"]
    pub fn d9b0mp(x: *mut f64, ampl: *mut f64, theta: *mut f64);
    #[link_name = "d9b1mp_"]
    pub fn d9b1mp(x: *mut f64, ampl: *mut f64, theta: *mut f64);
    #[link_name = "d9knus_"]
    pub fn d9knus(
        xnu: *mut f64,
        x: *mut f64,
        bknu: *mut f64,
        bknu1: *mut f64,
        iswtch: *mut FortranInteger,
    );
    #[link_name = "d9upak_"]
    pub fn d9upak(x: *mut f64, y: *mut f64, n: *mut FortranInteger);
    #[link_name = "dbkisr_"]
    pub fn dbkisr(x: *mut f64, n: *mut FortranInteger, sum: *mut f64, ierr: *mut FortranInteger);
    #[link_name = "dfzero_"]
    pub fn dfzero(
        f: *mut f64,
        b: *mut f64,
        c: *mut f64,
        r: *mut f64,
        re: *mut f64,
        ae: *mut f64,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dgamlm_"]
    pub fn dgamlm(xmin: *mut f64, xmax: *mut f64);
    #[link_name = "dgaus8_"]
    pub fn dgaus8(
        fun: *mut f64,
        a: *mut f64,
        b: *mut f64,
        err: *mut f64,
        ans: *mut f64,
        ierr: *mut FortranInteger,
    );
    #[link_name = "djairy_"]
    pub fn djairy(x: *mut f64, rx: *mut f64, c: *mut f64, ai: *mut f64, dai: *mut f64);
    #[link_name = "dlgams_"]
    pub fn dlgams(x: *mut f64, dlgam: *mut f64, sgngam: *mut f64);
    #[link_name = "dlpdoc_"]
    pub fn dlpdoc();
    #[link_name = "dmacon_"]
    pub fn dmacon();
    #[link_name = "dpchsw_"]
    pub fn dpchsw(
        dfmax: *mut f64,
        iextrm: *mut FortranInteger,
        d1: *mut f64,
        d2: *mut f64,
        h: *mut f64,
        slope: *mut f64,
        ierr: *mut FortranInteger,
    );
    #[link_name = "drotg_"]
    pub fn drotg(da: *mut f64, db: *mut f64, dc: *mut f64, ds: *mut f64);
    #[link_name = "dxadd_"]
    pub fn dxadd(
        x: *mut f64,
        ix: *mut FortranInteger,
        y: *mut f64,
        iy: *mut FortranInteger,
        z: *mut f64,
        iz: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxadj_"]
    pub fn dxadj(x: *mut f64, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "dxc210_"]
    pub fn dxc210(
        k: *mut FortranInteger,
        z: *mut f64,
        j: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxcon_"]
    pub fn dxcon(x: *mut f64, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "dxred_"]
    pub fn dxred(x: *mut f64, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "dxset_"]
    pub fn dxset(
        irad: *mut FortranInteger,
        nradpl: *mut FortranInteger,
        dzero: *mut f64,
        nbits: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dyairy_"]
    pub fn dyairy(x: *mut f64, rx: *mut f64, c: *mut f64, bi: *mut f64, dbi: *mut f64);
    #[link_name = "eisdoc_"]
    pub fn eisdoc();
    #[link_name = "fftdoc_"]
    pub fn fftdoc();
    #[link_name = "fundoc_"]
    pub fn fundoc();
    #[link_name = "fzero_"]
    pub fn fzero(
        f: *mut f32,
        b: *mut f32,
        c: *mut f32,
        r: *mut f32,
        re: *mut f32,
        ae: *mut f32,
        iflag: *mut FortranInteger,
    );
    #[link_name = "gamlim_"]
    pub fn gamlim(xmin: *mut f32, xmax: *mut f32);
    #[link_name = "gaus8_"]
    pub fn gaus8(
        fun: *mut f32,
        a: *mut f32,
        b: *mut f32,
        err: *mut f32,
        ans: *mut f32,
        ierr: *mut FortranInteger,
    );
    #[link_name = "indxa_"]
    pub fn indxa(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idxa: *mut FortranInteger,
        na: *mut FortranInteger,
    );
    #[link_name = "indxb_"]
    pub fn indxb(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idx: *mut FortranInteger,
        idp: *mut FortranInteger,
    );
    #[link_name = "indxc_"]
    pub fn indxc(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idxc: *mut FortranInteger,
        nc: *mut FortranInteger,
    );
    #[link_name = "inxca_"]
    pub fn inxca(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idxa: *mut FortranInteger,
        na: *mut FortranInteger,
    );
    #[link_name = "inxcb_"]
    pub fn inxcb(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idx: *mut FortranInteger,
        idp: *mut FortranInteger,
    );
    #[link_name = "inxcc_"]
    pub fn inxcc(
        i: *mut FortranInteger,
        ir: *mut FortranInteger,
        idxc: *mut FortranInteger,
        nc: *mut FortranInteger,
    );
    #[link_name = "jairy_"]
    pub fn jairy(x: *mut f32, rx: *mut f32, c: *mut f32, ai: *mut f32, dai: *mut f32);
    #[link_name = "macon_"]
    pub fn macon();
    #[link_name = "mpblas_"]
    pub fn mpblas(i1: *mut FortranInteger);
    #[link_name = "mpchk_"]
    pub fn mpchk(i: *mut FortranInteger, j: *mut FortranInteger);
    #[link_name = "mperr_"]
    pub fn mperr();
    #[link_name = "pchdoc_"]
    pub fn pchdoc();
    #[link_name = "pchsw_"]
    pub fn pchsw(
        dfmax: *mut f32,
        iextrm: *mut FortranInteger,
        d1: *mut f32,
        d2: *mut f32,
        h: *mut f32,
        slope: *mut f32,
        ierr: *mut FortranInteger,
    );
    #[link_name = "qpdoc_"]
    pub fn qpdoc();
    #[link_name = "r9aimp_"]
    pub fn r9aimp(x: *mut f32, ampl: *mut f32, theta: *mut f32);
    #[link_name = "r9knus_"]
    pub fn r9knus(
        xnu: *mut f32,
        x: *mut f32,
        bknu: *mut f32,
        bknu1: *mut f32,
        iswtch: *mut FortranInteger,
    );
    #[link_name = "r9upak_"]
    pub fn r9upak(x: *mut f32, y: *mut f32, n: *mut FortranInteger);
    #[link_name = "sclosm_"]
    pub fn sclosm(ipage: *mut FortranInteger);
    #[link_name = "slpdoc_"]
    pub fn slpdoc();
    #[link_name = "sopenm_"]
    pub fn sopenm(ipage: *mut FortranInteger, lpage: *mut FortranInteger);
    #[link_name = "srotg_"]
    pub fn srotg(sa: *mut f32, sb: *mut f32, sc: *mut f32, ss: *mut f32);
    #[link_name = "xadd_"]
    pub fn xadd(
        x: *mut f32,
        ix: *mut FortranInteger,
        y: *mut f32,
        iy: *mut FortranInteger,
        z: *mut f32,
        iz: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xadj_"]
    pub fn xadj(x: *mut f32, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "xc210_"]
    pub fn xc210(
        k: *mut FortranInteger,
        z: *mut f32,
        j: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xcon_"]
    pub fn xcon(x: *mut f32, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "xred_"]
    pub fn xred(x: *mut f32, ix: *mut FortranInteger, ierror: *mut FortranInteger);
    #[link_name = "yairy_"]
    pub fn yairy(x: *mut f32, rx: *mut f32, c: *mut f32, bi: *mut f32, dbi: *mut f32);
    #[link_name = "zairy_"]
    pub fn zairy(
        zr: *mut f64,
        zi: *mut f64,
        id: *mut FortranInteger,
        kode: *mut FortranInteger,
        air: *mut f64,
        aii: *mut f64,
        nz: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    #[link_name = "zbiry_"]
    pub fn zbiry(
        zr: *mut f64,
        zi: *mut f64,
        id: *mut FortranInteger,
        kode: *mut FortranInteger,
        bir: *mut f64,
        bii: *mut f64,
        ierr: *mut FortranInteger,
    );
    #[link_name = "zdiv_"]
    pub fn zdiv(ar: *mut f64, ai: *mut f64, br: *mut f64, bi: *mut f64, cr: *mut f64, ci: *mut f64);
    #[link_name = "zexp_"]
    pub fn zexp(ar: *mut f64, ai: *mut f64, br: *mut f64, bi: *mut f64);
    #[link_name = "zlog_"]
    pub fn zlog(ar: *mut f64, ai: *mut f64, br: *mut f64, bi: *mut f64, ierr: *mut FortranInteger);
    #[link_name = "zmlt_"]
    pub fn zmlt(ar: *mut f64, ai: *mut f64, br: *mut f64, bi: *mut f64, cr: *mut f64, ci: *mut f64);
    #[link_name = "zs1s2_"]
    pub fn zs1s2(
        zrr: *mut f64,
        zri: *mut f64,
        s1r: *mut f64,
        s1i: *mut f64,
        s2r: *mut f64,
        s2i: *mut f64,
        nz: *mut FortranInteger,
        ascle: *mut f64,
        alim: *mut f64,
        iuf: *mut FortranInteger,
    );
    #[link_name = "zshch_"]
    pub fn zshch(
        zr: *mut f64,
        zi: *mut f64,
        cshr: *mut f64,
        cshi: *mut f64,
        cchr: *mut f64,
        cchi: *mut f64,
    );
    #[link_name = "zsqrt_"]
    pub fn zsqrt(ar: *mut f64, ai: *mut f64, br: *mut f64, bi: *mut f64);
    #[link_name = "zuchk_"]
    pub fn zuchk(
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        ascle: *mut f64,
        tol: *mut f64,
    );
    #[link_name = "zunhj_"]
    pub fn zunhj(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        ipmtr: *mut FortranInteger,
        tol: *mut f64,
        phir: *mut f64,
        phii: *mut f64,
        argr: *mut f64,
        argi: *mut f64,
        zeta1r: *mut f64,
        zeta1i: *mut f64,
        zeta2r: *mut f64,
        zeta2i: *mut f64,
        asumr: *mut f64,
        asumi: *mut f64,
        bsumr: *mut f64,
        bsumi: *mut f64,
    );
}
