#![no_std]

//! Raw declarations generated from the selected, compiler-observed SLATEC corpus.
//!
//! The crate deliberately performs no download, native compilation, linking,
//! or safe conversion in `build.rs`. Native provider selection and automatic
//! hosted linking belong to `slatec-src`; an external-backend consumer can use
//! these declarations without any hidden native build directive.

/// GNU Fortran default `INTEGER` after the supported profile probe.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranInteger = i32;

/// GNU Fortran default `LOGICAL` after the supported profile probe. This is
/// intentionally not Rust `bool`.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranLogical = i32;

/// GNU Fortran hidden CHARACTER length for the supported 64-bit profile.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
pub type FortranCharacterLength = usize;

/// GNU Fortran-compatible default `COMPLEX` storage used by the validated raw
/// ABI profile. This is a layout record, not a safe numerical type.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

/// GNU Fortran-compatible default `DOUBLE COMPLEX` storage used by the
/// validated raw ABI profile. This is a layout record, not a safe numerical
/// type.
#[cfg(feature = "ffi-profile-gnu-mingw-x86_64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

pub mod generated;

/// Generated raw declarations grouped by safe public family rather than ABI
/// shape. These modules are the preferred dependency of narrow safe features.
#[cfg(any(
    feature = "raw-family-blas-level1",
    feature = "raw-family-blas-level2",
    feature = "raw-family-blas-level3",
    feature = "raw-family-special-elementary",
    feature = "raw-family-special-gamma",
    feature = "raw-family-special-beta",
    feature = "raw-family-special-error",
    feature = "raw-family-special-airy",
    feature = "raw-family-special-bessel",
    feature = "raw-family-special-integrals",
    feature = "raw-family-special-polynomials"
))]
pub mod families;

/// Hand-reviewed callback declarations for the focused safe QUADPACK surface.
///
/// These declarations remain separate from the broadly gated callback batch:
/// only the eight routines whose callback and workspace contracts are tested
/// by `slatec::quadrature` are available here.
#[cfg(any(
    feature = "raw-ffi-quadrature",
    feature = "raw-family-quadrature-basic",
    feature = "raw-family-quadrature-breakpoints",
    feature = "raw-family-quadrature-weighted",
    feature = "raw-family-quadrature-oscillatory",
    feature = "raw-family-quadrature-fourier",
    feature = "raw-family-quadrature-nonadaptive"
))]
pub mod quadrature;

/// Hand-reviewed scalar callback declarations for the focused FZERO family.
///
/// This narrow module remains separate from the general callback batch.
#[cfg(any(feature = "raw-ffi-roots", feature = "raw-family-roots-scalar"))]
pub mod roots;

/// Hand-reviewed nonlinear callback and Jacobian-check declarations.
///
/// This narrow module exposes `SNSQE`, `DNSQE`, `SNSQ`, `DNSQ`, `CHKDER`, and
/// `DCKDER` for the validated GNU MinGW ABI profile. The declarations remain
/// unsafe because the caller owns callback lifetime, vector and matrix
/// storage, workspace, and runtime serialization.
#[cfg(any(
    feature = "raw-ffi-nonlinear",
    feature = "raw-family-nonlinear-easy",
    feature = "raw-family-nonlinear-expert",
    feature = "raw-family-nonlinear-jacobian-check"
))]
pub mod nonlinear;
