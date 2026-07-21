//! Hand-reviewed raw declarations for the restricted SDRIVE expert surface.
//!
//! The safe crate calls `SDRIV3`/`DDRIV3` only with `MINT=1` (Adams) or
//! `MINT=2` (BDF), `MITER=0`, `IMPL=0`, and `NROOT=0`. The other callbacks
//! remain ABI arguments but are not invoked in that reviewed RHS-only mode.
//! Their reviewed argument order is
//! `N,T,Y,F,NSTATE,TOUT,NTASK,NROOT,EPS,EWT,IERROR,MINT,MITER,IMPL,ML,MU,`
//! `MXORD,HMAX,WORK,LENW,IWORK,LENIW,JACOBN,FA,NDE,MXSTEP,G,USERS,IERFLG`.

use crate::{Complex32, FortranInteger};

/// SDRIVE single-precision RHS callback: `F(N, T, Y, YDOT)`.
pub type SdriveRhsF32 = unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut f32, *mut f32);

/// SDRIVE double-precision RHS callback: `F(N, T, Y, YDOT)`.
pub type SdriveRhsF64 = unsafe extern "C" fn(*mut FortranInteger, *mut f64, *mut f64, *mut f64);

/// CDRIVE complex single-precision RHS callback: `F(N, T, Y, YDOT)`.
///
/// `T` is real; `Y` and `YDOT` are mutable/readable `N`-element complex
/// vectors in the selected GNU Fortran `COMPLEX` layout. The callback is
/// synchronous, has no user-data pointer, and must not unwind through Fortran.
pub type CdriveRhsF32 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut f32,
    *mut Complex32,
    *mut Complex32,
);

/// CDRIVE real-valued root-function callback: `G(N, T, Y, IROOT) -> value`.
///
/// `Y` is a readable `N`-element complex vector. The scalar return is a real
/// `f32`, directly returned by the GNU Fortran profile; the callback has no
/// user-data pointer and must not unwind through Fortran.
pub type CdriveRootF32 =
    unsafe extern "C" fn(*mut FortranInteger, *mut f32, *mut Complex32, *mut FortranInteger) -> f32;

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
    /// Original single-precision SLATEC SDRIVE convenience driver `SDRIV1`.
    /// Source: <https://www.netlib.org/slatec/src/sdriv1.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `EPS`, `WORK`, `LENW`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `F` must be a valid synchronous RHS callback. `Y` has at least `N`
    /// writable elements, `WORK` has at least `N*N + 11*N + 300` elements,
    /// and all scalar pointers remain valid for the native call and any
    /// continuation call that reuses `WORK`.
    #[link_name = "sdriv1_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdriv1.md"))]
    pub fn sdriv1(
        n: *mut FortranInteger,
        t: *mut f32,
        y: *mut f32,
        function: SdriveRhsF32,
        tout: *mut f32,
        mstate: *mut FortranInteger,
        eps: *mut f32,
        work: *mut f32,
        lenw: *mut FortranInteger,
        ierflg: *mut FortranInteger,
    );

    /// Original double-precision SLATEC SDRIVE convenience driver `DDRIV1`.
    /// Source: <https://www.netlib.org/slatec/src/ddriv1.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `EPS`, `WORK`, `LENW`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `F` must be a valid synchronous RHS callback. `Y` has at least `N`
    /// writable elements, `WORK` has at least `N*N + 11*N + 300` elements,
    /// and all scalar pointers remain valid for the native call and any
    /// continuation call that reuses `WORK`.
    #[link_name = "ddriv1_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddriv1.md"))]
    pub fn ddriv1(
        n: *mut FortranInteger,
        t: *mut f64,
        y: *mut f64,
        function: SdriveRhsF64,
        tout: *mut f64,
        mstate: *mut FortranInteger,
        eps: *mut f64,
        work: *mut f64,
        lenw: *mut FortranInteger,
        ierflg: *mut FortranInteger,
    );

    /// Original single-precision SLATEC SDRIVE advanced driver `SDRIV2`.
    /// Source: <https://www.netlib.org/slatec/src/sdriv2.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `NROOT`, `EPS`, `EWT`, `MINT`,
    /// `WORK`, `LENW`, `IWORK`, `LENIW`, `G`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `F` and `G` must use their reviewed callback ABIs. `Y` has at least
    /// `N` elements; `WORK` and `IWORK` meet the source-selected `MINT` and
    /// `NROOT` bounds and remain undisturbed across continuation calls.
    #[link_name = "sdriv2_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdriv2.md"))]
    pub fn sdriv2(
        n: *mut FortranInteger,
        t: *mut f32,
        y: *mut f32,
        function: SdriveRhsF32,
        tout: *mut f32,
        mstate: *mut FortranInteger,
        nroot: *mut FortranInteger,
        eps: *mut f32,
        ewt: *mut f32,
        mint: *mut FortranInteger,
        work: *mut f32,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        leniw: *mut FortranInteger,
        g: SdriveRootF32,
        ierflg: *mut FortranInteger,
    );

    /// Original double-precision SLATEC SDRIVE advanced driver `DDRIV2`.
    /// Source: <https://www.netlib.org/slatec/src/ddriv2.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `NROOT`, `EPS`, `EWT`, `MINT`,
    /// `WORK`, `LENW`, `IWORK`, `LENIW`, `G`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `F` and `G` must use their reviewed callback ABIs. `Y` has at least
    /// `N` elements; `WORK` and `IWORK` meet the source-selected `MINT` and
    /// `NROOT` bounds and remain undisturbed across continuation calls.
    #[link_name = "ddriv2_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddriv2.md"))]
    pub fn ddriv2(
        n: *mut FortranInteger,
        t: *mut f64,
        y: *mut f64,
        function: SdriveRhsF64,
        tout: *mut f64,
        mstate: *mut FortranInteger,
        nroot: *mut FortranInteger,
        eps: *mut f64,
        ewt: *mut f64,
        mint: *mut FortranInteger,
        work: *mut f64,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        leniw: *mut FortranInteger,
        g: SdriveRootF64,
        ierflg: *mut FortranInteger,
    );

    /// Original complex SLATEC SDRIVE convenience driver `CDRIV1`.
    /// Source: <https://www.netlib.org/slatec/src/cdriv1.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `EPS`, `WORK`, `LENW`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `Y` and `WORK` use `Complex32` in the selected GNU Fortran layout;
    /// `Y` has at least `N` elements and `WORK` at least `N*N + 11*N + 300`.
    /// `F` is synchronous, has no context pointer, and must not unwind.
    #[link_name = "cdriv1_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cdriv1.md"))]
    pub fn cdriv1(
        n: *mut FortranInteger,
        t: *mut f32,
        y: *mut Complex32,
        function: CdriveRhsF32,
        tout: *mut f32,
        mstate: *mut FortranInteger,
        eps: *mut f32,
        work: *mut Complex32,
        lenw: *mut FortranInteger,
        ierflg: *mut FortranInteger,
    );

    /// Original complex SLATEC SDRIVE advanced driver `CDRIV2`.
    /// Source: <https://www.netlib.org/slatec/src/cdriv2.f>. Arguments: `N`,
    /// `T`, `Y`, `F`, `TOUT`, `MSTATE`, `NROOT`, `EPS`, `EWT`, `MINT`,
    /// `WORK`, `LENW`, `IWORK`, `LENIW`, `G`, `IERFLG`.
    ///
    /// # Safety
    ///
    /// `Y` and `WORK` use `Complex32` in the selected GNU Fortran layout.
    /// `F` and the real-valued root callback `G` must remain valid for the
    /// native call; `WORK` and `IWORK` meet the documented `MINT`/`NROOT`
    /// bounds and remain undisturbed across continuation calls.
    #[link_name = "cdriv2_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/cdriv2.md"))]
    pub fn cdriv2(
        n: *mut FortranInteger,
        t: *mut f32,
        y: *mut Complex32,
        function: CdriveRhsF32,
        tout: *mut f32,
        mstate: *mut FortranInteger,
        nroot: *mut FortranInteger,
        eps: *mut f32,
        ewt: *mut f32,
        mint: *mut FortranInteger,
        work: *mut Complex32,
        lenw: *mut FortranInteger,
        iwork: *mut FortranInteger,
        leniw: *mut FortranInteger,
        g: CdriveRootF32,
        ierflg: *mut FortranInteger,
    );

    /// Original single-precision SLATEC expert SDRIVE driver `SDRIV3`.
    #[link_name = "sdriv3_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdriv3.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddriv3.md"))]
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

#[cfg(feature = "raw-family-ode-integration")]
#[path = "batch_a/ode.rs"]
mod canonical_bindings;

/// Canonical source-verified non-callback ODE declarations.
#[cfg(feature = "raw-family-ode-integration")]
pub use canonical_bindings::*;

#[cfg(feature = "raw-family-ode-callbacks")]
#[path = "batch_b/ode.rs"]
mod callback_bindings;

/// Canonical source-verified callback-bearing ODE declarations.
#[cfg(feature = "raw-family-ode-callbacks")]
pub use callback_bindings::callbacks;
