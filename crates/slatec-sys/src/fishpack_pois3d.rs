//! Hand-reviewed declaration for SLATEC FISHPACK `POIS3D`.
//!
//! The selected driver is `fishfft/pois3d.f`. Its scalar mode and dimension
//! arguments are read-only. For cyclic third-axis problems it temporarily
//! overwrites and restores `a`, `b`, and `c`; `f`, `ierror`, and `w` are also
//! mutable. `f` has Fortran layout `F(LDIMF, MDIMF, N)`.

use crate::FortranInteger;

unsafe extern "C" {
    /// Solves the reviewed structured three-dimensional block-tridiagonal system.
    #[link_name = "pois3d_"]
    pub fn pois3d(
        lperod: *const FortranInteger,
        l: *const FortranInteger,
        c1: *const f32,
        mperod: *const FortranInteger,
        m: *const FortranInteger,
        c2: *const f32,
        nperod: *const FortranInteger,
        n: *const FortranInteger,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        ldimf: *const FortranInteger,
        mdimf: *const FortranInteger,
        f: *mut f32,
        ierror: *mut FortranInteger,
        w: *mut f32,
    );
}
