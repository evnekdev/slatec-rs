//! Hand-reviewed raw declarations for the selected scalar B-spline routines.
//!
//! `BVALU`/`DBVALU` return a scalar function result by value. `BSQAD` and
//! `DBSQAD` return their definite integral through `BQUAD`.
//! `BINTK`/`DBINTK` construct coefficients for caller-supplied complete knot
//! sequences; their factorization workspace has exactly `(2*K - 1)*N`
//! elements and their scratch vector has exactly `2*K` elements. All arrays
//! are contiguous in the safe facade and use GNU Fortran's default `INTEGER`.

use crate::FortranInteger;

unsafe extern "C" {
    /// Evaluates a single-precision B-spline value or derivative.
    #[link_name = "bvalu_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bvalu.md"))]
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
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbvalu.md"))]
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
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bsqad.md"))]
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
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbsqad.md"))]
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
    /// Constructs single-precision B-spline coefficients that interpolate
    /// `Y(I)` at strictly increasing `X(I)` for a supplied knot sequence.
    ///
    /// `X`, `Y`, and `T` are read-only arrays of lengths `N`, `N`, and
    /// `N + K`; `BCOEF`, `Q`, and `WORK` are writable arrays of lengths `N`,
    /// `(2*K - 1)*N`, and `2*K`, respectively. The routine reports malformed
    /// or singular systems only through XERROR; the safe facade preflights the
    /// documented Schoenberg--Whitney conditions before this call.
    #[link_name = "bintk_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/bintk.md"))]
    pub fn bintk(
        nodes: *const f32,
        values: *const f32,
        knots: *const f32,
        point_count: *const FortranInteger,
        order: *const FortranInteger,
        coefficients: *mut f32,
        factorization: *mut f32,
        workspace: *mut f32,
    );
    /// Double-precision counterpart of [`bintk`].
    ///
    /// The array mutability and exact lengths are the same as for `BINTK`.
    #[link_name = "dbintk_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dbintk.md"))]
    pub fn dbintk(
        nodes: *const f64,
        values: *const f64,
        knots: *const f64,
        point_count: *const FortranInteger,
        order: *const FortranInteger,
        coefficients: *mut f64,
        factorization: *mut f64,
        workspace: *mut f64,
    );
}
