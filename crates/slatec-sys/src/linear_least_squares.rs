//! Reviewed GNU Fortran declarations for `WNNLS` and `DWNNLS`.
//!
//! The routines solve equality-constrained least squares with the first `L`
//! variables unrestricted and the remaining variables nonnegative. `W` is a
//! mutable Fortran column-major `MDW × (N + 1)` augmented matrix. The top
//! `ME` rows contain equality equations and the next `MA` rows contain the
//! least-squares equations. The declarations are unsafe because callers own
//! the exact matrix/workspace layout and the process-global legacy runtime.

use crate::FortranInteger;

unsafe extern "C" {
    /// Original double-precision SLATEC weighted nonnegative least-squares
    /// driver `DWNNLS`.
    #[link_name = "dwnnls_"]
    pub fn dwnnls(
        augmented_matrix: *mut f64,
        leading_dimension: *mut FortranInteger,
        equality_rows: *mut FortranInteger,
        least_squares_rows: *mut FortranInteger,
        variable_count: *mut FortranInteger,
        free_variable_count: *mut FortranInteger,
        program_options: *mut f64,
        solution: *mut f64,
        residual_norm: *mut f64,
        mode: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        workspace: *mut f64,
    );

    /// Original single-precision SLATEC weighted nonnegative least-squares
    /// driver `WNNLS`.
    #[link_name = "wnnls_"]
    pub fn wnnls(
        augmented_matrix: *mut f32,
        leading_dimension: *mut FortranInteger,
        equality_rows: *mut FortranInteger,
        least_squares_rows: *mut FortranInteger,
        variable_count: *mut FortranInteger,
        free_variable_count: *mut FortranInteger,
        program_options: *mut f32,
        solution: *mut f32,
        residual_norm: *mut f32,
        mode: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        workspace: *mut f32,
    );
}
