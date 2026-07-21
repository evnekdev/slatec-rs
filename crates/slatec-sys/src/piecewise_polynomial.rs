//! Hand-reviewed raw declarations for scalar SLATEC piecewise polynomials.
//!
//! `PPVAL`/`DPPVAL` return a scalar value by value. `PPQAD`/`DPPQAD`
//! return their exact integral through the final pointer. `BSPPP`/`DBSPPP`
//! fill caller-provided PP-form output arrays from a B-spline.

use crate::FortranInteger;

unsafe extern "C" {
    /// Evaluates a single-precision PP-form value or derivative.
    #[link_name = "ppval_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ppval.md"))]
    pub fn ppval(
        leading_dimension: *const FortranInteger,
        coefficients: *const f32,
        breakpoints: *const f32,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const f32,
        interval_state: *mut FortranInteger,
    ) -> f32;

    /// Evaluates a double-precision PP-form value or derivative.
    #[link_name = "dppval_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppval.md"))]
    pub fn dppval(
        leading_dimension: *const FortranInteger,
        coefficients: *const f64,
        breakpoints: *const f64,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const f64,
        interval_state: *mut FortranInteger,
    ) -> f64;

    /// Integrates a single-precision PP-form spline exactly.
    #[link_name = "ppqad_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ppqad.md"))]
    pub fn ppqad(
        leading_dimension: *const FortranInteger,
        coefficients: *const f32,
        breakpoints: *const f32,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const f32,
        upper: *const f32,
        integral: *mut f32,
    );

    /// Integrates a double-precision PP-form spline exactly.
    #[link_name = "dppqad_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dppqad.md"))]
    pub fn dppqad(
        leading_dimension: *const FortranInteger,
        coefficients: *const f64,
        breakpoints: *const f64,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const f64,
        upper: *const f64,
        integral: *mut f64,
    );

    /// Converts a single-precision B-spline to PP form.
    #[link_name = "bsppp_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bsppp.md"))]
    pub fn bsppp(
        knots: *const f32,
        coefficients: *const f32,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        leading_dimension: *const FortranInteger,
        pp_coefficients: *mut f32,
        breakpoints: *mut f32,
        piece_count: *mut FortranInteger,
        workspace: *mut f32,
    );

    /// Converts a double-precision B-spline to PP form.
    #[link_name = "dbsppp_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsppp.md"))]
    pub fn dbsppp(
        knots: *const f64,
        coefficients: *const f64,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        leading_dimension: *const FortranInteger,
        pp_coefficients: *mut f64,
        breakpoints: *mut f64,
        piece_count: *mut FortranInteger,
        workspace: *mut f64,
    );
}
