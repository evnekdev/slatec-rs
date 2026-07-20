//! Raw GNU Fortran declarations for the reviewed nonlinear easy drivers.
//!
//! `SNSQE` and `DNSQE` are called only with `IOPT = 2`, so the original
//! Fortran implementation forms a forward-difference Jacobian from `FCN`.
//! `JAC` remains an ABI argument but is not invoked in that mode. Callers must
//! still supply a valid function pointer and uphold all callback and workspace
//! invariants.

use crate::FortranInteger;

/// GNU Fortran residual callback used by `DNSQE`.
///
/// The callback receives `N`, a current iterate `X`, mutable residual storage
/// `FVEC`, and the mutable Fortran control flag `IFLAG` by reference.
pub type NonlinearFnF64 =
    unsafe extern "C" fn(*const FortranInteger, *const f64, *mut f64, *mut FortranInteger);

/// GNU Fortran residual callback used by `SNSQE`.
pub type NonlinearFnF32 =
    unsafe extern "C" fn(*const FortranInteger, *const f32, *mut f32, *mut FortranInteger);

/// GNU Fortran Jacobian callback shape required by the `DNSQE` ABI.
///
/// The reviewed safe API uses `IOPT = 2`, so this callback is deliberately not
/// called. It remains part of the raw declaration because the Fortran routine
/// retains the formal argument.
pub type NonlinearJacF64 = unsafe extern "C" fn(
    *const FortranInteger,
    *const f64,
    *const f64,
    *mut f64,
    *const FortranInteger,
    *mut FortranInteger,
);

/// GNU Fortran Jacobian callback shape required by the `SNSQE` ABI.
pub type NonlinearJacF32 = unsafe extern "C" fn(
    *const FortranInteger,
    *const f32,
    *const f32,
    *mut f32,
    *const FortranInteger,
    *mut FortranInteger,
);

unsafe extern "C" {
    /// Original double-precision SLATEC nonlinear easy driver `DNSQE`.
    #[link_name = "dnsqe_"]
    pub fn dnsqe(
        function: NonlinearFnF64,
        jacobian: NonlinearJacF64,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        tolerance: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace: *mut f64,
        workspace_length: *mut FortranInteger,
    );

    /// Original single-precision SLATEC nonlinear easy driver `SNSQE`.
    #[link_name = "snsqe_"]
    pub fn snsqe(
        function: NonlinearFnF32,
        jacobian: NonlinearJacF32,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        tolerance: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace: *mut f32,
        workspace_length: *mut FortranInteger,
    );

    /// Original double-precision SLATEC expert Powell-hybrid driver `DNSQ`.
    #[link_name = "dnsq_"]
    pub fn dnsq(
        function: NonlinearFnF64,
        jacobian: NonlinearJacF64,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        xtol: *mut f64,
        maxfev: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f64,
        diag: *mut f64,
        mode: *mut FortranInteger,
        factor: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        nfev: *mut FortranInteger,
        njev: *mut FortranInteger,
        r: *mut f64,
        lr: *mut FortranInteger,
        qtf: *mut f64,
        wa1: *mut f64,
        wa2: *mut f64,
        wa3: *mut f64,
        wa4: *mut f64,
    );

    /// Original single-precision SLATEC expert Powell-hybrid driver `SNSQ`.
    #[link_name = "snsq_"]
    pub fn snsq(
        function: NonlinearFnF32,
        jacobian: NonlinearJacF32,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        xtol: *mut f32,
        maxfev: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f32,
        diag: *mut f32,
        mode: *mut FortranInteger,
        factor: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        nfev: *mut FortranInteger,
        njev: *mut FortranInteger,
        r: *mut f32,
        lr: *mut FortranInteger,
        qtf: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );

    /// Original double-precision SLATEC Jacobian checker `DCKDER`.
    #[link_name = "dckder_"]
    pub fn dckder(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        xp: *mut f64,
        fvecp: *mut f64,
        mode: *mut FortranInteger,
        error: *mut f64,
    );

    /// Original single-precision SLATEC Jacobian checker `CHKDER`.
    #[link_name = "chkder_"]
    pub fn chkder(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        xp: *mut f32,
        fvecp: *mut f32,
        mode: *mut FortranInteger,
        error: *mut f32,
    );
}

#[cfg(feature = "raw-family-nonlinear-jacobian")]
#[path = "batch_a/nonlinear.rs"]
mod canonical_bindings;

/// Canonical source-verified Jacobian-check declarations.
#[cfg(feature = "raw-family-nonlinear-jacobian")]
pub use canonical_bindings::*;

/// Complex polynomial and nonlinear-equation interfaces.
#[cfg(feature = "raw-family-nonlinear-complex")]
pub mod complex {
    pub use crate::abi_bindings::nonlinear::*;
}
