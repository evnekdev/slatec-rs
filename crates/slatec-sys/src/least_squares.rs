//! Reviewed GNU Fortran declarations for nonlinear least-squares drivers.
//!
//! The easy safe layer calls `SNLS1E` and `DNLS1E` only with `IOPT = 1`; the
//! expert layer calls `SNLS1` and `DNLS1` with `IOPT = 1` or `IOPT = 2`.
//! `IOPT = 2` requests a dense column-major `FJAC(LDFJAC, N)` from the same
//! callback. The ABI remains unsafe because callers own callback lifetime,
//! rectangular matrix/workspace validity, and runtime serialization.

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
    /// Original double-precision SLATEC covariance helper `DCOV`.
    ///
    /// The reviewed `FCN` ABI is [`LeastSquaresFnF64`]. `DCOV` recomputes
    /// `FVEC` and either a forward-difference (`IOPT=1`) or dense analytic
    /// (`IOPT=2`) Jacobian at `X`; it does not consume `DNLS1` factorization
    /// arrays. In forward-difference mode a subsidiary temporarily mutates
    /// `X`, then restores it before return. `R` has `LDR * N` elements and
    /// receives an upper triangular
    /// covariance result on successful return. The declaration is unsafe
    /// because callers own the callback, all workspace, and XERROR state.
    #[link_name = "dcov_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dcov.md"))]
    pub fn dcov(
        function: LeastSquaresFnF64,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f64,
        residuals: *mut f64,
        covariance_workspace: *mut f64,
        leading_dimension: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace_one: *mut f64,
        workspace_two: *mut f64,
        workspace_three: *mut f64,
        workspace_four: *mut f64,
    );

    /// Original single-precision SLATEC covariance helper `SCOV`.
    ///
    /// This is the f32 counterpart of [`dcov`], using the reviewed
    /// [`LeastSquaresFnF32`] callback ABI and the same temporary-`X`, `IOPT`,
    /// `R`, `LDR`, and workspace contract.
    #[link_name = "scov_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/scov.md"))]
    pub fn scov(
        function: LeastSquaresFnF32,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f32,
        residuals: *mut f32,
        covariance_workspace: *mut f32,
        leading_dimension: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace_one: *mut f32,
        workspace_two: *mut f32,
        workspace_three: *mut f32,
        workspace_four: *mut f32,
    );

    /// Original double-precision SLATEC expert driver `DNLS1`.
    ///
    /// The reviewed `FCN` ABI is [`LeastSquaresFnF64`]. Safe code selects
    /// either `IOPT = 1` (forward differences) or `IOPT = 2` (dense user
    /// Jacobian), fixes `NPRINT = 0`, and supplies all documented work arrays.
    #[link_name = "dnls1_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnls1.md"))]
    pub fn dnls1(
        function: LeastSquaresFnF64,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f64,
        residuals: *mut f64,
        jacobian: *mut f64,
        leading_dimension: *mut FortranInteger,
        function_tolerance: *mut f64,
        parameter_tolerance: *mut f64,
        gradient_tolerance: *mut f64,
        maximum_function_evaluations: *mut FortranInteger,
        finite_difference_step: *mut f64,
        scaling: *mut f64,
        scaling_mode: *mut FortranInteger,
        step_bound_factor: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        function_evaluations: *mut FortranInteger,
        jacobian_evaluations: *mut FortranInteger,
        pivot_workspace: *mut FortranInteger,
        qtf: *mut f64,
        workspace_one: *mut f64,
        workspace_two: *mut f64,
        workspace_three: *mut f64,
        workspace_four: *mut f64,
    );

    /// Original single-precision SLATEC expert driver `SNLS1`.
    ///
    /// This is the f32 counterpart of [`dnls1`], with the reviewed
    /// [`LeastSquaresFnF32`] callback ABI.
    #[link_name = "snls1_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snls1.md"))]
    pub fn snls1(
        function: LeastSquaresFnF32,
        iopt: *mut FortranInteger,
        residual_count: *mut FortranInteger,
        parameter_count: *mut FortranInteger,
        parameters: *mut f32,
        residuals: *mut f32,
        jacobian: *mut f32,
        leading_dimension: *mut FortranInteger,
        function_tolerance: *mut f32,
        parameter_tolerance: *mut f32,
        gradient_tolerance: *mut f32,
        maximum_function_evaluations: *mut FortranInteger,
        finite_difference_step: *mut f32,
        scaling: *mut f32,
        scaling_mode: *mut FortranInteger,
        step_bound_factor: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        function_evaluations: *mut FortranInteger,
        jacobian_evaluations: *mut FortranInteger,
        pivot_workspace: *mut FortranInteger,
        qtf: *mut f32,
        workspace_one: *mut f32,
        workspace_two: *mut f32,
        workspace_three: *mut f32,
        workspace_four: *mut f32,
    );

    /// Original double-precision SLATEC easy driver `DNLS1E`.
    #[link_name = "dnls1e_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnls1e.md"))]
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
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snls1e.md"))]
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
