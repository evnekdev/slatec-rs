//! Hand-reviewed raw declarations for the restricted real DASSL drivers.
//!
//! The safe crate uses only `INFO(5)=0` (internal numerical differencing),
//! `INFO(6)=0` (dense iteration matrix), and `INFO(11)=0` (callers provide a
//! consistent `Y`/`YPRIME` pair). `JAC` remains a required ABI argument but
//! is never called in this reviewed mode.

use crate::FortranInteger;

/// Single-precision DASSL residual callback:
/// `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`.
pub type DasslResidualF32 = unsafe extern "C" fn(
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut f32,
    *mut FortranInteger,
);

/// Double-precision DASSL residual callback:
/// `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`.
pub type DasslResidualF64 = unsafe extern "C" fn(
    *mut f64,
    *mut f64,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
    *mut f64,
    *mut FortranInteger,
);

/// Single-precision optional DASSL Jacobian callback ABI.
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
