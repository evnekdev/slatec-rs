//! Hand-reviewed scalar declarations for the expanded real special-function API.
//!
//! These declarations cover the selected real scalar routines only.  The
//! Carlson routines return their numerical value as a Fortran function and
//! report their documented argument-range status through `IER`.

use crate::FortranInteger;

unsafe extern "C" {
    #[link_name = "ali_"]
    pub fn ali(x: *mut f32) -> f32;
    #[link_name = "dli_"]
    pub fn dli(x: *mut f64) -> f64;
    #[link_name = "spenc_"]
    pub fn spenc(x: *mut f32) -> f32;
    #[link_name = "dspenc_"]
    pub fn dspenc(x: *mut f64) -> f64;

    #[link_name = "rc_"]
    pub fn rc(x: *mut f32, y: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drc_"]
    pub fn drc(x: *mut f64, y: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rf_"]
    pub fn rf(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drf_"]
    pub fn drf(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rd_"]
    pub fn rd(x: *mut f32, y: *mut f32, z: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drd_"]
    pub fn drd(x: *mut f64, y: *mut f64, z: *mut f64, ier: *mut FortranInteger) -> f64;
    #[link_name = "rj_"]
    pub fn rj(x: *mut f32, y: *mut f32, z: *mut f32, p: *mut f32, ier: *mut FortranInteger) -> f32;
    #[link_name = "drj_"]
    pub fn drj(x: *mut f64, y: *mut f64, z: *mut f64, p: *mut f64, ier: *mut FortranInteger)
    -> f64;
}
