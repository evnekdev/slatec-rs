//! Raw declarations generated from the selected, compiler-observed SLATEC corpus.
//!
//! The crate deliberately performs no download, native compilation, linking, or
//! safe conversion in `build.rs`. Enable a specific `raw-ffi-*` feature only
//! after generating and validating the matching native evidence with
//! `slatec-corpus generate-raw-ffi --offline`.

/// GNU Fortran default `INTEGER` after the supported profile probe.
pub type FortranInteger = i32;

/// GNU Fortran default `LOGICAL` after the supported profile probe. This is
/// intentionally not Rust `bool`.
pub type FortranLogical = i32;

/// GNU Fortran hidden CHARACTER length for the supported 64-bit profile.
pub type FortranCharacterLength = usize;

/// GNU Fortran-compatible default `COMPLEX` storage used by the validated raw
/// ABI profile. This is a layout record, not a safe numerical type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

/// GNU Fortran-compatible default `DOUBLE COMPLEX` storage used by the
/// validated raw ABI profile. This is a layout record, not a safe numerical
/// type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

pub mod generated;
