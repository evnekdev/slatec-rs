//! Hand-reviewed raw declarations for the selected SLATEC PCHIP package.
//!
//! Every array is contiguous in the safe facade, so it passes `INCFD = 1`.
//! `SKIP` is a GNU Fortran `LOGICAL`, represented by [`FortranLogical`], not
//! a Rust `bool`.  The native routines do not take callbacks.

use crate::{FortranInteger, FortranLogical};

unsafe extern "C" {
    /// Constructs monotonicity-preserving single-precision Hermite derivatives.
    #[link_name = "pchim_"]
    pub fn pchim(
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Constructs monotonicity-preserving double-precision Hermite derivatives.
    #[link_name = "dpchim_"]
    pub fn dpchim(
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Constructs controlled monotone single-precision Hermite derivatives.
    #[link_name = "pchic_"]
    pub fn pchic(
        ic: *const FortranInteger,
        vc: *const f32,
        switch: *const f32,
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
        workspace: *mut f32,
        workspace_len: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Constructs controlled monotone double-precision Hermite derivatives.
    #[link_name = "dpchic_"]
    pub fn dpchic(
        ic: *const FortranInteger,
        vc: *const f64,
        switch: *const f64,
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
        workspace: *mut f64,
        workspace_len: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Constructs single-precision cubic-spline Hermite derivatives.
    #[link_name = "pchsp_"]
    pub fn pchsp(
        ic: *const FortranInteger,
        vc: *const f32,
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *mut f32,
        incfd: *mut FortranInteger,
        workspace: *mut f32,
        workspace_len: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Constructs double-precision cubic-spline Hermite derivatives.
    #[link_name = "dpchsp_"]
    pub fn dpchsp(
        ic: *const FortranInteger,
        vc: *const f64,
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *mut f64,
        incfd: *mut FortranInteger,
        workspace: *mut f64,
        workspace_len: *mut FortranInteger,
        ierr: *mut FortranInteger,
    );
    /// Evaluates a single-precision Hermite function.
    #[link_name = "pchfe_"]
    pub fn pchfe(
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *const f32,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        evaluation_count: *mut FortranInteger,
        evaluation_points: *const f32,
        values: *mut f32,
        ierr: *mut FortranInteger,
    );
    /// Evaluates a double-precision Hermite function.
    #[link_name = "dpchfe_"]
    pub fn dpchfe(
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *const f64,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        evaluation_count: *mut FortranInteger,
        evaluation_points: *const f64,
        values: *mut f64,
        ierr: *mut FortranInteger,
    );
    /// Evaluates a single-precision Hermite function and first derivative.
    #[link_name = "pchfd_"]
    pub fn pchfd(
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *const f32,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        evaluation_count: *mut FortranInteger,
        evaluation_points: *const f32,
        values: *mut f32,
        derivatives: *mut f32,
        ierr: *mut FortranInteger,
    );
    /// Evaluates a double-precision Hermite function and first derivative.
    #[link_name = "dpchfd_"]
    pub fn dpchfd(
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *const f64,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        evaluation_count: *mut FortranInteger,
        evaluation_points: *const f64,
        values: *mut f64,
        derivatives: *mut f64,
        ierr: *mut FortranInteger,
    );
    /// Integrates a single-precision Hermite curve over arbitrary limits.
    #[link_name = "pchia_"]
    pub fn pchia(
        n: *mut FortranInteger,
        x: *const f32,
        f: *const f32,
        d: *const f32,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        lower: *const f32,
        upper: *const f32,
        ierr: *mut FortranInteger,
    ) -> f32;
    /// Integrates a double-precision Hermite curve over arbitrary limits.
    #[link_name = "dpchia_"]
    pub fn dpchia(
        n: *mut FortranInteger,
        x: *const f64,
        f: *const f64,
        d: *const f64,
        incfd: *mut FortranInteger,
        skip: *mut FortranLogical,
        lower: *const f64,
        upper: *const f64,
        ierr: *mut FortranInteger,
    ) -> f64;
}
