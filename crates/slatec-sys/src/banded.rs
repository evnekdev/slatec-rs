//! Hand-reviewed raw declarations for real LINPACK general-band systems.
use crate::FortranInteger;

unsafe extern "C" {
    /// Factors an `f32` general-band matrix and estimates its reciprocal 1-norm condition number.
    #[link_name = "sgbco_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbco.md"))]
    pub fn sgbco(
        abd: *mut f32,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        rcond: *mut f32,
        z: *mut f32,
    );
    /// Factors an `f32` general band matrix in expanded LINPACK storage.
    #[link_name = "sgbfa_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbfa.md"))]
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
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbsl.md"))]
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
    /// Returns the normalized base-ten determinant pair for read-only `f32` band LU factors.
    #[link_name = "sgbdi_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/sgbdi.md"))]
    pub fn sgbdi(
        abd: *const f32,
        lda: *const FortranInteger,
        n: *const FortranInteger,
        ml: *const FortranInteger,
        mu: *const FortranInteger,
        ipvt: *const FortranInteger,
        det: *mut f32,
    );
    /// Factors an `f64` general-band matrix and estimates its reciprocal 1-norm condition number.
    #[link_name = "dgbco_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbco.md"))]
    pub fn dgbco(
        abd: *mut f64,
        lda: *mut FortranInteger,
        n: *mut FortranInteger,
        ml: *mut FortranInteger,
        mu: *mut FortranInteger,
        ipvt: *mut FortranInteger,
        rcond: *mut f64,
        z: *mut f64,
    );
    /// Factors an `f64` general band matrix in expanded LINPACK storage.
    #[link_name = "dgbfa_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbfa.md"))]
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
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbsl.md"))]
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
    /// Returns the normalized base-ten determinant pair for read-only `f64` band LU factors.
    #[link_name = "dgbdi_"]
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_docs/dgbdi.md"))]
    pub fn dgbdi(
        abd: *const f64,
        lda: *const FortranInteger,
        n: *const FortranInteger,
        ml: *const FortranInteger,
        mu: *const FortranInteger,
        ipvt: *const FortranInteger,
        det: *mut f64,
    );
}
