//! Reviewed GNU Fortran declarations for nonlinear least-squares easy drivers.
//!
//! The safe layer calls `SNLS1E` and `DNLS1E` only with `IOPT = 1`: the
//! original SLATEC implementation forms a forward-difference Jacobian from
//! the residual callback. `FJAC` and `LDFJAC` are formal callback arguments
//! but are not used in that mode. The ABI remains unsafe because callers own
//! callback lifetime, vector/workspace validity, and runtime serialization.

use crate::FortranInteger;

/// GNU Fortran residual callback accepted by `DNLS1E`.
///
/// The first pointer is the mutable callback control flag `IFLAG`, followed by
/// `M`, `N`, a read-only parameter vector `X`, mutable residual storage
/// `FVEC`, and the unused-in-`IOPT=1` Jacobian arguments `FJAC` and `LDFJAC`.
pub type LeastSquaresFnF64 = unsafe extern "C" fn(
    *mut FortranInteger,
    *const FortranInteger,
    *const FortranInteger,
    *const f64,
    *mut f64,
    *mut f64,
    *const FortranInteger,
);

/// GNU Fortran residual callback accepted by `SNLS1E`.
///
/// This is the single-precision counterpart of [`LeastSquaresFnF64`].
pub type LeastSquaresFnF32 = unsafe extern "C" fn(
    *mut FortranInteger,
    *const FortranInteger,
    *const FortranInteger,
    *const f32,
    *mut f32,
    *mut f32,
    *const FortranInteger,
);

unsafe extern "C" {
    /// Original double-precision SLATEC easy driver `DNLS1E`.
    #[link_name = "dnls1e_"]
    pub fn dnls1e(
        function: LeastSquaresFnF64,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f64,
        residuals: *mut f64,
        tolerance: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        workspace: *mut f64,
        workspace_length: *mut FortranInteger,
    );

    /// Original single-precision SLATEC easy driver `SNLS1E`.
    #[link_name = "snls1e_"]
    pub fn snls1e(
        function: LeastSquaresFnF32,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f32,
        residuals: *mut f32,
        tolerance: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        workspace: *mut f32,
        workspace_length: *mut FortranInteger,
    );
}
