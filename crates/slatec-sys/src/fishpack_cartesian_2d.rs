//! Hand-reviewed declaration for SLATEC FISHPACK `HWSCRT`.
//!
//! The selected driver is the single-precision `fishfft/hwscrt.f` source.
//! It receives scalar Fortran arguments by reference, mutates `f`, `pertrb`,
//! `ierror`, and its work array, and uses an `idimf`-by-`n + 1` column-major
//! `f` array.  The safe facade owns the mutable storage and validates every
//! dimensional precondition before entering this declaration.

use crate::FortranInteger;

unsafe extern "C" {
    /// Solves the reviewed Cartesian five-point Helmholtz finite-difference system.
    #[link_name = "hwscrt_"]
    pub fn hwscrt(
        a: *const f32,
        b: *const f32,
        m: *const FortranInteger,
        mbdcnd: *const FortranInteger,
        bda: *const f32,
        bdb: *const f32,
        c: *const f32,
        d: *const f32,
        n: *const FortranInteger,
        nbdcnd: *const FortranInteger,
        bdc: *const f32,
        bdd: *const f32,
        elmbda: *const f32,
        f: *mut f32,
        idimf: *const FortranInteger,
        pertrb: *mut f32,
        ierror: *mut FortranInteger,
        w: *mut f32,
    );
}
