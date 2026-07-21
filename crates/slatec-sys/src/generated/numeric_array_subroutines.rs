//! Generated raw declarations for `batch_numeric_array_subroutines`.
//! Snapshot: `complete-slatec-05078ebcb649b50e4435`. GNU Fortran target: `x86_64-w64-mingw32`.
//!
//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.
#![allow(clippy::missing_safety_doc, unused_imports)]

use crate::{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical};
use core::ffi::c_char;

unsafe extern "C" {
    #[link_name = "asyik_"]
    pub fn asyik(
        x: *mut f32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        flgik: *mut f32,
        ra: *mut f32,
        arg: *mut f32,
        r#in: *mut FortranInteger,
        y: *mut f32,
    );
    #[link_name = "asyjy_"]
    pub fn asyjy(
        funjy: *mut f32,
        x: *mut f32,
        fnu: *mut f32,
        flgjy: *mut f32,
        r#in: *mut FortranInteger,
        y: *mut f32,
        wk: *mut f32,
        iflw: *mut FortranInteger,
    );
    #[link_name = "bdiff_"]
    pub fn bdiff(l: *mut FortranInteger, v: *mut f32);
    #[link_name = "besknu_"]
    pub fn besknu(
        x: *mut f32,
        fnu: *mut f32,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut f32,
        nz: *mut FortranInteger,
    );
    #[link_name = "besynu_"]
    pub fn besynu(x: *mut f32, fnu: *mut f32, n: *mut FortranInteger, y: *mut f32);
    #[link_name = "bkias_"]
    pub fn bkias(
        x: *mut f32,
        n: *mut FortranInteger,
        ktrms: *mut FortranInteger,
        t: *mut f32,
        ans: *mut f32,
        ind: *mut FortranInteger,
        ms: *mut FortranInteger,
        gmrn: *mut f32,
        h: *mut f32,
        ierr: *mut FortranInteger,
    );
    #[link_name = "bksol_"]
    pub fn bksol(n: *mut FortranInteger, a: *mut f32, x: *mut f32);
    #[link_name = "blktr1_"]
    pub fn blktr1(
        n: *mut FortranInteger,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        m: *mut FortranInteger,
        am: *mut f32,
        bm: *mut f32,
        cm: *mut f32,
        idimy: *mut FortranInteger,
        y: *mut f32,
        b: *mut f32,
        w1: *mut f32,
        w2: *mut f32,
        w3: *mut f32,
        wd: *mut f32,
        ww: *mut f32,
        wu: *mut f32,
        prdct: *mut f32,
        cprdct: *mut f32,
    );
    #[link_name = "bnfac_"]
    pub fn bnfac(
        w: *mut f32,
        nroww: *mut FortranInteger,
        nrow: *mut FortranInteger,
        nbandl: *mut FortranInteger,
        nbandu: *mut FortranInteger,
        iflag: *mut FortranInteger,
    );
    #[link_name = "bnslv_"]
    pub fn bnslv(
        w: *mut f32,
        nroww: *mut FortranInteger,
        nrow: *mut FortranInteger,
        nbandl: *mut FortranInteger,
        nbandu: *mut FortranInteger,
        b: *mut f32,
    );
    #[link_name = "bsgq8_"]
    pub fn bsgq8(
        fun: *mut f32,
        xt: *mut f32,
        bc: *mut f32,
        n: *mut FortranInteger,
        kk: *mut FortranInteger,
        id: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        inbv: *mut FortranInteger,
        err: *mut f32,
        ans: *mut f32,
        ierr: *mut FortranInteger,
        work: *mut f32,
    );
    #[link_name = "bsplvd_"]
    pub fn bsplvd(
        t: *mut f32,
        k: *mut FortranInteger,
        x: *mut f32,
        ileft: *mut FortranInteger,
        vnikx: *mut f32,
        nderiv: *mut FortranInteger,
    );
    #[link_name = "bsplvn_"]
    pub fn bsplvn(
        t: *mut f32,
        jhigh: *mut FortranInteger,
        index: *mut FortranInteger,
        x: *mut f32,
        ileft: *mut FortranInteger,
        vnikx: *mut f32,
    );
    #[link_name = "bvder_"]
    pub fn bvder(x: *mut f32, y: *mut f32, yp: *mut f32, g: *mut f32, ipar: *mut FortranInteger);
    #[link_name = "bvpor_"]
    pub fn bvpor(
        y: *mut f32,
        nrowy: *mut FortranInteger,
        ncomp: *mut FortranInteger,
        xpts: *mut f32,
        nxpts: *mut FortranInteger,
        a: *mut f32,
        nrowa: *mut FortranInteger,
        alpha: *mut f32,
        nic: *mut FortranInteger,
        b: *mut f32,
        nrowb: *mut FortranInteger,
        beta: *mut f32,
        nfc: *mut FortranInteger,
        iflag: *mut FortranInteger,
        z: *mut f32,
        mxnon: *mut FortranInteger,
        p: *mut f32,
        ntp: *mut FortranInteger,
        ip: *mut FortranInteger,
        w: *mut f32,
        niv: *mut FortranInteger,
        yhp: *mut f32,
        u: *mut f32,
        v: *mut f32,
        coef: *mut f32,
        s: *mut f32,
        stowa: *mut f32,
        g: *mut f32,
        work: *mut f32,
        iwork: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "bvsup_"]
    pub fn bvsup(
        y: *mut f32,
        nrowy: *mut FortranInteger,
        ncomp: *mut FortranInteger,
        xpts: *mut f32,
        nxpts: *mut FortranInteger,
        a: *mut f32,
        nrowa: *mut FortranInteger,
        alpha: *mut f32,
        nic: *mut FortranInteger,
        b: *mut f32,
        nrowb: *mut FortranInteger,
        beta: *mut f32,
        nfc: *mut FortranInteger,
        igofx: *mut FortranInteger,
        re: *mut f32,
        ae: *mut f32,
        iflag: *mut FortranInteger,
        work: *mut f32,
        ndw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        ndiw: *mut FortranInteger,
        neqivp: *mut FortranInteger,
    );
    #[link_name = "ccmpb_"]
    pub fn ccmpb(
        n: *mut FortranInteger,
        ierror: *mut FortranInteger,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        b: *mut f32,
        ah: *mut f32,
        bh: *mut f32,
    );
    #[link_name = "cdcst_"]
    pub fn cdcst(
        maxord: *mut FortranInteger,
        mint: *mut FortranInteger,
        iswflg: *mut FortranInteger,
        el: *mut f32,
        tq: *mut f32,
    );
    #[link_name = "cffti_"]
    pub fn cffti(n: *mut FortranInteger, wsave: *mut f32);
    #[link_name = "cfod_"]
    pub fn cfod(meth: *mut FortranInteger, elco: *mut f32, tesco: *mut f32);
    #[link_name = "compb_"]
    pub fn compb(
        n: *mut FortranInteger,
        ierror: *mut FortranInteger,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        b: *mut f32,
        ah: *mut f32,
        bh: *mut f32,
    );
    #[link_name = "cosgen_"]
    pub fn cosgen(
        n: *mut FortranInteger,
        ijump: *mut FortranInteger,
        fnum: *mut f32,
        fden: *mut f32,
        a: *mut f32,
    );
    #[link_name = "cosqb1_"]
    pub fn cosqb1(n: *mut FortranInteger, x: *mut f32, w: *mut f32, xh: *mut f32);
    #[link_name = "cosqf1_"]
    pub fn cosqf1(n: *mut FortranInteger, x: *mut f32, w: *mut f32, xh: *mut f32);
    #[link_name = "cpevlr_"]
    pub fn cpevlr(
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        a: *mut f32,
        x: *mut f32,
        c: *mut f32,
    );
    #[link_name = "cscale_"]
    pub fn cscale(
        a: *mut f32,
        nrda: *mut FortranInteger,
        nrow: *mut FortranInteger,
        ncol: *mut FortranInteger,
        cols: *mut f32,
        colsav: *mut f32,
        rows: *mut f32,
        rowsav: *mut f32,
        anorm: *mut f32,
        scales: *mut f32,
        iscale: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "d1merg_"]
    pub fn d1merg(
        tcos: *mut f64,
        i1: *mut FortranInteger,
        m1: *mut FortranInteger,
        i2: *mut FortranInteger,
        m2: *mut FortranInteger,
        i3: *mut FortranInteger,
    );
    #[link_name = "d1mpyq_"]
    pub fn d1mpyq(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f64,
        lda: *mut FortranInteger,
        v: *mut f64,
        w: *mut f64,
    );
    #[link_name = "dasyik_"]
    pub fn dasyik(
        x: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        flgik: *mut f64,
        ra: *mut f64,
        arg: *mut f64,
        r#in: *mut FortranInteger,
        y: *mut f64,
    );
    #[link_name = "dasyjy_"]
    pub fn dasyjy(
        funjy: *mut f32,
        x: *mut f64,
        fnu: *mut f64,
        flgjy: *mut f64,
        r#in: *mut FortranInteger,
        y: *mut f64,
        wk: *mut f64,
        iflw: *mut FortranInteger,
    );
    #[link_name = "dbdiff_"]
    pub fn dbdiff(l: *mut FortranInteger, v: *mut f64);
    #[link_name = "dbhin_"]
    pub fn dbhin(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f64,
        isym: *mut FortranInteger,
        soln: *mut f64,
        rhs: *mut f64,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "dbkias_"]
    pub fn dbkias(
        x: *mut f64,
        n: *mut FortranInteger,
        ktrms: *mut FortranInteger,
        t: *mut f64,
        ans: *mut f64,
        ind: *mut FortranInteger,
        ms: *mut FortranInteger,
        gmrn: *mut f64,
        h: *mut f64,
        ierr: *mut FortranInteger,
    );
    #[link_name = "dbksol_"]
    pub fn dbksol(n: *mut FortranInteger, a: *mut f64, x: *mut f64);
    #[link_name = "dbnfac_"]
    pub fn dbnfac(
        w: *mut f64,
        nroww: *mut FortranInteger,
        nrow: *mut FortranInteger,
        nbandl: *mut FortranInteger,
        nbandu: *mut FortranInteger,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dbnslv_"]
    pub fn dbnslv(
        w: *mut f64,
        nroww: *mut FortranInteger,
        nrow: *mut FortranInteger,
        nbandl: *mut FortranInteger,
        nbandu: *mut FortranInteger,
        b: *mut f64,
    );
    #[link_name = "dbolsm_"]
    pub fn dbolsm(
        w: *mut f64,
        mdw: *mut FortranInteger,
        minput: *mut FortranInteger,
        ncols: *mut FortranInteger,
        bl: *mut f64,
        bu: *mut f64,
        ind: *mut FortranInteger,
        iopt: *mut FortranInteger,
        x: *mut f64,
        rnorm: *mut f64,
        mode: *mut FortranInteger,
        rw: *mut f64,
        ww: *mut f64,
        scl: *mut f64,
        ibasis: *mut FortranInteger,
        ibb: *mut FortranInteger,
    );
    #[link_name = "dbsgq8_"]
    pub fn dbsgq8(
        fun: *mut f64,
        xt: *mut f64,
        bc: *mut f64,
        n: *mut FortranInteger,
        kk: *mut FortranInteger,
        id: *mut FortranInteger,
        a: *mut f64,
        b: *mut f64,
        inbv: *mut FortranInteger,
        err: *mut f64,
        ans: *mut f64,
        ierr: *mut FortranInteger,
        work: *mut f64,
    );
    #[link_name = "dbsknu_"]
    pub fn dbsknu(
        x: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        y: *mut f64,
        nz: *mut FortranInteger,
    );
    #[link_name = "dbsynu_"]
    pub fn dbsynu(x: *mut f64, fnu: *mut f64, n: *mut FortranInteger, y: *mut f64);
    #[link_name = "dbvder_"]
    pub fn dbvder(x: *mut f64, y: *mut f64, yp: *mut f64, g: *mut f64, ipar: *mut FortranInteger);
    #[link_name = "dbvpor_"]
    pub fn dbvpor(
        y: *mut f64,
        nrowy: *mut FortranInteger,
        ncomp: *mut FortranInteger,
        xpts: *mut f64,
        nxpts: *mut FortranInteger,
        a: *mut f64,
        nrowa: *mut FortranInteger,
        alpha: *mut f64,
        nic: *mut FortranInteger,
        b: *mut f64,
        nrowb: *mut FortranInteger,
        beta: *mut f64,
        nfc: *mut FortranInteger,
        iflag: *mut FortranInteger,
        z: *mut f64,
        mxnon: *mut FortranInteger,
        p: *mut f64,
        ntp: *mut FortranInteger,
        ip: *mut FortranInteger,
        w: *mut f64,
        niv: *mut FortranInteger,
        yhp: *mut f64,
        u: *mut f64,
        v: *mut f64,
        coef: *mut f64,
        s: *mut f64,
        stowa: *mut f64,
        g: *mut f64,
        work: *mut f64,
        iwork: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "dbvsup_"]
    pub fn dbvsup(
        y: *mut f64,
        nrowy: *mut FortranInteger,
        ncomp: *mut FortranInteger,
        xpts: *mut f64,
        nxpts: *mut FortranInteger,
        a: *mut f64,
        nrowa: *mut FortranInteger,
        alpha: *mut f64,
        nic: *mut FortranInteger,
        b: *mut f64,
        nrowb: *mut FortranInteger,
        beta: *mut f64,
        nfc: *mut FortranInteger,
        igofx: *mut FortranInteger,
        re: *mut f64,
        ae: *mut f64,
        iflag: *mut FortranInteger,
        work: *mut f64,
        ndw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        ndiw: *mut FortranInteger,
        neqivp: *mut FortranInteger,
    );
    #[link_name = "dcfod_"]
    pub fn dcfod(meth: *mut FortranInteger, elco: *mut f64, tesco: *mut f64);
    #[link_name = "dcoef_"]
    pub fn dcoef(
        yh: *mut f64,
        yp: *mut f64,
        ncomp: *mut FortranInteger,
        nrowb: *mut FortranInteger,
        nfc: *mut FortranInteger,
        nic: *mut FortranInteger,
        b: *mut f64,
        beta: *mut f64,
        coef: *mut f64,
        inhomo: *mut FortranInteger,
        re: *mut f64,
        ae: *mut f64,
        by: *mut f64,
        cvec: *mut f64,
        work: *mut f64,
        iwork: *mut FortranInteger,
        iflag: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "dcpplt_"]
    pub fn dcpplt(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f64,
        isym: *mut FortranInteger,
        iunit: *mut FortranInteger,
    );
    #[link_name = "dcscal_"]
    pub fn dcscal(
        a: *mut f64,
        nrda: *mut FortranInteger,
        nrow: *mut FortranInteger,
        ncol: *mut FortranInteger,
        cols: *mut f64,
        colsav: *mut f64,
        rows: *mut f64,
        rowsav: *mut f64,
        anorm: *mut f64,
        scales: *mut f64,
        iscale: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "ddaslv_"]
    pub fn ddaslv(
        neq: *mut FortranInteger,
        delta: *mut f64,
        wm: *mut f64,
        iwm: *mut FortranInteger,
    );
    #[link_name = "ddatrp_"]
    pub fn ddatrp(
        x: *mut f64,
        xout: *mut f64,
        yout: *mut f64,
        ypout: *mut f64,
        neq: *mut FortranInteger,
        kold: *mut FortranInteger,
        phi: *mut f64,
        psi: *mut f64,
    );
    #[link_name = "ddawts_"]
    pub fn ddawts(
        neq: *mut FortranInteger,
        iwt: *mut FortranInteger,
        rtol: *mut f64,
        atol: *mut f64,
        y: *mut f64,
        wt: *mut f64,
        rpar: *mut f64,
        ipar: *mut FortranInteger,
    );
    #[link_name = "ddcst_"]
    pub fn ddcst(
        maxord: *mut FortranInteger,
        mint: *mut FortranInteger,
        iswflg: *mut FortranInteger,
        el: *mut f64,
        tq: *mut f64,
    );
    #[link_name = "ddntp_"]
    pub fn ddntp(
        h: *mut f64,
        k: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        t: *mut f64,
        tout: *mut f64,
        yh: *mut f64,
        y: *mut f64,
    );
    #[link_name = "ddoglg_"]
    pub fn ddoglg(
        n: *mut FortranInteger,
        r: *mut f64,
        lr: *mut FortranInteger,
        diag: *mut f64,
        qtb: *mut f64,
        delta: *mut f64,
        x: *mut f64,
        wa1: *mut f64,
        wa2: *mut f64,
    );
    #[link_name = "ddpsc_"]
    pub fn ddpsc(
        ksgn: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        yh: *mut f64,
    );
    #[link_name = "ddscl_"]
    pub fn ddscl(
        hmax: *mut f64,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        rmax: *mut f64,
        h: *mut f64,
        rc: *mut f64,
        rh: *mut f64,
        yh: *mut f64,
    );
    #[link_name = "ddzro_"]
    pub fn ddzro(
        ae: *mut f64,
        f: *mut f64,
        h: *mut f64,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        iroot: *mut FortranInteger,
        re: *mut f64,
        t: *mut f64,
        yh: *mut f64,
        uround: *mut f64,
        b: *mut f64,
        c: *mut f64,
        fb: *mut f64,
        fc: *mut f64,
        y: *mut f64,
    );
    #[link_name = "defcmn_"]
    pub fn defcmn(
        ndata: *mut FortranInteger,
        xdata: *mut f64,
        ydata: *mut f64,
        sddata: *mut f64,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkptin: *mut f64,
        mdein: *mut FortranInteger,
        mdeout: *mut FortranInteger,
        coeff: *mut f64,
        bf: *mut f64,
        xtemp: *mut f64,
        ptemp: *mut f64,
        bkpt: *mut f64,
        g: *mut f64,
        mdg: *mut FortranInteger,
        w: *mut f64,
        mdw: *mut FortranInteger,
        lw: *mut FortranInteger,
    );
    #[link_name = "defehl_"]
    pub fn defehl(
        f: *mut f32,
        neq: *mut FortranInteger,
        t: *mut f32,
        y: *mut f32,
        h: *mut f32,
        yp: *mut f32,
        f1: *mut f32,
        f2: *mut f32,
        f3: *mut f32,
        f4: *mut f32,
        f5: *mut f32,
        ys: *mut f32,
        rpar: *mut f32,
        ipar: *mut FortranInteger,
    );
    #[link_name = "dexbvp_"]
    pub fn dexbvp(
        y: *mut f64,
        nrowy: *mut FortranInteger,
        xpts: *mut f64,
        a: *mut f64,
        nrowa: *mut FortranInteger,
        alpha: *mut f64,
        b: *mut f64,
        nrowb: *mut FortranInteger,
        beta: *mut f64,
        iflag: *mut FortranInteger,
        work: *mut f64,
        iwork: *mut FortranInteger,
    );
    #[link_name = "dfcmn_"]
    pub fn dfcmn(
        ndata: *mut FortranInteger,
        xdata: *mut f64,
        ydata: *mut f64,
        sddata: *mut f64,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkptin: *mut f64,
        nconst: *mut FortranInteger,
        xconst: *mut f64,
        yconst: *mut f64,
        nderiv: *mut FortranInteger,
        mode: *mut FortranInteger,
        coeff: *mut f64,
        bf: *mut f64,
        xtemp: *mut f64,
        ptemp: *mut f64,
        bkpt: *mut f64,
        g: *mut f64,
        mdg: *mut FortranInteger,
        w: *mut f64,
        mdw: *mut FortranInteger,
        work: *mut f64,
        iwork: *mut FortranInteger,
    );
    #[link_name = "dfdjc1_"]
    pub fn dfdjc1(
        fcn: *mut f32,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        iflag: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f64,
        wa1: *mut f64,
        wa2: *mut f64,
    );
    #[link_name = "dfdjc3_"]
    pub fn dfdjc3(
        fcn: *mut f32,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        iflag: *mut FortranInteger,
        epsfcn: *mut f64,
        wa: *mut f64,
    );
    #[link_name = "dfehl_"]
    pub fn dfehl(
        df: *mut f32,
        neq: *mut FortranInteger,
        t: *mut f64,
        y: *mut f64,
        h: *mut f64,
        yp: *mut f64,
        f1: *mut f64,
        f2: *mut f64,
        f3: *mut f64,
        f4: *mut f64,
        f5: *mut f64,
        ys: *mut f64,
        rpar: *mut f64,
        ipar: *mut FortranInteger,
    );
    #[link_name = "dfspvd_"]
    pub fn dfspvd(
        t: *mut f64,
        k: *mut FortranInteger,
        x: *mut f64,
        ileft: *mut FortranInteger,
        vnikx: *mut f64,
        nderiv: *mut FortranInteger,
    );
    #[link_name = "dfspvn_"]
    pub fn dfspvn(
        t: *mut f64,
        jhigh: *mut FortranInteger,
        index: *mut FortranInteger,
        x: *mut f64,
        ileft: *mut FortranInteger,
        vnikx: *mut f64,
    );
    #[link_name = "dfulmt_"]
    pub fn dfulmt(
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        aij: *mut f64,
        indcat: *mut FortranInteger,
        prgopt: *mut f64,
        dattrv: *mut f64,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dh12_"]
    pub fn dh12(
        mode: *mut FortranInteger,
        lpivot: *mut FortranInteger,
        l1: *mut FortranInteger,
        m: *mut FortranInteger,
        u: *mut f64,
        iue: *mut FortranInteger,
        up: *mut f64,
        c: *mut f64,
        ice: *mut FortranInteger,
        icv: *mut FortranInteger,
        ncv: *mut FortranInteger,
    );
    #[link_name = "dhels_"]
    pub fn dhels(
        a: *mut f64,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f64,
        b: *mut f64,
    );
    #[link_name = "dheqr_"]
    pub fn dheqr(
        a: *mut f64,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f64,
        info: *mut FortranInteger,
        ijob: *mut FortranInteger,
    );
    #[link_name = "dhkseq_"]
    pub fn dhkseq(x: *mut f64, m: *mut FortranInteger, h: *mut f64, ierr: *mut FortranInteger);
    #[link_name = "dintyd_"]
    pub fn dintyd(
        t: *mut f64,
        k: *mut FortranInteger,
        yh: *mut f64,
        nyh: *mut FortranInteger,
        dky: *mut f64,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dlpdp_"]
    pub fn dlpdp(
        a: *mut f64,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n1: *mut FortranInteger,
        n2: *mut FortranInteger,
        prgopt: *mut f64,
        x: *mut f64,
        wnorm: *mut f64,
        mode: *mut FortranInteger,
        ws: *mut f64,
        is: *mut FortranInteger,
    );
    #[link_name = "dlsi_"]
    pub fn dlsi(
        w: *mut f64,
        mdw: *mut FortranInteger,
        ma: *mut FortranInteger,
        mg: *mut FortranInteger,
        n: *mut FortranInteger,
        prgopt: *mut f64,
        x: *mut f64,
        rnorm: *mut f64,
        mode: *mut FortranInteger,
        ws: *mut f64,
        ip: *mut FortranInteger,
    );
    #[link_name = "dlssud_"]
    pub fn dlssud(
        a: *mut f64,
        x: *mut f64,
        b: *mut f64,
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        nrda: *mut FortranInteger,
        u: *mut f64,
        nrdu: *mut FortranInteger,
        iflag: *mut FortranInteger,
        mlso: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        q: *mut f64,
        diag: *mut f64,
        kpivot: *mut FortranInteger,
        s: *mut f64,
        div: *mut f64,
        td: *mut f64,
        isflg: *mut FortranInteger,
        scales: *mut f64,
    );
    #[link_name = "dmgsbv_"]
    pub fn dmgsbv(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f64,
        ia: *mut FortranInteger,
        niv: *mut FortranInteger,
        iflag: *mut FortranInteger,
        s: *mut f64,
        p: *mut f64,
        ip: *mut FortranInteger,
        inhomo: *mut FortranInteger,
        v: *mut f64,
        w: *mut f64,
        wcnd: *mut f64,
    );
    #[link_name = "dmpar_"]
    pub fn dmpar(
        n: *mut FortranInteger,
        r: *mut f64,
        ldr: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        diag: *mut f64,
        qtb: *mut f64,
        delta: *mut f64,
        par: *mut f64,
        x: *mut f64,
        sigma: *mut f64,
        wa1: *mut f64,
        wa2: *mut f64,
    );
    #[link_name = "dogleg_"]
    pub fn dogleg(
        n: *mut FortranInteger,
        r: *mut f32,
        lr: *mut FortranInteger,
        diag: *mut f32,
        qtb: *mut f32,
        delta: *mut f32,
        x: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "dohtrl_"]
    pub fn dohtrl(
        q: *mut f64,
        n: *mut FortranInteger,
        nrda: *mut FortranInteger,
        diag: *mut f64,
        irank: *mut FortranInteger,
        div: *mut f64,
        td: *mut f64,
    );
    #[link_name = "dorth_"]
    pub fn dorth(
        vnew: *mut f64,
        v: *mut f64,
        hes: *mut f64,
        n: *mut FortranInteger,
        ll: *mut FortranInteger,
        ldhes: *mut FortranInteger,
        kmp: *mut FortranInteger,
        snormw: *mut f64,
    );
    #[link_name = "dorthr_"]
    pub fn dorthr(
        a: *mut f64,
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        diag: *mut f64,
        kpivot: *mut FortranInteger,
        scales: *mut f64,
        rows: *mut f64,
        rs: *mut f64,
    );
    #[link_name = "dpchce_"]
    pub fn dpchce(
        ic: *mut FortranInteger,
        vc: *mut f64,
        n: *mut FortranInteger,
        x: *mut f64,
        h: *mut f64,
        slope: *mut f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    #[link_name = "dpchci_"]
    pub fn dpchci(
        n: *mut FortranInteger,
        h: *mut f64,
        slope: *mut f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
    );
    #[link_name = "dpchcs_"]
    pub fn dpchcs(
        switch: *mut f64,
        n: *mut FortranInteger,
        h: *mut f64,
        slope: *mut f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    #[link_name = "dpchkt_"]
    pub fn dpchkt(n: *mut FortranInteger, x: *mut f64, knotyp: *mut FortranInteger, t: *mut f64);
    #[link_name = "dpchng_"]
    pub fn dpchng(
        ii: *mut FortranInteger,
        xval: *mut f64,
        iplace: *mut FortranInteger,
        sx: *mut f64,
        ix: *mut FortranInteger,
        ircx: *mut FortranInteger,
    );
    #[link_name = "dpintm_"]
    pub fn dpintm(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        sx: *mut f64,
        ix: *mut FortranInteger,
        lmx: *mut FortranInteger,
        ipagef: *mut FortranInteger,
    );
    #[link_name = "dpnnzr_"]
    pub fn dpnnzr(
        i: *mut FortranInteger,
        xval: *mut f64,
        iplace: *mut FortranInteger,
        sx: *mut f64,
        ix: *mut FortranInteger,
        ircx: *mut FortranInteger,
    );
    #[link_name = "dpperm_"]
    pub fn dpperm(
        dx: *mut f64,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "dppgq8_"]
    pub fn dppgq8(
        fun: *mut f64,
        ldc: *mut FortranInteger,
        c: *mut f64,
        xi: *mut f64,
        lxi: *mut FortranInteger,
        kk: *mut FortranInteger,
        id: *mut FortranInteger,
        a: *mut f64,
        b: *mut f64,
        inppv: *mut FortranInteger,
        err: *mut f64,
        ans: *mut f64,
        ierr: *mut FortranInteger,
    );
    #[link_name = "dprwpg_"]
    pub fn dprwpg(
        key: *mut FortranInteger,
        ipage: *mut FortranInteger,
        lpg: *mut FortranInteger,
        sx: *mut f64,
        ix: *mut FortranInteger,
    );
    #[link_name = "dprwvr_"]
    pub fn dprwvr(
        key: *mut FortranInteger,
        ipage: *mut FortranInteger,
        lpg: *mut FortranInteger,
        sx: *mut f64,
        ix: *mut FortranInteger,
    );
    #[link_name = "dpsort_"]
    pub fn dpsort(
        dx: *mut f64,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        kflag: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "dqcheb_"]
    pub fn dqcheb(x: *mut f64, fval: *mut f64, cheb12: *mut f64, cheb24: *mut f64);
    #[link_name = "dqelg_"]
    pub fn dqelg(
        n: *mut FortranInteger,
        epstab: *mut f64,
        result: *mut f64,
        abserr: *mut f64,
        res3la: *mut f64,
        nres: *mut FortranInteger,
    );
    #[link_name = "dqform_"]
    pub fn dqform(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f64,
        ldq: *mut FortranInteger,
        wa: *mut f64,
    );
    #[link_name = "dqpsrt_"]
    pub fn dqpsrt(
        limit: *mut FortranInteger,
        last: *mut FortranInteger,
        maxerr: *mut FortranInteger,
        ermax: *mut f64,
        elist: *mut f64,
        iord: *mut FortranInteger,
        nrmax: *mut FortranInteger,
    );
    #[link_name = "dqrslv_"]
    pub fn dqrslv(
        n: *mut FortranInteger,
        r: *mut f64,
        ldr: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        diag: *mut f64,
        qtb: *mut f64,
        x: *mut f64,
        sigma: *mut f64,
        wa: *mut f64,
    );
    #[link_name = "dreadp_"]
    pub fn dreadp(
        ipage: *mut FortranInteger,
        list: *mut FortranInteger,
        rlist: *mut f64,
        lpage: *mut FortranInteger,
        irec: *mut FortranInteger,
    );
    #[link_name = "dreort_"]
    pub fn dreort(
        ncomp: *mut FortranInteger,
        y: *mut f64,
        yp: *mut f64,
        yhp: *mut f64,
        niv: *mut FortranInteger,
        w: *mut f64,
        s: *mut f64,
        p: *mut f64,
        ip: *mut FortranInteger,
        stowa: *mut f64,
        iflag: *mut FortranInteger,
    );
    #[link_name = "drkfab_"]
    pub fn drkfab(
        ncomp: *mut FortranInteger,
        xpts: *mut f64,
        nxpts: *mut FortranInteger,
        nfc: *mut FortranInteger,
        iflag: *mut FortranInteger,
        z: *mut f64,
        mxnon: *mut FortranInteger,
        p: *mut f64,
        ntp: *mut FortranInteger,
        ip: *mut FortranInteger,
        yhp: *mut f64,
        niv: *mut FortranInteger,
        u: *mut f64,
        v: *mut f64,
        w: *mut f64,
        s: *mut f64,
        stowa: *mut f64,
        g: *mut f64,
        work: *mut f64,
        iwork: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "drlcal_"]
    pub fn drlcal(
        n: *mut FortranInteger,
        kmp: *mut FortranInteger,
        ll: *mut FortranInteger,
        maxl: *mut FortranInteger,
        v: *mut f64,
        q: *mut f64,
        rl: *mut f64,
        snormw: *mut f64,
        prod: *mut f64,
        r0nrm: *mut f64,
    );
    #[link_name = "drsco_"]
    pub fn drsco(rsav: *mut f64, isav: *mut FortranInteger);
    #[link_name = "dslvs_"]
    pub fn dslvs(wm: *mut f64, iwm: *mut FortranInteger, x: *mut f64, tem: *mut f64);
    #[link_name = "dsort_"]
    pub fn dsort(dx: *mut f64, dy: *mut f64, n: *mut FortranInteger, kflag: *mut FortranInteger);
    #[link_name = "dsoseq_"]
    pub fn dsoseq(
        fnc: *mut f64,
        n: *mut FortranInteger,
        s: *mut f64,
        rtolx: *mut f64,
        atolx: *mut f64,
        tolf: *mut f64,
        iflag: *mut FortranInteger,
        mxit: *mut FortranInteger,
        ncjs: *mut FortranInteger,
        nsrrc: *mut FortranInteger,
        nsri: *mut FortranInteger,
        iprint: *mut FortranInteger,
        fmax: *mut f64,
        c: *mut f64,
        nc: *mut FortranInteger,
        b: *mut f64,
        p: *mut f64,
        temp: *mut f64,
        x: *mut f64,
        y: *mut f64,
        fac: *mut f64,
        is: *mut FortranInteger,
    );
    #[link_name = "dsossl_"]
    pub fn dsossl(
        k: *mut FortranInteger,
        n: *mut FortranInteger,
        l: *mut FortranInteger,
        x: *mut f64,
        c: *mut f64,
        b: *mut f64,
        m: *mut FortranInteger,
    );
    #[link_name = "dstor1_"]
    pub fn dstor1(
        u: *mut f64,
        yh: *mut f64,
        v: *mut f64,
        yp: *mut f64,
        ntemp: *mut FortranInteger,
        ndisk: *mut FortranInteger,
        ntape: *mut FortranInteger,
    );
    #[link_name = "dstway_"]
    pub fn dstway(
        u: *mut f64,
        v: *mut f64,
        yhp: *mut f64,
        inout: *mut FortranInteger,
        stowa: *mut f64,
    );
    #[link_name = "dsuds_"]
    pub fn dsuds(
        a: *mut f64,
        x: *mut f64,
        b: *mut f64,
        neq: *mut FortranInteger,
        nuk: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        mlso: *mut FortranInteger,
        work: *mut f64,
        iwork: *mut FortranInteger,
    );
    #[link_name = "dsvco_"]
    pub fn dsvco(rsav: *mut f64, isav: *mut FortranInteger);
    #[link_name = "dtin_"]
    pub fn dtin(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f64,
        isym: *mut FortranInteger,
        soln: *mut f64,
        rhs: *mut f64,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "dtout_"]
    pub fn dtout(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f64,
        isym: *mut FortranInteger,
        soln: *mut f64,
        rhs: *mut f64,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "du11ls_"]
    pub fn du11ls(
        a: *mut f64,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        ub: *mut f64,
        db: *mut f64,
        mode: *mut FortranInteger,
        np: *mut FortranInteger,
        krank: *mut FortranInteger,
        ksure: *mut FortranInteger,
        h: *mut f64,
        w: *mut f64,
        eb: *mut f64,
        ic: *mut FortranInteger,
        ir: *mut FortranInteger,
    );
    #[link_name = "du11us_"]
    pub fn du11us(
        a: *mut f64,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        ub: *mut f64,
        db: *mut f64,
        mode: *mut FortranInteger,
        np: *mut FortranInteger,
        krank: *mut FortranInteger,
        ksure: *mut FortranInteger,
        h: *mut f64,
        w: *mut f64,
        eb: *mut f64,
        ir: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "du12ls_"]
    pub fn du12ls(
        a: *mut f64,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        b: *mut f64,
        mdb: *mut FortranInteger,
        nb: *mut FortranInteger,
        mode: *mut FortranInteger,
        krank: *mut FortranInteger,
        rnorm: *mut f64,
        h: *mut f64,
        w: *mut f64,
        ic: *mut FortranInteger,
        ir: *mut FortranInteger,
    );
    #[link_name = "du12us_"]
    pub fn du12us(
        a: *mut f64,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        b: *mut f64,
        mdb: *mut FortranInteger,
        nb: *mut FortranInteger,
        mode: *mut FortranInteger,
        krank: *mut FortranInteger,
        rnorm: *mut f64,
        h: *mut f64,
        w: *mut f64,
        ir: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "dusrmt_"]
    pub fn dusrmt(
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        aij: *mut f64,
        indcat: *mut FortranInteger,
        prgopt: *mut f64,
        dattrv: *mut f64,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dvecs_"]
    pub fn dvecs(
        ncomp: *mut FortranInteger,
        lnfc: *mut FortranInteger,
        yhp: *mut f64,
        work: *mut f64,
        iwork: *mut FortranInteger,
        inhomo: *mut FortranInteger,
        iflag: *mut FortranInteger,
    );
    #[link_name = "dwnlsm_"]
    pub fn dwnlsm(
        w: *mut f64,
        mdw: *mut FortranInteger,
        mme: *mut FortranInteger,
        ma: *mut FortranInteger,
        n: *mut FortranInteger,
        l: *mut FortranInteger,
        prgopt: *mut f64,
        x: *mut f64,
        rnorm: *mut f64,
        mode: *mut FortranInteger,
        ipivot: *mut FortranInteger,
        itype: *mut FortranInteger,
        wd: *mut f64,
        h: *mut f64,
        scale: *mut f64,
        z: *mut f64,
        temp: *mut f64,
        d: *mut f64,
    );
    #[link_name = "dwnlt3_"]
    pub fn dwnlt3(
        i: *mut FortranInteger,
        imax: *mut FortranInteger,
        m: *mut FortranInteger,
        mdw: *mut FortranInteger,
        ipivot: *mut FortranInteger,
        h: *mut f64,
        w: *mut f64,
    );
    #[link_name = "dwritp_"]
    pub fn dwritp(
        ipage: *mut FortranInteger,
        list: *mut FortranInteger,
        rlist: *mut f64,
        lpage: *mut FortranInteger,
        irec: *mut FortranInteger,
    );
    #[link_name = "dwupdt_"]
    pub fn dwupdt(
        n: *mut FortranInteger,
        r: *mut f64,
        ldr: *mut FortranInteger,
        w: *mut f64,
        b: *mut f64,
        alpha: *mut f64,
        cos: *mut f64,
        sin: *mut f64,
    );
    #[link_name = "dx_"]
    pub fn dx(
        u: *mut f32,
        idmn: *mut FortranInteger,
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        uxxx: *mut f32,
        uxxxx: *mut f32,
    );
    #[link_name = "dx4_"]
    pub fn dx4(
        u: *mut f32,
        idmn: *mut FortranInteger,
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        uxxx: *mut f32,
        uxxxx: *mut f32,
    );
    #[link_name = "dxpmu_"]
    pub fn dxpmu(
        nu1: *mut f64,
        nu2: *mut f64,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        theta: *mut f64,
        x: *mut f64,
        sx: *mut f64,
        id: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxpmup_"]
    pub fn dxpmup(
        nu1: *mut f64,
        nu2: *mut f64,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxpnrm_"]
    pub fn dxpnrm(
        nu1: *mut f64,
        nu2: *mut f64,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxpqnu_"]
    pub fn dxpqnu(
        nu1: *mut f64,
        nu2: *mut f64,
        mu: *mut FortranInteger,
        theta: *mut f64,
        id: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxqmu_"]
    pub fn dxqmu(
        nu1: *mut f64,
        nu2: *mut f64,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        theta: *mut f64,
        x: *mut f64,
        sx: *mut f64,
        id: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dxqnu_"]
    pub fn dxqnu(
        nu1: *mut f64,
        nu2: *mut f64,
        mu1: *mut FortranInteger,
        theta: *mut f64,
        x: *mut f64,
        sx: *mut f64,
        id: *mut FortranInteger,
        pqa: *mut f64,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "dy_"]
    pub fn dy(
        u: *mut f32,
        idmn: *mut FortranInteger,
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        uyyy: *mut f32,
        uyyyy: *mut f32,
    );
    #[link_name = "dy4_"]
    pub fn dy4(
        u: *mut f32,
        idmn: *mut FortranInteger,
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        uyyy: *mut f32,
        uyyyy: *mut f32,
    );
    #[link_name = "efcmn_"]
    pub fn efcmn(
        ndata: *mut FortranInteger,
        xdata: *mut f32,
        ydata: *mut f32,
        sddata: *mut f32,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkptin: *mut f32,
        mdein: *mut FortranInteger,
        mdeout: *mut FortranInteger,
        coeff: *mut f32,
        bf: *mut f32,
        xtemp: *mut f32,
        ptemp: *mut f32,
        bkpt: *mut f32,
        g: *mut f32,
        mdg: *mut FortranInteger,
        w: *mut f32,
        mdw: *mut FortranInteger,
        lw: *mut FortranInteger,
    );
    #[link_name = "exbvp_"]
    pub fn exbvp(
        y: *mut f32,
        nrowy: *mut FortranInteger,
        xpts: *mut f32,
        a: *mut f32,
        nrowa: *mut FortranInteger,
        alpha: *mut f32,
        b: *mut f32,
        nrowb: *mut FortranInteger,
        beta: *mut f32,
        iflag: *mut FortranInteger,
        work: *mut f32,
        iwork: *mut FortranInteger,
    );
    #[link_name = "ezfft1_"]
    pub fn ezfft1(n: *mut FortranInteger, wa: *mut f32, ifac: *mut FortranInteger);
    #[link_name = "fcmn_"]
    pub fn fcmn(
        ndata: *mut FortranInteger,
        xdata: *mut f32,
        ydata: *mut f32,
        sddata: *mut f32,
        nord: *mut FortranInteger,
        nbkpt: *mut FortranInteger,
        bkptin: *mut f32,
        nconst: *mut FortranInteger,
        xconst: *mut f32,
        yconst: *mut f32,
        nderiv: *mut FortranInteger,
        mode: *mut FortranInteger,
        coeff: *mut f32,
        bf: *mut f32,
        xtemp: *mut f32,
        ptemp: *mut f32,
        bkpt: *mut f32,
        g: *mut f32,
        mdg: *mut FortranInteger,
        w: *mut f32,
        mdw: *mut FortranInteger,
        work: *mut f32,
        iwork: *mut FortranInteger,
    );
    #[link_name = "fdjac1_"]
    pub fn fdjac1(
        fcn: *mut f32,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        iflag: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "fdjac3_"]
    pub fn fdjac3(
        fcn: *mut f32,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        iflag: *mut FortranInteger,
        epsfcn: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "fulmat_"]
    pub fn fulmat(
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        aij: *mut f32,
        indcat: *mut FortranInteger,
        prgopt: *mut f32,
        dattrv: *mut f32,
        iflag: *mut FortranInteger,
    );
    #[link_name = "h12_"]
    pub fn h12(
        mode: *mut FortranInteger,
        lpivot: *mut FortranInteger,
        l1: *mut FortranInteger,
        m: *mut FortranInteger,
        u: *mut f32,
        iue: *mut FortranInteger,
        up: *mut f32,
        c: *mut f32,
        ice: *mut FortranInteger,
        icv: *mut FortranInteger,
        ncv: *mut FortranInteger,
    );
    #[link_name = "hkseq_"]
    pub fn hkseq(x: *mut f32, m: *mut FortranInteger, h: *mut f32, ierr: *mut FortranInteger);
    #[link_name = "hstcs1_"]
    pub fn hstcs1(
        intl: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        m: *mut FortranInteger,
        mbdcnd: *mut FortranInteger,
        bda: *mut f32,
        bdb: *mut f32,
        c: *mut f32,
        d: *mut f32,
        n: *mut FortranInteger,
        nbdcnd: *mut FortranInteger,
        bdc: *mut f32,
        bdd: *mut f32,
        elmbda: *mut f32,
        f: *mut f32,
        idimf: *mut FortranInteger,
        pertrb: *mut f32,
        ierr1: *mut FortranInteger,
        am: *mut f32,
        bm: *mut f32,
        cm: *mut f32,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        snth: *mut f32,
        rsq: *mut f32,
        wrk: *mut f32,
    );
    #[link_name = "hwscs1_"]
    pub fn hwscs1(
        intl: *mut FortranInteger,
        ts: *mut f32,
        tf: *mut f32,
        m: *mut FortranInteger,
        mbdcnd: *mut FortranInteger,
        bdts: *mut f32,
        bdtf: *mut f32,
        rs: *mut f32,
        rf: *mut f32,
        n: *mut FortranInteger,
        nbdcnd: *mut FortranInteger,
        bdrs: *mut f32,
        bdrf: *mut f32,
        elmbda: *mut f32,
        f: *mut f32,
        idimf: *mut FortranInteger,
        pertrb: *mut f32,
        w: *mut f32,
        s: *mut f32,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        r: *mut f32,
        am: *mut f32,
        bm: *mut f32,
        cm: *mut f32,
        sint: *mut f32,
        bmh: *mut f32,
    );
    #[link_name = "hwsss1_"]
    pub fn hwsss1(
        ts: *mut f32,
        tf: *mut f32,
        m: *mut FortranInteger,
        mbdcnd: *mut FortranInteger,
        bdts: *mut f32,
        bdtf: *mut f32,
        ps: *mut f32,
        pf: *mut f32,
        n: *mut FortranInteger,
        nbdcnd: *mut FortranInteger,
        bdps: *mut f32,
        bdpf: *mut f32,
        elmbda: *mut f32,
        f: *mut f32,
        idimf: *mut FortranInteger,
        pertrb: *mut f32,
        am: *mut f32,
        bm: *mut f32,
        cm: *mut f32,
        sn: *mut f32,
        ss: *mut f32,
        sint: *mut f32,
        d: *mut f32,
    );
    #[link_name = "i1merg_"]
    pub fn i1merg(
        icos: *mut f32,
        i1: *mut FortranInteger,
        m1: *mut FortranInteger,
        i2: *mut FortranInteger,
        m2: *mut FortranInteger,
        i3: *mut FortranInteger,
    );
    #[link_name = "intyd_"]
    pub fn intyd(
        t: *mut f32,
        k: *mut FortranInteger,
        yh: *mut f32,
        nyh: *mut FortranInteger,
        dky: *mut f32,
        iflag: *mut FortranInteger,
    );
    #[link_name = "ipperm_"]
    pub fn ipperm(
        ix: *mut FortranInteger,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "ipsort_"]
    pub fn ipsort(
        ix: *mut FortranInteger,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        kflag: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "isort_"]
    pub fn isort(
        ix: *mut FortranInteger,
        iy: *mut FortranInteger,
        n: *mut FortranInteger,
        kflag: *mut FortranInteger,
    );
    #[link_name = "la05ad_"]
    pub fn la05ad(
        a: *mut f64,
        ind: *mut FortranInteger,
        nz: *mut FortranInteger,
        ia: *mut FortranInteger,
        n: *mut FortranInteger,
        ip: *mut FortranInteger,
        iw: *mut FortranInteger,
        w: *mut f64,
        g: *mut f64,
        u: *mut f64,
    );
    #[link_name = "la05as_"]
    pub fn la05as(
        a: *mut f32,
        ind: *mut FortranInteger,
        nz: *mut FortranInteger,
        ia: *mut FortranInteger,
        n: *mut FortranInteger,
        ip: *mut FortranInteger,
        iw: *mut FortranInteger,
        w: *mut f32,
        g: *mut f32,
        u: *mut f32,
    );
    #[link_name = "la05cd_"]
    pub fn la05cd(
        a: *mut f64,
        ind: *mut FortranInteger,
        ia: *mut FortranInteger,
        n: *mut FortranInteger,
        ip: *mut FortranInteger,
        iw: *mut FortranInteger,
        w: *mut f64,
        g: *mut f64,
        u: *mut f64,
        mm: *mut FortranInteger,
    );
    #[link_name = "la05cs_"]
    pub fn la05cs(
        a: *mut f32,
        ind: *mut FortranInteger,
        ia: *mut FortranInteger,
        n: *mut FortranInteger,
        ip: *mut FortranInteger,
        iw: *mut FortranInteger,
        w: *mut f32,
        g: *mut f32,
        u: *mut f32,
        mm: *mut FortranInteger,
    );
    #[link_name = "lmpar_"]
    pub fn lmpar(
        n: *mut FortranInteger,
        r: *mut f32,
        ldr: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        diag: *mut f32,
        qtb: *mut f32,
        delta: *mut f32,
        par: *mut f32,
        x: *mut f32,
        sigma: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "lpdp_"]
    pub fn lpdp(
        a: *mut f32,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n1: *mut FortranInteger,
        n2: *mut FortranInteger,
        prgopt: *mut f32,
        x: *mut f32,
        wnorm: *mut f32,
        mode: *mut FortranInteger,
        ws: *mut f32,
        is: *mut FortranInteger,
    );
    #[link_name = "lsi_"]
    pub fn lsi(
        w: *mut f32,
        mdw: *mut FortranInteger,
        ma: *mut FortranInteger,
        mg: *mut FortranInteger,
        n: *mut FortranInteger,
        prgopt: *mut f32,
        x: *mut f32,
        rnorm: *mut f32,
        mode: *mut FortranInteger,
        ws: *mut f32,
        ip: *mut FortranInteger,
    );
    #[link_name = "lssods_"]
    pub fn lssods(
        a: *mut f32,
        x: *mut f32,
        b: *mut f32,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        q: *mut f32,
        diag: *mut f32,
        kpivot: *mut FortranInteger,
        iter: *mut FortranInteger,
        resnrm: *mut f32,
        xnorm: *mut f32,
        z: *mut f32,
        r: *mut f32,
        div: *mut f32,
        td: *mut f32,
        scales: *mut f32,
    );
    #[link_name = "lssuds_"]
    pub fn lssuds(
        a: *mut f32,
        x: *mut f32,
        b: *mut f32,
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        nrda: *mut FortranInteger,
        u: *mut f32,
        nrdu: *mut FortranInteger,
        iflag: *mut FortranInteger,
        mlso: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        q: *mut f32,
        diag: *mut f32,
        kpivot: *mut FortranInteger,
        s: *mut f32,
        div: *mut f32,
        td: *mut f32,
        isflg: *mut FortranInteger,
        scales: *mut f32,
    );
    #[link_name = "mc20ad_"]
    pub fn mc20ad(
        nc: *mut FortranInteger,
        maxa: *mut FortranInteger,
        a: *mut f64,
        inum: *mut FortranInteger,
        jptr: *mut FortranInteger,
        jnum: *mut FortranInteger,
        jdisp: *mut FortranInteger,
    );
    #[link_name = "mc20as_"]
    pub fn mc20as(
        nc: *mut FortranInteger,
        maxa: *mut FortranInteger,
        a: *mut f32,
        inum: *mut FortranInteger,
        jptr: *mut FortranInteger,
        jnum: *mut FortranInteger,
        jdisp: *mut FortranInteger,
    );
    #[link_name = "mgsbv_"]
    pub fn mgsbv(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f32,
        ia: *mut FortranInteger,
        niv: *mut FortranInteger,
        iflag: *mut FortranInteger,
        s: *mut f32,
        p: *mut f32,
        ip: *mut FortranInteger,
        inhomo: *mut FortranInteger,
        v: *mut f32,
        w: *mut f32,
        wcnd: *mut f32,
    );
    #[link_name = "minso4_"]
    pub fn minso4(
        usol: *mut f32,
        idmn: *mut FortranInteger,
        zn: *mut f32,
        zm: *mut f32,
        pertb: *mut f32,
    );
    #[link_name = "minsol_"]
    pub fn minsol(
        usol: *mut f32,
        idmn: *mut FortranInteger,
        zn: *mut f32,
        zm: *mut f32,
        pertb: *mut f32,
    );
    #[link_name = "mpadd_"]
    pub fn mpadd(x: *mut FortranInteger, y: *mut FortranInteger, z: *mut FortranInteger);
    #[link_name = "mpadd2_"]
    pub fn mpadd2(
        x: *mut FortranInteger,
        y: *mut FortranInteger,
        z: *mut FortranInteger,
        y1: *mut FortranInteger,
        trunc: *mut FortranInteger,
    );
    #[link_name = "mpadd3_"]
    pub fn mpadd3(
        x: *mut FortranInteger,
        y: *mut FortranInteger,
        s: *mut FortranInteger,
        med: *mut FortranInteger,
        re: *mut FortranInteger,
    );
    #[link_name = "mpcdm_"]
    pub fn mpcdm(dx: *mut f64, z: *mut FortranInteger);
    #[link_name = "mpcmd_"]
    pub fn mpcmd(x: *mut FortranInteger, dz: *mut f64);
    #[link_name = "mpdivi_"]
    pub fn mpdivi(x: *mut FortranInteger, iy: *mut FortranInteger, z: *mut FortranInteger);
    #[link_name = "mpmaxr_"]
    pub fn mpmaxr(x: *mut FortranInteger);
    #[link_name = "mpmlp_"]
    pub fn mpmlp(
        u: *mut FortranInteger,
        v: *mut FortranInteger,
        w: *mut FortranInteger,
        j: *mut FortranInteger,
    );
    #[link_name = "mpmul_"]
    pub fn mpmul(x: *mut FortranInteger, y: *mut FortranInteger, z: *mut FortranInteger);
    #[link_name = "mpmul2_"]
    pub fn mpmul2(
        x: *mut FortranInteger,
        iy: *mut FortranInteger,
        z: *mut FortranInteger,
        trunc: *mut FortranInteger,
    );
    #[link_name = "mpmuli_"]
    pub fn mpmuli(x: *mut FortranInteger, iy: *mut FortranInteger, z: *mut FortranInteger);
    #[link_name = "mpnzr_"]
    pub fn mpnzr(
        rs: *mut FortranInteger,
        re: *mut FortranInteger,
        z: *mut FortranInteger,
        trunc: *mut FortranInteger,
    );
    #[link_name = "mpovfl_"]
    pub fn mpovfl(x: *mut FortranInteger);
    #[link_name = "mpstr_"]
    pub fn mpstr(x: *mut FortranInteger, y: *mut FortranInteger);
    #[link_name = "mpunfl_"]
    pub fn mpunfl(x: *mut FortranInteger);
    #[link_name = "ohtrol_"]
    pub fn ohtrol(
        q: *mut f32,
        n: *mut FortranInteger,
        nrda: *mut FortranInteger,
        diag: *mut f32,
        irank: *mut FortranInteger,
        div: *mut f32,
        td: *mut f32,
    );
    #[link_name = "ohtror_"]
    pub fn ohtror(
        q: *mut f32,
        n: *mut FortranInteger,
        nrda: *mut FortranInteger,
        diag: *mut f32,
        irank: *mut FortranInteger,
        div: *mut f32,
        td: *mut f32,
    );
    #[link_name = "ortho4_"]
    pub fn ortho4(
        usol: *mut f32,
        idmn: *mut FortranInteger,
        zn: *mut f32,
        zm: *mut f32,
        pertrb: *mut f32,
    );
    #[link_name = "orthog_"]
    pub fn orthog(
        usol: *mut f32,
        idmn: *mut FortranInteger,
        zn: *mut f32,
        zm: *mut f32,
        pertrb: *mut f32,
    );
    #[link_name = "orthol_"]
    pub fn orthol(
        a: *mut f32,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        diag: *mut f32,
        kpivot: *mut FortranInteger,
        scales: *mut f32,
        cols: *mut f32,
        cs: *mut f32,
    );
    #[link_name = "orthor_"]
    pub fn orthor(
        a: *mut f32,
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        irank: *mut FortranInteger,
        iscale: *mut FortranInteger,
        diag: *mut f32,
        kpivot: *mut FortranInteger,
        scales: *mut f32,
        rows: *mut f32,
        rs: *mut f32,
    );
    #[link_name = "passb_"]
    pub fn passb(
        nac: *mut FortranInteger,
        ido: *mut FortranInteger,
        ip: *mut FortranInteger,
        l1: *mut FortranInteger,
        idl1: *mut FortranInteger,
        cc: *mut f32,
        c1: *mut f32,
        c2: *mut f32,
        ch: *mut f32,
        ch2: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "passb2_"]
    pub fn passb2(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
    );
    #[link_name = "passb3_"]
    pub fn passb3(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "passb4_"]
    pub fn passb4(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
    );
    #[link_name = "passb5_"]
    pub fn passb5(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );
    #[link_name = "passf_"]
    pub fn passf(
        nac: *mut FortranInteger,
        ido: *mut FortranInteger,
        ip: *mut FortranInteger,
        l1: *mut FortranInteger,
        idl1: *mut FortranInteger,
        cc: *mut f32,
        c1: *mut f32,
        c2: *mut f32,
        ch: *mut f32,
        ch2: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "passf2_"]
    pub fn passf2(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
    );
    #[link_name = "passf3_"]
    pub fn passf3(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "passf4_"]
    pub fn passf4(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
    );
    #[link_name = "passf5_"]
    pub fn passf5(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );
    #[link_name = "pchce_"]
    pub fn pchce(
        ic: *mut FortranInteger,
        vc: *mut f32,
        n: *mut FortranInteger,
        x: *mut f32,
        h: *mut f32,
        slope: *mut f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    #[link_name = "pchci_"]
    pub fn pchci(
        n: *mut FortranInteger,
        h: *mut f32,
        slope: *mut f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
    );
    #[link_name = "pchcs_"]
    pub fn pchcs(
        switch: *mut f32,
        n: *mut FortranInteger,
        h: *mut f32,
        slope: *mut f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    #[link_name = "pchkt_"]
    pub fn pchkt(n: *mut FortranInteger, x: *mut f32, knotyp: *mut FortranInteger, t: *mut f32);
    #[link_name = "pchngs_"]
    pub fn pchngs(
        ii: *mut FortranInteger,
        xval: *mut f32,
        iplace: *mut FortranInteger,
        sx: *mut f32,
        ix: *mut FortranInteger,
        ircx: *mut FortranInteger,
    );
    #[link_name = "pinitm_"]
    pub fn pinitm(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        sx: *mut f32,
        ix: *mut FortranInteger,
        lmx: *mut FortranInteger,
        ipagef: *mut FortranInteger,
    );
    #[link_name = "pnnzrs_"]
    pub fn pnnzrs(
        i: *mut FortranInteger,
        xval: *mut f32,
        iplace: *mut FortranInteger,
        sx: *mut f32,
        ix: *mut FortranInteger,
        ircx: *mut FortranInteger,
    );
    #[link_name = "poisd2_"]
    pub fn poisd2(
        mr: *mut FortranInteger,
        nr: *mut FortranInteger,
        istag: *mut FortranInteger,
        ba: *mut f32,
        bb: *mut f32,
        bc: *mut f32,
        q: *mut f32,
        idimq: *mut FortranInteger,
        b: *mut f32,
        w: *mut f32,
        d: *mut f32,
        tcos: *mut f32,
        p: *mut f32,
    );
    #[link_name = "poisn2_"]
    pub fn poisn2(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        istag: *mut FortranInteger,
        mixbnd: *mut FortranInteger,
        a: *mut f32,
        bb: *mut f32,
        c: *mut f32,
        q: *mut f32,
        idimq: *mut FortranInteger,
        b: *mut f32,
        b2: *mut f32,
        b3: *mut f32,
        w: *mut f32,
        w2: *mut f32,
        w3: *mut f32,
        d: *mut f32,
        tcos: *mut f32,
        p: *mut f32,
    );
    #[link_name = "poisp2_"]
    pub fn poisp2(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f32,
        bb: *mut f32,
        c: *mut f32,
        q: *mut f32,
        idimq: *mut FortranInteger,
        b: *mut f32,
        b2: *mut f32,
        b3: *mut f32,
        w: *mut f32,
        w2: *mut f32,
        w3: *mut f32,
        d: *mut f32,
        tcos: *mut f32,
        p: *mut f32,
    );
    #[link_name = "pos3d1_"]
    pub fn pos3d1(
        lp: *mut FortranInteger,
        l: *mut FortranInteger,
        mp: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        ldimf: *mut FortranInteger,
        mdimf: *mut FortranInteger,
        f: *mut f32,
        xrt: *mut f32,
        yrt: *mut f32,
        t: *mut f32,
        d: *mut f32,
        wx: *mut f32,
        wy: *mut f32,
        c1: *mut f32,
        c2: *mut f32,
        bb: *mut f32,
    );
    #[link_name = "postg2_"]
    pub fn postg2(
        nperod: *mut FortranInteger,
        n: *mut FortranInteger,
        m: *mut FortranInteger,
        a: *mut f32,
        bb: *mut f32,
        c: *mut f32,
        idimq: *mut FortranInteger,
        q: *mut f32,
        b: *mut f32,
        b2: *mut f32,
        b3: *mut f32,
        w: *mut f32,
        w2: *mut f32,
        w3: *mut f32,
        d: *mut f32,
        tcos: *mut f32,
        p: *mut f32,
    );
    #[link_name = "ppgq8_"]
    pub fn ppgq8(
        fun: *mut f32,
        ldc: *mut FortranInteger,
        c: *mut f32,
        xi: *mut f32,
        lxi: *mut FortranInteger,
        kk: *mut FortranInteger,
        id: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        inppv: *mut FortranInteger,
        err: *mut f32,
        ans: *mut f32,
        ierr: *mut FortranInteger,
    );
    #[link_name = "prod_"]
    pub fn prod(
        nd: *mut FortranInteger,
        bd: *mut f32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut f32,
        y: *mut f32,
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut f32,
        w: *mut f32,
        u: *mut f32,
    );
    #[link_name = "prodp_"]
    pub fn prodp(
        nd: *mut FortranInteger,
        bd: *mut f32,
        nm1: *mut FortranInteger,
        bm1: *mut f32,
        nm2: *mut FortranInteger,
        bm2: *mut f32,
        na: *mut FortranInteger,
        aa: *mut f32,
        x: *mut f32,
        y: *mut f32,
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut f32,
        u: *mut f32,
        w: *mut f32,
    );
    #[link_name = "prwpge_"]
    pub fn prwpge(
        key: *mut FortranInteger,
        ipage: *mut FortranInteger,
        lpg: *mut FortranInteger,
        sx: *mut f32,
        ix: *mut FortranInteger,
    );
    #[link_name = "prwvir_"]
    pub fn prwvir(
        key: *mut FortranInteger,
        ipage: *mut FortranInteger,
        lpg: *mut FortranInteger,
        sx: *mut f32,
        ix: *mut FortranInteger,
    );
    #[link_name = "qcheb_"]
    pub fn qcheb(x: *mut f32, fval: *mut f32, cheb12: *mut f32, cheb24: *mut f32);
    #[link_name = "qelg_"]
    pub fn qelg(
        n: *mut FortranInteger,
        epstab: *mut f32,
        result: *mut f32,
        abserr: *mut f32,
        res3la: *mut f32,
        nres: *mut FortranInteger,
    );
    #[link_name = "qform_"]
    pub fn qform(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f32,
        ldq: *mut FortranInteger,
        wa: *mut f32,
    );
    #[link_name = "qpsrt_"]
    pub fn qpsrt(
        limit: *mut FortranInteger,
        last: *mut FortranInteger,
        maxerr: *mut FortranInteger,
        ermax: *mut f32,
        elist: *mut f32,
        iord: *mut FortranInteger,
        nrmax: *mut FortranInteger,
    );
    #[link_name = "qrsolv_"]
    pub fn qrsolv(
        n: *mut FortranInteger,
        r: *mut f32,
        ldr: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        diag: *mut f32,
        qtb: *mut f32,
        x: *mut f32,
        sigma: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "qs2i1d_"]
    pub fn qs2i1d(
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f64,
        n: *mut FortranInteger,
        kflag: *mut FortranInteger,
    );
    #[link_name = "qs2i1r_"]
    pub fn qs2i1r(
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f32,
        n: *mut FortranInteger,
        kflag: *mut FortranInteger,
    );
    #[link_name = "r1mpyq_"]
    pub fn r1mpyq(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        a: *mut f32,
        lda: *mut FortranInteger,
        v: *mut f32,
        w: *mut f32,
    );
    #[link_name = "radb2_"]
    pub fn radb2(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
    );
    #[link_name = "radb3_"]
    pub fn radb3(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "radb4_"]
    pub fn radb4(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
    );
    #[link_name = "radb5_"]
    pub fn radb5(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );
    #[link_name = "radbg_"]
    pub fn radbg(
        ido: *mut FortranInteger,
        ip: *mut FortranInteger,
        l1: *mut FortranInteger,
        idl1: *mut FortranInteger,
        cc: *mut f32,
        c1: *mut f32,
        c2: *mut f32,
        ch: *mut f32,
        ch2: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "radf2_"]
    pub fn radf2(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
    );
    #[link_name = "radf3_"]
    pub fn radf3(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
    );
    #[link_name = "radf4_"]
    pub fn radf4(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
    );
    #[link_name = "radf5_"]
    pub fn radf5(
        ido: *mut FortranInteger,
        l1: *mut FortranInteger,
        cc: *mut f32,
        ch: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );
    #[link_name = "radfg_"]
    pub fn radfg(
        ido: *mut FortranInteger,
        ip: *mut FortranInteger,
        l1: *mut FortranInteger,
        idl1: *mut FortranInteger,
        cc: *mut f32,
        c1: *mut f32,
        c2: *mut f32,
        ch: *mut f32,
        ch2: *mut f32,
        wa: *mut f32,
    );
    #[link_name = "reort_"]
    pub fn reort(
        ncomp: *mut FortranInteger,
        y: *mut f32,
        yp: *mut f32,
        yhp: *mut f32,
        niv: *mut FortranInteger,
        w: *mut f32,
        s: *mut f32,
        p: *mut f32,
        ip: *mut FortranInteger,
        stowa: *mut f32,
        iflag: *mut FortranInteger,
    );
    #[link_name = "rfftb_"]
    pub fn rfftb(n: *mut FortranInteger, r: *mut f32, wsave: *mut f32);
    #[link_name = "rfftf_"]
    pub fn rfftf(n: *mut FortranInteger, r: *mut f32, wsave: *mut f32);
    #[link_name = "rffti_"]
    pub fn rffti(n: *mut FortranInteger, wsave: *mut f32);
    #[link_name = "rkfab_"]
    pub fn rkfab(
        ncomp: *mut FortranInteger,
        xpts: *mut f32,
        nxpts: *mut FortranInteger,
        nfc: *mut FortranInteger,
        iflag: *mut FortranInteger,
        z: *mut f32,
        mxnon: *mut FortranInteger,
        p: *mut f32,
        ntp: *mut FortranInteger,
        ip: *mut FortranInteger,
        yhp: *mut f32,
        niv: *mut FortranInteger,
        u: *mut f32,
        v: *mut f32,
        w: *mut f32,
        s: *mut f32,
        stowa: *mut f32,
        g: *mut f32,
        work: *mut f32,
        iwork: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "rsco_"]
    pub fn rsco(rsav: *mut f32, isav: *mut FortranInteger);
    #[link_name = "rwupdt_"]
    pub fn rwupdt(
        n: *mut FortranInteger,
        r: *mut f32,
        ldr: *mut FortranInteger,
        w: *mut f32,
        b: *mut f32,
        alpha: *mut f32,
        cos: *mut f32,
        sin: *mut f32,
    );
    #[link_name = "s1merg_"]
    pub fn s1merg(
        tcos: *mut f32,
        i1: *mut FortranInteger,
        m1: *mut FortranInteger,
        i2: *mut FortranInteger,
        m2: *mut FortranInteger,
        i3: *mut FortranInteger,
    );
    #[link_name = "sbhin_"]
    pub fn sbhin(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f32,
        isym: *mut FortranInteger,
        soln: *mut f32,
        rhs: *mut f32,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "sbolsm_"]
    pub fn sbolsm(
        w: *mut f32,
        mdw: *mut FortranInteger,
        minput: *mut FortranInteger,
        ncols: *mut FortranInteger,
        bl: *mut f32,
        bu: *mut f32,
        ind: *mut FortranInteger,
        iopt: *mut FortranInteger,
        x: *mut f32,
        rnorm: *mut f32,
        mode: *mut FortranInteger,
        rw: *mut f32,
        ww: *mut f32,
        scl: *mut f32,
        ibasis: *mut FortranInteger,
        ibb: *mut FortranInteger,
    );
    #[link_name = "scoef_"]
    pub fn scoef(
        yh: *mut f32,
        yp: *mut f32,
        ncomp: *mut FortranInteger,
        nrowb: *mut FortranInteger,
        nfc: *mut FortranInteger,
        nic: *mut FortranInteger,
        b: *mut f32,
        beta: *mut f32,
        coef: *mut f32,
        inhomo: *mut FortranInteger,
        re: *mut f32,
        ae: *mut f32,
        by: *mut f32,
        cvec: *mut f32,
        work: *mut f32,
        iwork: *mut FortranInteger,
        iflag: *mut FortranInteger,
        nfcc: *mut FortranInteger,
    );
    #[link_name = "scpplt_"]
    pub fn scpplt(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f32,
        isym: *mut FortranInteger,
        iunit: *mut FortranInteger,
    );
    #[link_name = "sdaslv_"]
    pub fn sdaslv(
        neq: *mut FortranInteger,
        delta: *mut f32,
        wm: *mut f32,
        iwm: *mut FortranInteger,
    );
    #[link_name = "sdatrp_"]
    pub fn sdatrp(
        x: *mut f32,
        xout: *mut f32,
        yout: *mut f32,
        ypout: *mut f32,
        neq: *mut FortranInteger,
        kold: *mut FortranInteger,
        phi: *mut f32,
        psi: *mut f32,
    );
    #[link_name = "sdawts_"]
    pub fn sdawts(
        neq: *mut FortranInteger,
        iwt: *mut FortranInteger,
        rtol: *mut f32,
        atol: *mut f32,
        y: *mut f32,
        wt: *mut f32,
        rpar: *mut f32,
        ipar: *mut FortranInteger,
    );
    #[link_name = "sdcst_"]
    pub fn sdcst(
        maxord: *mut FortranInteger,
        mint: *mut FortranInteger,
        iswflg: *mut FortranInteger,
        el: *mut f32,
        tq: *mut f32,
    );
    #[link_name = "sdntp_"]
    pub fn sdntp(
        h: *mut f32,
        k: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        t: *mut f32,
        tout: *mut f32,
        yh: *mut f32,
        y: *mut f32,
    );
    #[link_name = "sdpsc_"]
    pub fn sdpsc(
        ksgn: *mut FortranInteger,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        yh: *mut f32,
    );
    #[link_name = "sdscl_"]
    pub fn sdscl(
        hmax: *mut f32,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        rmax: *mut f32,
        h: *mut f32,
        rc: *mut f32,
        rh: *mut f32,
        yh: *mut f32,
    );
    #[link_name = "sdzro_"]
    pub fn sdzro(
        ae: *mut f32,
        f: *mut f32,
        h: *mut f32,
        n: *mut FortranInteger,
        nq: *mut FortranInteger,
        iroot: *mut FortranInteger,
        re: *mut f32,
        t: *mut f32,
        yh: *mut f32,
        uround: *mut f32,
        b: *mut f32,
        c: *mut f32,
        fb: *mut f32,
        fc: *mut f32,
        y: *mut f32,
    );
    #[link_name = "shels_"]
    pub fn shels(
        a: *mut f32,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f32,
        b: *mut f32,
    );
    #[link_name = "sheqr_"]
    pub fn sheqr(
        a: *mut f32,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        q: *mut f32,
        info: *mut FortranInteger,
        ijob: *mut FortranInteger,
    );
    #[link_name = "slvs_"]
    pub fn slvs(wm: *mut f32, iwm: *mut FortranInteger, x: *mut f32, tem: *mut f32);
    #[link_name = "sods_"]
    pub fn sods(
        a: *mut f32,
        x: *mut f32,
        b: *mut f32,
        neq: *mut FortranInteger,
        nuk: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        work: *mut f32,
        iwork: *mut FortranInteger,
    );
    #[link_name = "sorth_"]
    pub fn sorth(
        vnew: *mut f32,
        v: *mut f32,
        hes: *mut f32,
        n: *mut FortranInteger,
        ll: *mut FortranInteger,
        ldhes: *mut FortranInteger,
        kmp: *mut FortranInteger,
        snormw: *mut f32,
    );
    #[link_name = "soseqs_"]
    pub fn soseqs(
        fnc: *mut f32,
        n: *mut FortranInteger,
        s: *mut f32,
        rtolx: *mut f32,
        atolx: *mut f32,
        tolf: *mut f32,
        iflag: *mut FortranInteger,
        mxit: *mut FortranInteger,
        ncjs: *mut FortranInteger,
        nsrrc: *mut FortranInteger,
        nsri: *mut FortranInteger,
        iprint: *mut FortranInteger,
        fmax: *mut f32,
        c: *mut f32,
        nc: *mut FortranInteger,
        b: *mut f32,
        p: *mut f32,
        temp: *mut f32,
        x: *mut f32,
        y: *mut f32,
        fac: *mut f32,
        is: *mut FortranInteger,
    );
    #[link_name = "sossol_"]
    pub fn sossol(
        k: *mut FortranInteger,
        n: *mut FortranInteger,
        l: *mut FortranInteger,
        x: *mut f32,
        c: *mut f32,
        b: *mut f32,
        m: *mut FortranInteger,
    );
    #[link_name = "spperm_"]
    pub fn spperm(
        x: *mut f32,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "spsort_"]
    pub fn spsort(
        x: *mut f32,
        n: *mut FortranInteger,
        iperm: *mut FortranInteger,
        kflag: *mut FortranInteger,
        ier: *mut FortranInteger,
    );
    #[link_name = "sreadp_"]
    pub fn sreadp(
        ipage: *mut FortranInteger,
        list: *mut FortranInteger,
        rlist: *mut f32,
        lpage: *mut FortranInteger,
        irec: *mut FortranInteger,
    );
    #[link_name = "srlcal_"]
    pub fn srlcal(
        n: *mut FortranInteger,
        kmp: *mut FortranInteger,
        ll: *mut FortranInteger,
        maxl: *mut FortranInteger,
        v: *mut f32,
        q: *mut f32,
        rl: *mut f32,
        snormw: *mut f32,
        prod: *mut f32,
        r0nrm: *mut f32,
    );
    #[link_name = "ssort_"]
    pub fn ssort(x: *mut f32, y: *mut f32, n: *mut FortranInteger, kflag: *mut FortranInteger);
    #[link_name = "stin_"]
    pub fn stin(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f32,
        isym: *mut FortranInteger,
        soln: *mut f32,
        rhs: *mut f32,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "stor1_"]
    pub fn stor1(
        u: *mut f32,
        yh: *mut f32,
        v: *mut f32,
        yp: *mut f32,
        ntemp: *mut FortranInteger,
        ndisk: *mut FortranInteger,
        ntape: *mut FortranInteger,
    );
    #[link_name = "stout_"]
    pub fn stout(
        n: *mut FortranInteger,
        nelt: *mut FortranInteger,
        ia: *mut FortranInteger,
        ja: *mut FortranInteger,
        a: *mut f32,
        isym: *mut FortranInteger,
        soln: *mut f32,
        rhs: *mut f32,
        iunit: *mut FortranInteger,
        job: *mut FortranInteger,
    );
    #[link_name = "stway_"]
    pub fn stway(
        u: *mut f32,
        v: *mut f32,
        yhp: *mut f32,
        inout: *mut FortranInteger,
        stowa: *mut f32,
    );
    #[link_name = "suds_"]
    pub fn suds(
        a: *mut f32,
        x: *mut f32,
        b: *mut f32,
        neq: *mut FortranInteger,
        nuk: *mut FortranInteger,
        nrda: *mut FortranInteger,
        iflag: *mut FortranInteger,
        mlso: *mut FortranInteger,
        work: *mut f32,
        iwork: *mut FortranInteger,
    );
    #[link_name = "svco_"]
    pub fn svco(rsav: *mut f32, isav: *mut FortranInteger);
    #[link_name = "svecs_"]
    pub fn svecs(
        ncomp: *mut FortranInteger,
        lnfc: *mut FortranInteger,
        yhp: *mut f32,
        work: *mut f32,
        iwork: *mut FortranInteger,
        inhomo: *mut FortranInteger,
        iflag: *mut FortranInteger,
    );
    #[link_name = "swritp_"]
    pub fn swritp(
        ipage: *mut FortranInteger,
        list: *mut FortranInteger,
        rlist: *mut f32,
        lpage: *mut FortranInteger,
        irec: *mut FortranInteger,
    );
    #[link_name = "tevlc_"]
    pub fn tevlc(n: *mut FortranInteger, d: *mut f32, e2: *mut f32, ierr: *mut FortranInteger);
    #[link_name = "tevls_"]
    pub fn tevls(n: *mut FortranInteger, d: *mut f32, e2: *mut f32, ierr: *mut FortranInteger);
    #[link_name = "tri3_"]
    pub fn tri3(
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        k: *mut FortranInteger,
        y1: *mut f32,
        y2: *mut f32,
        y3: *mut f32,
        tcos: *mut f32,
        d: *mut f32,
        w1: *mut f32,
        w2: *mut f32,
        w3: *mut f32,
    );
    #[link_name = "tridq_"]
    pub fn tridq(
        mr: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        y: *mut f32,
        d: *mut f32,
    );
    #[link_name = "tris4_"]
    pub fn tris4(
        n: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut f32,
        u: *mut f32,
        z: *mut f32,
    );
    #[link_name = "trisp_"]
    pub fn trisp(
        n: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut f32,
        u: *mut f32,
        z: *mut f32,
    );
    #[link_name = "trix_"]
    pub fn trix(
        idegbr: *mut FortranInteger,
        idegcr: *mut FortranInteger,
        m: *mut FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        y: *mut f32,
        tcos: *mut f32,
        d: *mut f32,
        w: *mut f32,
    );
    #[link_name = "u11ls_"]
    pub fn u11ls(
        a: *mut f32,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        ub: *mut f32,
        db: *mut f32,
        mode: *mut FortranInteger,
        np: *mut FortranInteger,
        krank: *mut FortranInteger,
        ksure: *mut FortranInteger,
        h: *mut f32,
        w: *mut f32,
        eb: *mut f32,
        ic: *mut FortranInteger,
        ir: *mut FortranInteger,
    );
    #[link_name = "u11us_"]
    pub fn u11us(
        a: *mut f32,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        ub: *mut f32,
        db: *mut f32,
        mode: *mut FortranInteger,
        np: *mut FortranInteger,
        krank: *mut FortranInteger,
        ksure: *mut FortranInteger,
        h: *mut f32,
        w: *mut f32,
        eb: *mut f32,
        ir: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "u12ls_"]
    pub fn u12ls(
        a: *mut f32,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        b: *mut f32,
        mdb: *mut FortranInteger,
        nb: *mut FortranInteger,
        mode: *mut FortranInteger,
        krank: *mut FortranInteger,
        rnorm: *mut f32,
        h: *mut f32,
        w: *mut f32,
        ic: *mut FortranInteger,
        ir: *mut FortranInteger,
    );
    #[link_name = "u12us_"]
    pub fn u12us(
        a: *mut f32,
        mda: *mut FortranInteger,
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        b: *mut f32,
        mdb: *mut FortranInteger,
        nb: *mut FortranInteger,
        mode: *mut FortranInteger,
        krank: *mut FortranInteger,
        rnorm: *mut f32,
        h: *mut f32,
        w: *mut f32,
        ir: *mut FortranInteger,
        ic: *mut FortranInteger,
    );
    #[link_name = "usrmat_"]
    pub fn usrmat(
        i: *mut FortranInteger,
        j: *mut FortranInteger,
        aij: *mut f32,
        indcat: *mut FortranInteger,
        prgopt: *mut f32,
        dattrv: *mut f32,
        iflag: *mut FortranInteger,
    );
    #[link_name = "wnlsm_"]
    pub fn wnlsm(
        w: *mut f32,
        mdw: *mut FortranInteger,
        mme: *mut FortranInteger,
        ma: *mut FortranInteger,
        n: *mut FortranInteger,
        l: *mut FortranInteger,
        prgopt: *mut f32,
        x: *mut f32,
        rnorm: *mut f32,
        mode: *mut FortranInteger,
        ipivot: *mut FortranInteger,
        itype: *mut FortranInteger,
        wd: *mut f32,
        h: *mut f32,
        scale: *mut f32,
        z: *mut f32,
        temp: *mut f32,
        d: *mut f32,
    );
    #[link_name = "wnlt3_"]
    pub fn wnlt3(
        i: *mut FortranInteger,
        imax: *mut FortranInteger,
        m: *mut FortranInteger,
        mdw: *mut FortranInteger,
        ipivot: *mut FortranInteger,
        h: *mut f32,
        w: *mut f32,
    );
    #[link_name = "xpmu_"]
    pub fn xpmu(
        nu1: *mut f32,
        nu2: *mut f32,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        theta: *mut f32,
        x: *mut f32,
        sx: *mut f32,
        id: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xpmup_"]
    pub fn xpmup(
        nu1: *mut f32,
        nu2: *mut f32,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xpnrm_"]
    pub fn xpnrm(
        nu1: *mut f32,
        nu2: *mut f32,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xpqnu_"]
    pub fn xpqnu(
        nu1: *mut f32,
        nu2: *mut f32,
        mu: *mut FortranInteger,
        theta: *mut f32,
        id: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xqmu_"]
    pub fn xqmu(
        nu1: *mut f32,
        nu2: *mut f32,
        mu1: *mut FortranInteger,
        mu2: *mut FortranInteger,
        theta: *mut f32,
        x: *mut f32,
        sx: *mut f32,
        id: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "xqnu_"]
    pub fn xqnu(
        nu1: *mut f32,
        nu2: *mut f32,
        mu1: *mut FortranInteger,
        theta: *mut f32,
        x: *mut f32,
        sx: *mut f32,
        id: *mut FortranInteger,
        pqa: *mut f32,
        ipqa: *mut FortranInteger,
        ierror: *mut FortranInteger,
    );
    #[link_name = "zacai_"]
    pub fn zacai(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        rl: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zacon_"]
    pub fn zacon(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        rl: *mut f64,
        fnul: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zasyi_"]
    pub fn zasyi(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        rl: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zbinu_"]
    pub fn zbinu(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        cyr: *mut f64,
        cyi: *mut f64,
        nz: *mut FortranInteger,
        rl: *mut f64,
        fnul: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zbknu_"]
    pub fn zbknu(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zbuni_"]
    pub fn zbuni(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        nui: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zbunk_"]
    pub fn zbunk(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zkscl_"]
    pub fn zkscl(
        zrr: *mut f64,
        zri: *mut f64,
        fnu: *mut f64,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        rzr: *mut f64,
        rzi: *mut f64,
        ascle: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
    );
    #[link_name = "zmlri_"]
    pub fn zmlri(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
    );
    #[link_name = "zrati_"]
    pub fn zrati(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        n: *mut FortranInteger,
        cyr: *mut f64,
        cyi: *mut f64,
        tol: *mut f64,
    );
    #[link_name = "zseri_"]
    pub fn zseri(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zuni1_"]
    pub fn zuni1(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zuni2_"]
    pub fn zuni2(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        nlast: *mut FortranInteger,
        fnul: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zunik_"]
    pub fn zunik(
        zrr: *mut f64,
        zri: *mut f64,
        fnu: *mut f64,
        ikflg: *mut FortranInteger,
        ipmtr: *mut FortranInteger,
        tol: *mut f64,
        init: *mut FortranInteger,
        phir: *mut f64,
        phii: *mut f64,
        zeta1r: *mut f64,
        zeta1i: *mut f64,
        zeta2r: *mut f64,
        zeta2i: *mut f64,
        sumr: *mut f64,
        sumi: *mut f64,
        cwrkr: *mut f64,
        cwrki: *mut f64,
    );
    #[link_name = "zunk1_"]
    pub fn zunk1(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zunk2_"]
    pub fn zunk2(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        mr: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zuoik_"]
    pub fn zuoik(
        zr: *mut f64,
        zi: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        ikflg: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nuf: *mut FortranInteger,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
    #[link_name = "zwrsk_"]
    pub fn zwrsk(
        zrr: *mut f64,
        zri: *mut f64,
        fnu: *mut f64,
        kode: *mut FortranInteger,
        n: *mut FortranInteger,
        yr: *mut f64,
        yi: *mut f64,
        nz: *mut FortranInteger,
        cwr: *mut f64,
        cwi: *mut f64,
        tol: *mut f64,
        elim: *mut f64,
        alim: *mut f64,
    );
}

// ffi-declaration-aliases:start
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dbocls`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbocls.md"))]
pub use crate::approximation::dbocls;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dbols`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbols.md"))]
pub use crate::approximation::dbols;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::defc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/defc.md"))]
pub use crate::approximation::defc;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dfc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dfc.md"))]
pub use crate::approximation::dfc;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dlsei`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dlsei.md"))]
pub use crate::approximation::dlsei;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dp1vlu`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dp1vlu.md"))]
pub use crate::approximation::dp1vlu;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dpcoef`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpcoef.md"))]
pub use crate::approximation::dpcoef;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dpolft`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpolft.md"))]
pub use crate::approximation::dpolft;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::dwnnls`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dwnnls.md"))]
pub use crate::approximation::dwnnls;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::efc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/efc.md"))]
pub use crate::approximation::efc;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::fc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/fc.md"))]
pub use crate::approximation::fc;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::lsei`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/lsei.md"))]
pub use crate::approximation::lsei;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::pcoef`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pcoef.md"))]
pub use crate::approximation::pcoef;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::polfit`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/polfit.md"))]
pub use crate::approximation::polfit;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::pvalue`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pvalue.md"))]
pub use crate::approximation::pvalue;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::sbocls`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sbocls.md"))]
pub use crate::approximation::sbocls;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::sbols`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sbols.md"))]
pub use crate::approximation::sbols;
#[doc = "Transitional ABI-shaped alias; use `crate::approximation::wnnls`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/wnnls.md"))]
pub use crate::approximation::wnnls;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::daxpy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/daxpy.md"))]
pub use crate::blas::level1::daxpy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dcopy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcopy.md"))]
pub use crate::blas::level1::dcopy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dcopym`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcopym.md"))]
pub use crate::blas::level1::dcopym;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::drot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drot.md"))]
pub use crate::blas::level1::drot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::drotm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drotm.md"))]
pub use crate::blas::level1::drotm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::drotmg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drotmg.md"))]
pub use crate::blas::level1::drotmg;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ds2y`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ds2y.md"))]
pub use crate::blas::level1::ds2y;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dscal`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dscal.md"))]
pub use crate::blas::level1::dscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dsdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdi.md"))]
pub use crate::blas::level1::dsdi;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dsmtv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsmtv.md"))]
pub use crate::blas::level1::dsmtv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dsmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsmv.md"))]
pub use crate::blas::level1::dsmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::dswap`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dswap.md"))]
pub use crate::blas::level1::dswap;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::icopy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/icopy.md"))]
pub use crate::blas::level1::icopy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::iswap`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/iswap.md"))]
pub use crate::blas::level1::iswap;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::saxpy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/saxpy.md"))]
pub use crate::blas::level1::saxpy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scopy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/scopy.md"))]
pub use crate::blas::level1::scopy;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::scopym`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/scopym.md"))]
pub use crate::blas::level1::scopym;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::srot`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/srot.md"))]
pub use crate::blas::level1::srot;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::srotm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/srotm.md"))]
pub use crate::blas::level1::srotm;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::srotmg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/srotmg.md"))]
pub use crate::blas::level1::srotmg;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ss2y`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ss2y.md"))]
pub use crate::blas::level1::ss2y;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::sscal`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sscal.md"))]
pub use crate::blas::level1::sscal;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ssdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdi.md"))]
pub use crate::blas::level1::ssdi;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ssmtv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssmtv.md"))]
pub use crate::blas::level1::ssmtv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::ssmv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssmv.md"))]
pub use crate::blas::level1::ssmv;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level1::sswap`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sswap.md"))]
pub use crate::blas::level1::sswap;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::dger`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dger.md"))]
pub use crate::blas::level2::dger;
#[doc = "Transitional ABI-shaped alias; use `crate::blas::level2::sger`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sger.md"))]
pub use crate::blas::level2::sger;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cfftb1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cfftb1.md"))]
pub use crate::fftpack::cfftb1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cfftf1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cfftf1.md"))]
pub use crate::fftpack::cfftf1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cffti1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cffti1.md"))]
pub use crate::fftpack::cffti1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cosqb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cosqb.md"))]
pub use crate::fftpack::cosqb;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cosqf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cosqf.md"))]
pub use crate::fftpack::cosqf;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cosqi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cosqi.md"))]
pub use crate::fftpack::cosqi;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::cost`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cost.md"))]
pub use crate::fftpack::cost;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::costi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/costi.md"))]
pub use crate::fftpack::costi;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::ezfftb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ezfftb.md"))]
pub use crate::fftpack::ezfftb;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::ezfftf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ezfftf.md"))]
pub use crate::fftpack::ezfftf;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::ezffti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ezffti.md"))]
pub use crate::fftpack::ezffti;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::rfftb1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rfftb1.md"))]
pub use crate::fftpack::rfftb1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::rfftf1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rfftf1.md"))]
pub use crate::fftpack::rfftf1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::rffti1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rffti1.md"))]
pub use crate::fftpack::rffti1;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::sinqb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sinqb.md"))]
pub use crate::fftpack::sinqb;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::sinqf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sinqf.md"))]
pub use crate::fftpack::sinqf;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::sinqi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sinqi.md"))]
pub use crate::fftpack::sinqi;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::sint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sint.md"))]
pub use crate::fftpack::sint;
#[doc = "Transitional ABI-shaped alias; use `crate::fftpack::sinti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sinti.md"))]
pub use crate::fftpack::sinti;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bint4`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bint4.md"))]
pub use crate::interpolation::bint4;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bintk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bintk.md"))]
pub use crate::interpolation::bintk;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bspdr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bspdr.md"))]
pub use crate::interpolation::bspdr;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bspev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bspev.md"))]
pub use crate::interpolation::bspev;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bsppp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bsppp.md"))]
pub use crate::interpolation::bsppp;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bspvd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bspvd.md"))]
pub use crate::interpolation::bspvd;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::bspvn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bspvn.md"))]
pub use crate::interpolation::bspvn;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::chfdv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chfdv.md"))]
pub use crate::interpolation::chfdv;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::chfev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chfev.md"))]
pub use crate::interpolation::chfev;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbint4`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbint4.md"))]
pub use crate::interpolation::dbint4;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbintk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbintk.md"))]
pub use crate::interpolation::dbintk;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbspdr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbspdr.md"))]
pub use crate::interpolation::dbspdr;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbspev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbspev.md"))]
pub use crate::interpolation::dbspev;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbsppp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsppp.md"))]
pub use crate::interpolation::dbsppp;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbspvd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbspvd.md"))]
pub use crate::interpolation::dbspvd;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dbspvn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbspvn.md"))]
pub use crate::interpolation::dbspvn;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dchfdv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchfdv.md"))]
pub use crate::interpolation::dchfdv;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dchfev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchfev.md"))]
pub use crate::interpolation::dchfev;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dintrv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dintrv.md"))]
pub use crate::interpolation::dintrv;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpchbs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpchbs.md"))]
pub use crate::interpolation::dpchbs;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpchic`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpchic.md"))]
pub use crate::interpolation::dpchic;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpchim`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpchim.md"))]
pub use crate::interpolation::dpchim;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpchsp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpchsp.md"))]
pub use crate::interpolation::dpchsp;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dplint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dplint.md"))]
pub use crate::interpolation::dplint;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpolcf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpolcf.md"))]
pub use crate::interpolation::dpolcf;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::dpolvl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpolvl.md"))]
pub use crate::interpolation::dpolvl;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::intrv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/intrv.md"))]
pub use crate::interpolation::intrv;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::pchbs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pchbs.md"))]
pub use crate::interpolation::pchbs;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::pchic`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pchic.md"))]
pub use crate::interpolation::pchic;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::pchim`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pchim.md"))]
pub use crate::interpolation::pchim;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::pchsp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pchsp.md"))]
pub use crate::interpolation::pchsp;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::polcof`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/polcof.md"))]
pub use crate::interpolation::polcof;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::polint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/polint.md"))]
pub use crate::interpolation::polint;
#[doc = "Transitional ABI-shaped alias; use `crate::interpolation::polyvl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/polyvl.md"))]
pub use crate::interpolation::polyvl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::bndacc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bndacc.md"))]
pub use crate::linear_algebra::banded::bndacc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::bndsol`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bndsol.md"))]
pub use crate::linear_algebra::banded::bndsol;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dbndac`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbndac.md"))]
pub use crate::linear_algebra::banded::dbndac;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dbndsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbndsl.md"))]
pub use crate::linear_algebra::banded::dbndsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dgbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbco.md"))]
pub use crate::linear_algebra::banded::dgbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dgbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbdi.md"))]
pub use crate::linear_algebra::banded::dgbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dgbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbfa.md"))]
pub use crate::linear_algebra::banded::dgbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dgbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbsl.md"))]
pub use crate::linear_algebra::banded::dgbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dgtsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgtsl.md"))]
pub use crate::linear_algebra::banded::dgtsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dhfti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dhfti.md"))]
pub use crate::linear_algebra::banded::dhfti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dnbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnbco.md"))]
pub use crate::linear_algebra::banded::dnbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dnbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnbdi.md"))]
pub use crate::linear_algebra::banded::dnbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dnbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnbfa.md"))]
pub use crate::linear_algebra::banded::dnbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dnbfs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnbfs.md"))]
pub use crate::linear_algebra::banded::dnbfs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dnbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnbsl.md"))]
pub use crate::linear_algebra::banded::dnbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dpbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpbco.md"))]
pub use crate::linear_algebra::banded::dpbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dpbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpbdi.md"))]
pub use crate::linear_algebra::banded::dpbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dpbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpbfa.md"))]
pub use crate::linear_algebra::banded::dpbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dpbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpbsl.md"))]
pub use crate::linear_algebra::banded::dpbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::dptsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dptsl.md"))]
pub use crate::linear_algebra::banded::dptsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sgbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbco.md"))]
pub use crate::linear_algebra::banded::sgbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sgbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbdi.md"))]
pub use crate::linear_algebra::banded::sgbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sgbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbfa.md"))]
pub use crate::linear_algebra::banded::sgbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sgbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbsl.md"))]
pub use crate::linear_algebra::banded::sgbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sgtsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgtsl.md"))]
pub use crate::linear_algebra::banded::sgtsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbco.md"))]
pub use crate::linear_algebra::banded::snbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbdi.md"))]
pub use crate::linear_algebra::banded::snbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbfa.md"))]
pub use crate::linear_algebra::banded::snbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbfs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbfs.md"))]
pub use crate::linear_algebra::banded::snbfs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbir.md"))]
pub use crate::linear_algebra::banded::snbir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::snbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snbsl.md"))]
pub use crate::linear_algebra::banded::snbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::spbco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spbco.md"))]
pub use crate::linear_algebra::banded::spbco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::spbdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spbdi.md"))]
pub use crate::linear_algebra::banded::spbdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::spbfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spbfa.md"))]
pub use crate::linear_algebra::banded::spbfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::spbsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spbsl.md"))]
pub use crate::linear_algebra::banded::spbsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::banded::sptsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sptsl.md"))]
pub use crate::linear_algebra::banded::sptsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dchdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchdc.md"))]
pub use crate::linear_algebra::dense::dchdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dchdd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchdd.md"))]
pub use crate::linear_algebra::dense::dchdd;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dchex`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchex.md"))]
pub use crate::linear_algebra::dense::dchex;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dchud`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dchud.md"))]
pub use crate::linear_algebra::dense::dchud;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dgeco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgeco.md"))]
pub use crate::linear_algebra::dense::dgeco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dgedi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgedi.md"))]
pub use crate::linear_algebra::dense::dgedi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dgefa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgefa.md"))]
pub use crate::linear_algebra::dense::dgefa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dgefs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgefs.md"))]
pub use crate::linear_algebra::dense::dgefs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dgesl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgesl.md"))]
pub use crate::linear_algebra::dense::dgesl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dglss`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dglss.md"))]
pub use crate::linear_algebra::dense::dglss;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dllsia`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dllsia.md"))]
pub use crate::linear_algebra::dense::dllsia;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dllti2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dllti2.md"))]
pub use crate::linear_algebra::dense::dllti2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dpoco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpoco.md"))]
pub use crate::linear_algebra::dense::dpoco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dpodi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpodi.md"))]
pub use crate::linear_algebra::dense::dpodi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dpofa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpofa.md"))]
pub use crate::linear_algebra::dense::dpofa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dpofs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpofs.md"))]
pub use crate::linear_algebra::dense::dpofs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dposl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dposl.md"))]
pub use crate::linear_algebra::dense::dposl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dqrdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqrdc.md"))]
pub use crate::linear_algebra::dense::dqrdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dqrsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqrsl.md"))]
pub use crate::linear_algebra::dense::dqrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ds2lt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ds2lt.md"))]
pub use crate::linear_algebra::dense::ds2lt;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsd2s`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsd2s.md"))]
pub use crate::linear_algebra::dense::dsd2s;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsds`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsds.md"))]
pub use crate::linear_algebra::dense::dsds;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsdscl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdscl.md"))]
pub use crate::linear_algebra::dense::dsdscl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsico`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsico.md"))]
pub use crate::linear_algebra::dense::dsico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsics`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsics.md"))]
pub use crate::linear_algebra::dense::dsics;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsidi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsidi.md"))]
pub use crate::linear_algebra::dense::dsidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsifa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsifa.md"))]
pub use crate::linear_algebra::dense::dsifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsilus`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsilus.md"))]
pub use crate::linear_algebra::dense::dsilus;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsisl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsisl.md"))]
pub use crate::linear_algebra::dense::dsisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsli`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsli.md"))]
pub use crate::linear_algebra::dense::dsli;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsli2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsli2.md"))]
pub use crate::linear_algebra::dense::dsli2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsllti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsllti.md"))]
pub use crate::linear_algebra::dense::dsllti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dslucs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslucs.md"))]
pub use crate::linear_algebra::dense::dslucs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dslui`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslui.md"))]
pub use crate::linear_algebra::dense::dslui;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dslui2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslui2.md"))]
pub use crate::linear_algebra::dense::dslui2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dslui4`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslui4.md"))]
pub use crate::linear_algebra::dense::dslui4;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsluti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsluti.md"))]
pub use crate::linear_algebra::dense::dsluti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsmmi2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsmmi2.md"))]
pub use crate::linear_algebra::dense::dsmmi2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsmmti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsmmti.md"))]
pub use crate::linear_algebra::dense::dsmmti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dsvdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsvdc.md"))]
pub use crate::linear_algebra::dense::dsvdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dtrco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrco.md"))]
pub use crate::linear_algebra::dense::dtrco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dtrdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrdi.md"))]
pub use crate::linear_algebra::dense::dtrdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dtrsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dtrsl.md"))]
pub use crate::linear_algebra::dense::dtrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::dulsia`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dulsia.md"))]
pub use crate::linear_algebra::dense::dulsia;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::hfti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hfti.md"))]
pub use crate::linear_algebra::dense::hfti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::llsia`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/llsia.md"))]
pub use crate::linear_algebra::dense::llsia;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::minfit`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/minfit.md"))]
pub use crate::linear_algebra::dense::minfit;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::schdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/schdc.md"))]
pub use crate::linear_algebra::dense::schdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::schdd`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/schdd.md"))]
pub use crate::linear_algebra::dense::schdd;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::schex`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/schex.md"))]
pub use crate::linear_algebra::dense::schex;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::schud`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/schud.md"))]
pub use crate::linear_algebra::dense::schud;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgeco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgeco.md"))]
pub use crate::linear_algebra::dense::sgeco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgedi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgedi.md"))]
pub use crate::linear_algebra::dense::sgedi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgefa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgefa.md"))]
pub use crate::linear_algebra::dense::sgefa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgefs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgefs.md"))]
pub use crate::linear_algebra::dense::sgefs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgeir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgeir.md"))]
pub use crate::linear_algebra::dense::sgeir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sgesl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgesl.md"))]
pub use crate::linear_algebra::dense::sgesl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sglss`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sglss.md"))]
pub use crate::linear_algebra::dense::sglss;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sllti2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sllti2.md"))]
pub use crate::linear_algebra::dense::sllti2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::spoco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spoco.md"))]
pub use crate::linear_algebra::dense::spoco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::spodi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spodi.md"))]
pub use crate::linear_algebra::dense::spodi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::spofa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spofa.md"))]
pub use crate::linear_algebra::dense::spofa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::spofs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spofs.md"))]
pub use crate::linear_algebra::dense::spofs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::spoir`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spoir.md"))]
pub use crate::linear_algebra::dense::spoir;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sposl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sposl.md"))]
pub use crate::linear_algebra::dense::sposl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sqrdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sqrdc.md"))]
pub use crate::linear_algebra::dense::sqrdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sqrsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sqrsl.md"))]
pub use crate::linear_algebra::dense::sqrsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ss2lt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ss2lt.md"))]
pub use crate::linear_algebra::dense::ss2lt;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssd2s`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssd2s.md"))]
pub use crate::linear_algebra::dense::ssd2s;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssds`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssds.md"))]
pub use crate::linear_algebra::dense::ssds;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssdscl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdscl.md"))]
pub use crate::linear_algebra::dense::ssdscl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssico`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssico.md"))]
pub use crate::linear_algebra::dense::ssico;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssics`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssics.md"))]
pub use crate::linear_algebra::dense::ssics;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssidi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssidi.md"))]
pub use crate::linear_algebra::dense::ssidi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssifa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssifa.md"))]
pub use crate::linear_algebra::dense::ssifa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssilus`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssilus.md"))]
pub use crate::linear_algebra::dense::ssilus;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssisl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssisl.md"))]
pub use crate::linear_algebra::dense::ssisl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssli`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssli.md"))]
pub use crate::linear_algebra::dense::ssli;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssli2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssli2.md"))]
pub use crate::linear_algebra::dense::ssli2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssllti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssllti.md"))]
pub use crate::linear_algebra::dense::ssllti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sslucs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslucs.md"))]
pub use crate::linear_algebra::dense::sslucs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sslui`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslui.md"))]
pub use crate::linear_algebra::dense::sslui;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sslui2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslui2.md"))]
pub use crate::linear_algebra::dense::sslui2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::sslui4`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslui4.md"))]
pub use crate::linear_algebra::dense::sslui4;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssluti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssluti.md"))]
pub use crate::linear_algebra::dense::ssluti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssmmi2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssmmi2.md"))]
pub use crate::linear_algebra::dense::ssmmi2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssmmti`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssmmti.md"))]
pub use crate::linear_algebra::dense::ssmmti;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ssvdc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssvdc.md"))]
pub use crate::linear_algebra::dense::ssvdc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::strco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strco.md"))]
pub use crate::linear_algebra::dense::strco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::strdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strdi.md"))]
pub use crate::linear_algebra::dense::strdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::strsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/strsl.md"))]
pub use crate::linear_algebra::dense::strsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::dense::ulsia`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ulsia.md"))]
pub use crate::linear_algebra::dense::ulsia;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::bakvec`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bakvec.md"))]
pub use crate::linear_algebra::eigen::bakvec;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::balanc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/balanc.md"))]
pub use crate::linear_algebra::eigen::balanc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::balbak`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/balbak.md"))]
pub use crate::linear_algebra::eigen::balbak;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::bandv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bandv.md"))]
pub use crate::linear_algebra::eigen::bandv;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::bisect`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bisect.md"))]
pub use crate::linear_algebra::eigen::bisect;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::bqr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bqr.md"))]
pub use crate::linear_algebra::eigen::bqr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::cbabk2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbabk2.md"))]
pub use crate::linear_algebra::eigen::cbabk2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::cbal`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cbal.md"))]
pub use crate::linear_algebra::eigen::cbal;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::cg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cg.md"))]
pub use crate::linear_algebra::eigen::cg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::cgeev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cgeev.md"))]
pub use crate::linear_algebra::eigen::cgeev;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::ch`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ch.md"))]
pub use crate::linear_algebra::eigen::ch;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::chiev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chiev.md"))]
pub use crate::linear_algebra::eigen::chiev;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::combak`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/combak.md"))]
pub use crate::linear_algebra::eigen::combak;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::comhes`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/comhes.md"))]
pub use crate::linear_algebra::eigen::comhes;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::comlr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/comlr.md"))]
pub use crate::linear_algebra::eigen::comlr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::comlr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/comlr2.md"))]
pub use crate::linear_algebra::eigen::comlr2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::comqr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/comqr.md"))]
pub use crate::linear_algebra::eigen::comqr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::comqr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/comqr2.md"))]
pub use crate::linear_algebra::eigen::comqr2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::cortb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cortb.md"))]
pub use crate::linear_algebra::eigen::cortb;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::corth`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/corth.md"))]
pub use crate::linear_algebra::eigen::corth;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::elmbak`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/elmbak.md"))]
pub use crate::linear_algebra::eigen::elmbak;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::elmhes`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/elmhes.md"))]
pub use crate::linear_algebra::eigen::elmhes;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::eltran`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/eltran.md"))]
pub use crate::linear_algebra::eigen::eltran;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::figi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/figi.md"))]
pub use crate::linear_algebra::eigen::figi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::figi2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/figi2.md"))]
pub use crate::linear_algebra::eigen::figi2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::hqr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hqr.md"))]
pub use crate::linear_algebra::eigen::hqr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::hqr2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hqr2.md"))]
pub use crate::linear_algebra::eigen::hqr2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::htrib3`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/htrib3.md"))]
pub use crate::linear_algebra::eigen::htrib3;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::htribk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/htribk.md"))]
pub use crate::linear_algebra::eigen::htribk;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::htrid3`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/htrid3.md"))]
pub use crate::linear_algebra::eigen::htrid3;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::htridi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/htridi.md"))]
pub use crate::linear_algebra::eigen::htridi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::imtql1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/imtql1.md"))]
pub use crate::linear_algebra::eigen::imtql1;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::imtql2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/imtql2.md"))]
pub use crate::linear_algebra::eigen::imtql2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::imtqlv`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/imtqlv.md"))]
pub use crate::linear_algebra::eigen::imtqlv;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::ortbak`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ortbak.md"))]
pub use crate::linear_algebra::eigen::ortbak;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::orthes`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/orthes.md"))]
pub use crate::linear_algebra::eigen::orthes;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::ortran`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ortran.md"))]
pub use crate::linear_algebra::eigen::ortran;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::qzvec`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qzvec.md"))]
pub use crate::linear_algebra::eigen::qzvec;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rebak`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rebak.md"))]
pub use crate::linear_algebra::eigen::rebak;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rebakb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rebakb.md"))]
pub use crate::linear_algebra::eigen::rebakb;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::reduc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/reduc.md"))]
pub use crate::linear_algebra::eigen::reduc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::reduc2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/reduc2.md"))]
pub use crate::linear_algebra::eigen::reduc2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rg.md"))]
pub use crate::linear_algebra::eigen::rg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rgg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rgg.md"))]
pub use crate::linear_algebra::eigen::rgg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rs.md"))]
pub use crate::linear_algebra::eigen::rs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rsb`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rsb.md"))]
pub use crate::linear_algebra::eigen::rsb;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rsg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rsg.md"))]
pub use crate::linear_algebra::eigen::rsg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rsgab`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rsgab.md"))]
pub use crate::linear_algebra::eigen::rsgab;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rsgba`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rsgba.md"))]
pub use crate::linear_algebra::eigen::rsgba;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rsp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rsp.md"))]
pub use crate::linear_algebra::eigen::rsp;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rst`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rst.md"))]
pub use crate::linear_algebra::eigen::rst;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::rt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rt.md"))]
pub use crate::linear_algebra::eigen::rt;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::sgeev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgeev.md"))]
pub use crate::linear_algebra::eigen::sgeev;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::ssiev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssiev.md"))]
pub use crate::linear_algebra::eigen::ssiev;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::sspev`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspev.md"))]
pub use crate::linear_algebra::eigen::sspev;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tinvit`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tinvit.md"))]
pub use crate::linear_algebra::eigen::tinvit;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tql1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tql1.md"))]
pub use crate::linear_algebra::eigen::tql1;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tql2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tql2.md"))]
pub use crate::linear_algebra::eigen::tql2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tqlrat`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tqlrat.md"))]
pub use crate::linear_algebra::eigen::tqlrat;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::trbak1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/trbak1.md"))]
pub use crate::linear_algebra::eigen::trbak1;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::trbak3`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/trbak3.md"))]
pub use crate::linear_algebra::eigen::trbak3;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tred1`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tred1.md"))]
pub use crate::linear_algebra::eigen::tred1;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tred2`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tred2.md"))]
pub use crate::linear_algebra::eigen::tred2;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tred3`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tred3.md"))]
pub use crate::linear_algebra::eigen::tred3;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tridib`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tridib.md"))]
pub use crate::linear_algebra::eigen::tridib;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::eigen::tsturm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/tsturm.md"))]
pub use crate::linear_algebra::eigen::tsturm;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dppco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppco.md"))]
pub use crate::linear_algebra::packed::dppco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dppdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppdi.md"))]
pub use crate::linear_algebra::packed::dppdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dppfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppfa.md"))]
pub use crate::linear_algebra::packed::dppfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dppsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppsl.md"))]
pub use crate::linear_algebra::packed::dppsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dspco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspco.md"))]
pub use crate::linear_algebra::packed::dspco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dspdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspdi.md"))]
pub use crate::linear_algebra::packed::dspdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dspfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspfa.md"))]
pub use crate::linear_algebra::packed::dspfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::dspsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspsl.md"))]
pub use crate::linear_algebra::packed::dspsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sppco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sppco.md"))]
pub use crate::linear_algebra::packed::sppco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sppdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sppdi.md"))]
pub use crate::linear_algebra::packed::sppdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sppfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sppfa.md"))]
pub use crate::linear_algebra::packed::sppfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sppsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sppsl.md"))]
pub use crate::linear_algebra::packed::sppsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sspco`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspco.md"))]
pub use crate::linear_algebra::packed::sspco;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sspdi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspdi.md"))]
pub use crate::linear_algebra::packed::sspdi;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sspfa`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspfa.md"))]
pub use crate::linear_algebra::packed::sspfa;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::packed::sspsl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sspsl.md"))]
pub use crate::linear_algebra::packed::sspsl;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdbcg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdbcg.md"))]
pub use crate::linear_algebra::sparse::dsdbcg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdcg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdcg.md"))]
pub use crate::linear_algebra::sparse::dsdcg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdcgn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdcgn.md"))]
pub use crate::linear_algebra::sparse::dsdcgn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdcgs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdcgs.md"))]
pub use crate::linear_algebra::sparse::dsdcgs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdgmr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdgmr.md"))]
pub use crate::linear_algebra::sparse::dsdgmr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsdomn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsdomn.md"))]
pub use crate::linear_algebra::sparse::dsdomn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsgs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsgs.md"))]
pub use crate::linear_algebra::sparse::dsgs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsiccg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsiccg.md"))]
pub use crate::linear_algebra::sparse::dsiccg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsilur`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsilur.md"))]
pub use crate::linear_algebra::sparse::dsilur;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsjac`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsjac.md"))]
pub use crate::linear_algebra::sparse::dsjac;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dslubc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslubc.md"))]
pub use crate::linear_algebra::sparse::dslubc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dslucn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslucn.md"))]
pub use crate::linear_algebra::sparse::dslucn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dslugm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dslugm.md"))]
pub use crate::linear_algebra::sparse::dslugm;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::dsluom`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsluom.md"))]
pub use crate::linear_algebra::sparse::dsluom;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdbcg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdbcg.md"))]
pub use crate::linear_algebra::sparse::ssdbcg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdcg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdcg.md"))]
pub use crate::linear_algebra::sparse::ssdcg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdcgn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdcgn.md"))]
pub use crate::linear_algebra::sparse::ssdcgn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdcgs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdcgs.md"))]
pub use crate::linear_algebra::sparse::ssdcgs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdgmr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdgmr.md"))]
pub use crate::linear_algebra::sparse::ssdgmr;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssdomn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssdomn.md"))]
pub use crate::linear_algebra::sparse::ssdomn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssgs`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssgs.md"))]
pub use crate::linear_algebra::sparse::ssgs;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssiccg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssiccg.md"))]
pub use crate::linear_algebra::sparse::ssiccg;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssilur`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssilur.md"))]
pub use crate::linear_algebra::sparse::ssilur;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssjac`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssjac.md"))]
pub use crate::linear_algebra::sparse::ssjac;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::sslubc`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslubc.md"))]
pub use crate::linear_algebra::sparse::sslubc;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::sslucn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslucn.md"))]
pub use crate::linear_algebra::sparse::sslucn;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::sslugm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sslugm.md"))]
pub use crate::linear_algebra::sparse::sslugm;
#[doc = "Transitional ABI-shaped alias; use `crate::linear_algebra::sparse::ssluom`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ssluom.md"))]
pub use crate::linear_algebra::sparse::ssluom;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::jacobian_check::chkder`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chkder.md"))]
pub use crate::nonlinear::jacobian_check::chkder;
#[doc = "Transitional ABI-shaped alias; use `crate::nonlinear::jacobian_check::dckder`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dckder.md"))]
pub use crate::nonlinear::jacobian_check::dckder;
#[doc = "Transitional ABI-shaped alias; use `crate::ode::dintp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dintp.md"))]
pub use crate::ode::dintp;
#[doc = "Transitional ABI-shaped alias; use `crate::ode::sintrp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sintrp.md"))]
pub use crate::ode::sintrp;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::blktri`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/blktri.md"))]
pub use crate::pde::fishpack::blktri;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::genbun`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/genbun.md"))]
pub use crate::pde::fishpack::genbun;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hstcrt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hstcrt.md"))]
pub use crate::pde::fishpack::hstcrt;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hstcsp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hstcsp.md"))]
pub use crate::pde::fishpack::hstcsp;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hstcyl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hstcyl.md"))]
pub use crate::pde::fishpack::hstcyl;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hstplr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hstplr.md"))]
pub use crate::pde::fishpack::hstplr;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hstssp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hstssp.md"))]
pub use crate::pde::fishpack::hstssp;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hw3crt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hw3crt.md"))]
pub use crate::pde::fishpack::hw3crt;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hwscrt`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwscrt.md"))]
pub use crate::pde::fishpack::hwscrt;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hwscsp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwscsp.md"))]
pub use crate::pde::fishpack::hwscsp;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hwscyl`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwscyl.md"))]
pub use crate::pde::fishpack::hwscyl;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hwsplr`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwsplr.md"))]
pub use crate::pde::fishpack::hwsplr;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::hwsssp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/hwsssp.md"))]
pub use crate::pde::fishpack::hwsssp;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::pois3d`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/pois3d.md"))]
pub use crate::pde::fishpack::pois3d;
#[doc = "Transitional ABI-shaped alias; use `crate::pde::fishpack::poistg`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/poistg.md"))]
pub use crate::pde::fishpack::poistg;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::avint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/avint.md"))]
pub use crate::quadrature::avint;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::bsqad`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bsqad.md"))]
pub use crate::quadrature::bsqad;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::davint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/davint.md"))]
pub use crate::quadrature::davint;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::dbsqad`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsqad.md"))]
pub use crate::quadrature::dbsqad;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::dppqad`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppqad.md"))]
pub use crate::quadrature::dppqad;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::dqmomo`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dqmomo.md"))]
pub use crate::quadrature::dqmomo;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::ppqad`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ppqad.md"))]
pub use crate::quadrature::ppqad;
#[doc = "Transitional ABI-shaped alias; use `crate::quadrature::qmomo`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/qmomo.md"))]
pub use crate::quadrature::qmomo;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besi.md"))]
pub use crate::special::bessel::besi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besj.md"))]
pub use crate::special::bessel::besj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besk.md"))]
pub use crate::special::bessel::besk;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::beskes`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/beskes.md"))]
pub use crate::special::bessel::beskes;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besks`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besks.md"))]
pub use crate::special::bessel::besks;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::besy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/besy.md"))]
pub use crate::special::bessel::besy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesi.md"))]
pub use crate::special::bessel::dbesi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesj.md"))]
pub use crate::special::bessel::dbesj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesk.md"))]
pub use crate::special::bessel::dbesk;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesks`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesks.md"))]
pub use crate::special::bessel::dbesks;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::dbesy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbesy.md"))]
pub use crate::special::bessel::dbesy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::zbesh`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/zbesh.md"))]
pub use crate::special::bessel::zbesh;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::zbesi`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/zbesi.md"))]
pub use crate::special::bessel::zbesi;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::zbesj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/zbesj.md"))]
pub use crate::special::bessel::zbesj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::zbesk`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/zbesk.md"))]
pub use crate::special::bessel::zbesk;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bessel::zbesy`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/zbesy.md"))]
pub use crate::special::bessel::zbesy;
#[doc = "Transitional ABI-shaped alias; use `crate::special::bskin`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bskin.md"))]
pub use crate::special::bskin;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbskes`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbskes.md"))]
pub use crate::special::dbskes;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dbskin`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbskin.md"))]
pub use crate::special::dbskin;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dexint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dexint.md"))]
pub use crate::special::dexint;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dpsifn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dpsifn.md"))]
pub use crate::special::dpsifn;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drc3jj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drc3jj.md"))]
pub use crate::special::drc3jj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drc3jm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drc3jm.md"))]
pub use crate::special::drc3jm;
#[doc = "Transitional ABI-shaped alias; use `crate::special::drc6j`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drc6j.md"))]
pub use crate::special::drc6j;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dxlegf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dxlegf.md"))]
pub use crate::special::dxlegf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::dxnrmp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dxnrmp.md"))]
pub use crate::special::dxnrmp;
#[doc = "Transitional ABI-shaped alias; use `crate::special::exint`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/exint.md"))]
pub use crate::special::exint;
#[doc = "Transitional ABI-shaped alias; use `crate::special::psifn`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/psifn.md"))]
pub use crate::special::psifn;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rc3jj`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rc3jj.md"))]
pub use crate::special::rc3jj;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rc3jm`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rc3jm.md"))]
pub use crate::special::rc3jm;
#[doc = "Transitional ABI-shaped alias; use `crate::special::rc6j`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rc6j.md"))]
pub use crate::special::rc6j;
#[doc = "Transitional ABI-shaped alias; use `crate::special::xlegf`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/xlegf.md"))]
pub use crate::special::xlegf;
#[doc = "Transitional ABI-shaped alias; use `crate::special::xnrmp`."]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/xnrmp.md"))]
pub use crate::special::xnrmp;
// ffi-declaration-aliases:end
