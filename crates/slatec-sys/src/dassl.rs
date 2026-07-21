//! Hand-reviewed raw declarations for the restricted real DASSL drivers.
//!
//! The safe crate uses only `INFO(5)=0` (internal numerical differencing),
//! `INFO(6)=0` (dense iteration matrix), and `INFO(11)=0` (callers provide a
//! consistent `Y`/`YPRIME` pair). `JAC` remains a required ABI argument but
//! is never called in this reviewed mode.

use crate::FortranInteger;

/// Single-precision DASSL residual callback for
/// `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`.
///
/// `Y`, `YPRIME`, and `DELTA` each address `NEQ` elements. Write
/// `DELTA = G(T, Y, YPRIME)`; do not modify `T`, `Y`, or `YPRIME`. `IRES` is
/// zero on entry, may be set to `-1` for an illegal input, or to `-2` to make
/// the driver return `IDID = -11`. `RPAR` and `IPAR` are caller-owned
/// communication arrays. This callback is synchronous and must not unwind
/// through Fortran.
///
/// Source contract: <https://www.netlib.org/slatec/src/sdassl.f>.
///
/// # Safety
///
/// All pointers must be valid for the callback's documented scalar or array
/// extent. The callback must not retain native pointers after it returns.
pub type DasslResidualF32 = unsafe extern "C" fn(
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut f32,
    *mut FortranInteger,
);

/// Double-precision DASSL residual callback for
/// `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`.
///
/// `Y`, `YPRIME`, and `DELTA` each address `NEQ` `f64` elements. Write
/// `DELTA = G(T, Y, YPRIME)`; do not modify `T`, `Y`, or `YPRIME`. `IRES` is
/// zero on entry, may be set to `-1` for an illegal input, or to `-2` to make
/// the driver return `IDID = -11`. `RPAR` and `IPAR` are caller-owned
/// communication arrays. This callback is synchronous and must not unwind
/// through Fortran. Source contract: <https://www.netlib.org/slatec/src/ddassl.f>.
///
/// # Safety
///
/// All pointers must be valid for the callback's documented scalar or array
/// extent. The callback must not retain native pointers after it returns.
pub type DasslResidualF64 = unsafe extern "C" fn(
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
    *mut f64,
    *mut FortranInteger,
);

/// Single-precision optional DASSL Jacobian callback ABI:
/// `JAC(T, Y, YPRIME, PD, CJ, RPAR, IPAR)`.
///
/// When `INFO(5)=1`, write `PD = dG/dY + CJ*dG/dYPRIME` without modifying
/// `T`, `Y`, `YPRIME`, or `CJ`. Dense storage has first dimension `NEQ` and
/// stores `PD(I,J)`; banded storage has first dimension `2*ML+MU+1` and
/// stores `PD(I-J+ML+MU+1,J)`. It is synchronous and must not unwind through
/// Fortran. Source contract: <https://www.netlib.org/slatec/src/sdassl.f>.
///
/// # Safety
///
/// Every pointer must be valid for the callback's documented extent and must
/// not be retained after the callback returns.
pub type DasslJacobianF32 = unsafe extern "C" fn(
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
);

/// Double-precision optional DASSL Jacobian callback ABI.
///
/// When `INFO(5)=1`, write `PD = dG/dY + CJ*dG/dYPRIME` without modifying
/// `T`, `Y`, `YPRIME`, or `CJ`. Dense storage has first dimension `NEQ` and
/// stores `PD(I,J)`; banded storage has first dimension `2*ML+MU+1` and
/// stores `PD(I-J+ML+MU+1,J)`. It is synchronous and must not unwind through
/// Fortran. Source contract: <https://www.netlib.org/slatec/src/ddassl.f>.
///
/// # Safety
///
/// Every pointer must be valid for the callback's documented extent and must
/// not be retained after the callback returns.
pub type DasslJacobianF64 = unsafe extern "C" fn(
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
);

unsafe extern "C" {
    /// Original single-precision SLATEC DASSL driver `SDASSL`.
    #[link_name = "sdassl_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sdassl.md"))]
    pub fn sdassl(
        residual: DasslResidualF32,
        equations: *mut FortranInteger,
        time: *mut f32,
        state: *mut f32,
        state_derivative: *mut f32,
        target: *mut f32,
        info: *mut FortranInteger,
        relative_tolerance: *mut f32,
        absolute_tolerance: *mut f32,
        idid: *mut FortranInteger,
        real_workspace: *mut f32,
        real_workspace_length: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_length: *mut FortranInteger,
        real_parameters: *mut f32,
        integer_parameters: *mut FortranInteger,
        jacobian: DasslJacobianF32,
    );

    /// Original double-precision SLATEC DASSL driver `DDASSL`.
    #[link_name = "ddassl_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ddassl.md"))]
    pub fn ddassl(
        residual: DasslResidualF64,
        equations: *mut FortranInteger,
        time: *mut f64,
        state: *mut f64,
        state_derivative: *mut f64,
        target: *mut f64,
        info: *mut FortranInteger,
        relative_tolerance: *mut f64,
        absolute_tolerance: *mut f64,
        idid: *mut FortranInteger,
        real_workspace: *mut f64,
        real_workspace_length: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_length: *mut FortranInteger,
        real_parameters: *mut f64,
        integer_parameters: *mut FortranInteger,
        jacobian: DasslJacobianF64,
    );
}
