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
}
