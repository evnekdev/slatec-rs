//! Hand-reviewed scalar declarations for the expanded real special-function API.
//!
//! These declarations cover the selected real scalar routines only.  The
//! Carlson routines return their numerical value as a Fortran function and
//! report their documented argument-range status through `IER`.

use crate::FortranInteger;

unsafe extern "C" {
    #[link_name = "ali_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/ali.md"))]
    pub fn ali(x: *mut f32) -> f32;
    #[link_name = "dli_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dli.md"))]
    pub fn dli(x: *mut f64) -> f64;
    #[link_name = "spenc_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/spenc.md"))]
    pub fn spenc(x: *mut f32) -> f32;
    #[link_name = "dspenc_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dspenc.md"))]
    pub fn dspenc(x: *mut f64) -> f64;

    #[link_name = "rc_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rc.md"))]
    pub fn rc(x: *mut f32, y: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drc_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drc.md"))]
    pub fn drc(x: *mut f64, y: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rf_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rf.md"))]
    pub fn rf(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drf_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drf.md"))]
    pub fn drf(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rd_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rd.md"))]
    pub fn rd(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drd_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drd.md"))]
    pub fn drd(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rj_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/rj.md"))]
    pub fn rj(x: *mut f32, y: *mut f32, z: *mut f32, p: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drj_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/drj.md"))]
    pub fn drj(x: *mut f64, y: *mut f64, z: *mut f64, p: *mut f64, ier: *mut FortranInteger)
    -> f64;
}
