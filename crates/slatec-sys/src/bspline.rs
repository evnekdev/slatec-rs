//! Hand-reviewed raw declarations for the selected scalar B-spline routines.
//!
//! `BVALU`/`DBVALU` return a scalar function result by value. `BSQAD` and
//! `DBSQAD` return their definite integral through `BQUAD`. All arrays are
//! contiguous in the safe facade and use GNU Fortran's default `INTEGER`.

use crate::FortranInteger;

unsafe extern "C" {
    /// Evaluates a single-precision B-spline value or derivative.
    #[link_name = "bvalu_"]
    pub fn bvalu(
        knots: *const f32,
        coefficients: *const f32,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const f32,
        interval_state: *mut FortranInteger,
        workspace: *mut f32,
    ) -> f32;
    /// Evaluates a double-precision B-spline value or derivative.
    #[link_name = "dbvalu_"]
    pub fn dbvalu(
        knots: *const f64,
        coefficients: *const f64,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const f64,
        interval_state: *mut FortranInteger,
        workspace: *mut f64,
    ) -> f64;
    /// Integrates a single-precision B-spline over two validated limits.
    #[link_name = "bsqad_"]
    pub fn bsqad(
        knots: *const f32,
        coefficients: *const f32,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const f32,
        upper: *const f32,
        integral: *mut f32,
        workspace: *mut f32,
    );
    /// Integrates a double-precision B-spline over two validated limits.
    #[link_name = "dbsqad_"]
    pub fn dbsqad(
        knots: *const f64,
        coefficients: *const f64,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const f64,
        upper: *const f64,
        integral: *mut f64,
        workspace: *mut f64,
    );
}
