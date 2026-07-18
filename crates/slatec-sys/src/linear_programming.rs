//! Hand-reviewed raw declarations for the in-memory `SPLP`/`DSPLP` surface.
//!
//! The callback supplies one-based sparse matrix coordinates through
//! `IFLAG(1) = 1, 2, 3`. The safe crate fixes the option array so paging,
//! save/restore, and printing remain disabled and owns every mutable array.

use crate::FortranInteger;

/// Single-precision sparse matrix callback used by `SPLP`.
pub type LpMatrixF32 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut FortranInteger,
    *mut f32,
    *mut FortranInteger,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
);

/// Double-precision sparse matrix callback used by `DSPLP`.
pub type LpMatrixF64 = unsafe extern "C" fn(
    *mut FortranInteger,
    *mut FortranInteger,
    *mut f64,
    *mut FortranInteger,
    *mut f64,
    *mut f64,
    *mut FortranInteger,
);

unsafe extern "C" {
    /// Original single-precision SLATEC sparse linear-programming driver.
    #[link_name = "splp_"]
    pub fn splp(
        matrix: LpMatrixF32,
        rows: *mut FortranInteger,
        columns: *mut FortranInteger,
        costs: *mut f32,
        options: *mut f32,
        callback_data: *mut f32,
        lower_bounds: *mut f32,
        upper_bounds: *mut f32,
        bound_kinds: *mut FortranInteger,
        info: *mut FortranInteger,
        primal: *mut f32,
        duals: *mut f32,
        basis: *mut FortranInteger,
        workspace: *mut f32,
        workspace_len: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_len: *mut FortranInteger,
    );

    /// Original double-precision SLATEC sparse linear-programming driver.
    #[link_name = "dsplp_"]
    pub fn dsplp(
        matrix: LpMatrixF64,
        rows: *mut FortranInteger,
        columns: *mut FortranInteger,
        costs: *mut f64,
        options: *mut f64,
        callback_data: *mut f64,
        lower_bounds: *mut f64,
        upper_bounds: *mut f64,
        bound_kinds: *mut FortranInteger,
        info: *mut FortranInteger,
        primal: *mut f64,
        duals: *mut f64,
        basis: *mut FortranInteger,
        workspace: *mut f64,
        workspace_len: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_len: *mut FortranInteger,
    );
}
