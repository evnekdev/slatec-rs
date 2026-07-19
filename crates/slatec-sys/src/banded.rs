//! Hand-reviewed raw declarations for real LINPACK general-band systems.
use crate::FortranInteger;

unsafe extern "C" {
    /// Factors an `f32` general band matrix in expanded LINPACK storage.
    #[link_name = "sgbfa_"]
    pub fn sgbfa(
        abd: *mut f32,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        info: *mut FortranInteger,
    );
    /// Solves an `f32` factored system; `job=0` is A and nonzero is transpose(A).
    #[link_name = "sgbsl_"]
    pub fn sgbsl(
        abd: *mut f32,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        b: *mut f32,
        job: *mut FortranInteger,
    );
    /// Factors an `f64` general band matrix in expanded LINPACK storage.
    #[link_name = "dgbfa_"]
    pub fn dgbfa(
        abd: *mut f64,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        info: *mut FortranInteger,
    );
    /// Solves an `f64` factored system; `job=0` is A and nonzero is transpose(A).
    #[link_name = "dgbsl_"]
    pub fn dgbsl(
        abd: *mut f64,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        b: *mut f64,
        job: *mut FortranInteger,
    );
}
