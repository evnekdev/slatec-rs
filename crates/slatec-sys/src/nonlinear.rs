//! Raw GNU Fortran declarations for the reviewed nonlinear easy drivers.
//!
//! `SNSQE` and `DNSQE` are called only with `IOPT = 2`, so the original
//! Fortran implementation forms a forward-difference Jacobian from `FCN`.
//! `JAC` remains an ABI argument but is not invoked in that mode. Callers must
//! still supply a valid function pointer and uphold all callback and workspace
//! invariants.

use crate::FortranInteger;

/// GNU Fortran residual callback used by `DNSQE`.
///
/// The callback receives `N`, a current iterate `X`, mutable residual storage
/// `FVEC`, and the mutable Fortran control flag `IFLAG` by reference.
pub type NonlinearFnF64 =
    unsafe extern "C" fn(*const FortranInteger, *const f64, *mut f64, *mut FortranInteger);

/// GNU Fortran residual callback used by `SNSQE`.
pub type NonlinearFnF32 =
    unsafe extern "C" fn(*const FortranInteger, *const f32, *mut f32, *mut FortranInteger);

/// GNU Fortran Jacobian callback shape required by the `DNSQE` ABI.
///
/// The reviewed safe API uses `IOPT = 2`, so this callback is deliberately not
/// called. It remains part of the raw declaration because the Fortran routine
/// retains the formal argument.
pub type NonlinearJacF64 = unsafe extern "C" fn(
    *const FortranInteger,
    *const f64,
    *const f64,
    *mut f64,
    *const FortranInteger,
    *mut FortranInteger,
);

/// GNU Fortran Jacobian callback shape required by the `SNSQE` ABI.
pub type NonlinearJacF32 = unsafe extern "C" fn(
    *const FortranInteger,
    *const f32,
    *const f32,
    *mut f32,
    *const FortranInteger,
    *mut FortranInteger,
);

/// GNU Fortran scalar equation callback used by the reviewed `SOS` driver.
///
/// The native callback receives only `X` and `K`; `NEQ` is not forwarded.
/// `X` points to a readable iterate whose extent must be known from externally
/// managed problem state, and `K` points to a one-based equation index in
/// `1..=NEQ`. The raw ABI has no user-data/context pointer, so a stateful Rust
/// wrapper requires a separate scoped context mechanism. The callback returns
/// equation `K` as a single-precision scalar and must not mutate or retain
/// either pointer.
#[cfg(feature = "raw-family-nonlinear-systems")]
pub type SosEquationF32 = unsafe extern "C" fn(*const f32, *const FortranInteger) -> f32;

/// GNU Fortran scalar equation callback used by the reviewed `DSOS` driver.
///
/// The native callback receives only `X` and `K`; `NEQ` is not forwarded.
/// `X` points to a readable iterate whose extent must be known from externally
/// managed problem state, and `K` points to a one-based equation index in
/// `1..=NEQ`. The raw ABI has no user-data/context pointer, so a stateful Rust
/// wrapper requires a separate scoped context mechanism. The callback returns
/// equation `K` as a double-precision scalar and must not mutate or retain
/// either pointer.
#[cfg(feature = "raw-family-nonlinear-systems")]
pub type SosEquationF64 = unsafe extern "C" fn(*const f64, *const FortranInteger) -> f64;

unsafe extern "C" {
    /// Original double-precision SLATEC nonlinear easy driver `DNSQE`.
    #[link_name = "dnsqe_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnsqe.md"))]
    pub fn dnsqe(
        function: NonlinearFnF64,
        jacobian: NonlinearJacF64,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        tolerance: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace: *mut f64,
        workspace_length: *mut FortranInteger,
    );

    /// Original single-precision SLATEC nonlinear easy driver `SNSQE`.
    #[link_name = "snsqe_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snsqe.md"))]
    pub fn snsqe(
        function: NonlinearFnF32,
        jacobian: NonlinearJacF32,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        tolerance: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        workspace: *mut f32,
        workspace_length: *mut FortranInteger,
    );

    /// Original double-precision SLATEC expert Powell-hybrid driver `DNSQ`.
    #[link_name = "dnsq_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dnsq.md"))]
    pub fn dnsq(
        function: NonlinearFnF64,
        jacobian: NonlinearJacF64,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        xtol: *mut f64,
        maxfev: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f64,
        diag: *mut f64,
        mode: *mut FortranInteger,
        factor: *mut f64,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        nfev: *mut FortranInteger,
        njev: *mut FortranInteger,
        r: *mut f64,
        lr: *mut FortranInteger,
        qtf: *mut f64,
        wa1: *mut f64,
        wa2: *mut f64,
        wa3: *mut f64,
        wa4: *mut f64,
    );

    /// Original single-precision SLATEC expert Powell-hybrid driver `SNSQ`.
    #[link_name = "snsq_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/snsq.md"))]
    pub fn snsq(
        function: NonlinearFnF32,
        jacobian: NonlinearJacF32,
        iopt: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        xtol: *mut f32,
        maxfev: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        epsfcn: *mut f32,
        diag: *mut f32,
        mode: *mut FortranInteger,
        factor: *mut f32,
        nprint: *mut FortranInteger,
        info: *mut FortranInteger,
        nfev: *mut FortranInteger,
        njev: *mut FortranInteger,
        r: *mut f32,
        lr: *mut FortranInteger,
        qtf: *mut f32,
        wa1: *mut f32,
        wa2: *mut f32,
        wa3: *mut f32,
        wa4: *mut f32,
    );

    /// Original double-precision SLATEC Jacobian checker `DCKDER`.
    #[link_name = "dckder_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dckder.md"))]
    pub fn dckder(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f64,
        fvec: *mut f64,
        fjac: *mut f64,
        ldfjac: *mut FortranInteger,
        xp: *mut f64,
        fvecp: *mut f64,
        mode: *mut FortranInteger,
        error: *mut f64,
    );

    /// Original single-precision SLATEC Jacobian checker `CHKDER`.
    #[link_name = "chkder_"]
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/chkder.md"))]
    pub fn chkder(
        m: *mut FortranInteger,
        n: *mut FortranInteger,
        x: *mut f32,
        fvec: *mut f32,
        fjac: *mut f32,
        ldfjac: *mut FortranInteger,
        xp: *mut f32,
        fvecp: *mut f32,
        mode: *mut FortranInteger,
        error: *mut f32,
    );

    /// Original single-precision SLATEC nonlinear-system driver `SOS`.
    ///
    /// The verified selected source is <https://www.netlib.org/slatec/src/sos.f>.
    /// Arguments: `FNC`, `NEQ`, `X`, `RTOLX`, `ATOLX`, `TOLF`, `IFLAG`, `RW`,
    /// `LRW`, `IW`, and `LIW`. See the generated canonical contract for the
    /// exact callback, workspace, tolerance, and status requirements.
    ///
    /// # Safety
    ///
    /// `FNC` must use [`SosEquationF32`], remain valid synchronously, and not
    /// unwind. All scalar pointers must be non-null and mutable where passed;
    /// `X` has `NEQ` writable elements, `RW` has at least
    /// `1 + 6*NEQ + NEQ*(NEQ + 1)/2` writable elements, and `IW` has at least
    /// `3 + NEQ` writable elements. The arrays must not alias incompatibly and
    /// the caller serializes legacy native runtime access.
    #[cfg(feature = "raw-family-nonlinear-systems")]
    #[link_name = "sos_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sos.md"))]
    pub fn sos(
        function: SosEquationF32,
        equation_count: *mut FortranInteger,
        solution: *mut f32,
        relative_x_tolerance: *mut f32,
        absolute_x_tolerance: *mut f32,
        residual_tolerance: *mut f32,
        status: *mut FortranInteger,
        real_workspace: *mut f32,
        real_workspace_length: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_length: *mut FortranInteger,
    );

    /// Original double-precision SLATEC nonlinear-system driver `DSOS`.
    ///
    /// The verified selected source is <https://www.netlib.org/slatec/src/dsos.f>.
    /// Arguments: `FNC`, `NEQ`, `X`, `RTOLX`, `ATOLX`, `TOLF`, `IFLAG`, `RW`,
    /// `LRW`, `IW`, and `LIW`. See the generated canonical contract for the
    /// exact callback, workspace, tolerance, and status requirements.
    ///
    /// # Safety
    ///
    /// `FNC` must use [`SosEquationF64`], remain valid synchronously, and not
    /// unwind. All scalar pointers must be non-null and mutable where passed;
    /// `X` has `NEQ` writable elements, `RW` has at least
    /// `1 + 6*NEQ + NEQ*(NEQ + 1)/2` writable elements, and `IW` has at least
    /// `3 + NEQ` writable elements. The arrays must not alias incompatibly and
    /// the caller serializes legacy native runtime access.
    #[cfg(feature = "raw-family-nonlinear-systems")]
    #[link_name = "dsos_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dsos.md"))]
    pub fn dsos(
        function: SosEquationF64,
        equation_count: *mut FortranInteger,
        solution: *mut f64,
        relative_x_tolerance: *mut f64,
        absolute_x_tolerance: *mut f64,
        residual_tolerance: *mut f64,
        status: *mut FortranInteger,
        real_workspace: *mut f64,
        real_workspace_length: *mut FortranInteger,
        integer_workspace: *mut FortranInteger,
        integer_workspace_length: *mut FortranInteger,
    );
}

/// Canonical reviewed raw SLATEC nonlinear-system drivers.
#[cfg(feature = "raw-family-nonlinear-systems")]
pub mod systems {
    pub use super::{dsos, sos, SosEquationF32, SosEquationF64};
}

#[cfg(feature = "raw-family-nonlinear-jacobian")]
#[path = "batch_a/nonlinear.rs"]
mod canonical_bindings;

/// Canonical source-verified Jacobian-check declarations.
#[cfg(feature = "raw-family-nonlinear-jacobian")]
pub use canonical_bindings::*;
