//! Hand-reviewed raw declarations for the restricted SDRIVE expert surface.
//!
//! The safe crate calls `SDRIV3`/`DDRIV3` only with `MINT=1` (Adams) or
//! `MINT=2` (BDF), `MITER=0`, `IMPL=0`, and `NROOT=0`. The other callbacks
//! remain ABI arguments but are not invoked in that reviewed RHS-only mode.
//! Their reviewed argument order is
//! `N,T,Y,F,NSTATE,TOUT,NTASK,NROOT,EPS,EWT,IERROR,MINT,MITER,IMPL,ML,MU,`
//! `MXORD,HMAX,WORK,LENW,IWORK,LENIW,JACOBN,FA,NDE,MXSTEP,G,USERS,IERFLG`.

use crate::FortranInteger;

/// SDRIVE single-precision RHS callback: `F(N, T, Y, YDOT)`.
pub type SdriveRhsF32 = unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut f32, *mut f32);

/// SDRIVE double-precision RHS callback: `F(N, T, Y, YDOT)`.
pub type SdriveRhsF64 = unsafe extern "C" fn(*mut FortranInteger, *mut f64, *mut f64, *mut f64);

/// SDRIVE single-precision Jacobian callback ABI.
pub type SdriveJacobianF32 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);
/// SDRIVE double-precision Jacobian callback ABI.
pub type SdriveJacobianF64 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);
/// SDRIVE single-precision mass-matrix callback ABI.
pub type SdriveMassF32 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);
/// SDRIVE double-precision mass-matrix callback ABI.
pub type SdriveMassF64 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);
/// SDRIVE single-precision root-function callback ABI.
pub type SdriveRootF32 =
    unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut f32, *mut FortranInteger) -> f32;
/// SDRIVE double-precision root-function callback ABI.
pub type SdriveRootF64 =
    unsafe extern "C" fn(*mut FortranInteger, *mut f64, *mut f64, *mut FortranInteger) -> f64;
/// SDRIVE single-precision user-linear-solver callback ABI.
pub type SdriveUsersF32 = unsafe extern "C" fn(
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);
/// SDRIVE double-precision user-linear-solver callback ABI.
pub type SdriveUsersF64 = unsafe extern "C" fn(
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut FortranInteger,
);

unsafe extern "C" {
    /// Original single-precision SLATEC expert SDRIVE driver `SDRIV3`.
    #[link_name = "sdriv3_"]
    pub fn sdriv3(
        n: *mut FortranInteger,
        t: *mut f32,
        y: *mut f32,
        function: SdriveRhsF32,
        nstate: *mut FortranInteger,
        tout: *mut f32,
        ntask: *mut FortranInteger,
        nroot: *mut FortranInteger,
        eps: *mut f32,
        ewt: *mut f32,
        ierror: *mut FortranInteger,
        mint: *mut FortranInteger,
        miter: *mut FortranInteger,
        impl_: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        mxord: *mut FortranInteger,
        hmax: *mut f32,
        work: *mut f32,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        leniw: *mut FortranInteger,
        jacobn: SdriveJacobianF32,
        fa: SdriveMassF32,
        nde: *mut FortranInteger,
        mxstep: *mut FortranInteger,
        g: SdriveRootF32,
        users: SdriveUsersF32,
        ierflg: *mut FortranInteger,
    );
    /// Original double-precision SLATEC expert SDRIVE driver `DDRIV3`.
    #[link_name = "ddriv3_"]
    pub fn ddriv3(
        n: *mut FortranInteger,
        t: *mut f64,
        y: *mut f64,
        function: SdriveRhsF64,
        nstate: *mut FortranInteger,
        tout: *mut f64,
        ntask: *mut FortranInteger,
        nroot: *mut FortranInteger,
        eps: *mut f64,
        ewt: *mut f64,
        ierror: *mut FortranInteger,
        mint: *mut FortranInteger,
        miter: *mut FortranInteger,
        impl_: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        mxord: *mut FortranInteger,
        hmax: *mut f64,
        work: *mut f64,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        leniw: *mut FortranInteger,
        jacobn: SdriveJacobianF64,
        fa: SdriveMassF64,
        nde: *mut FortranInteger,
        mxstep: *mut FortranInteger,
        g: SdriveRootF64,
        users: SdriveUsersF64,
        ierflg: *mut FortranInteger,
    );
}
