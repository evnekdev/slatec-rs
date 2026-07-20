//! Generated Batch C canonical raw declarations.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-c --offline`.

// raw-api-routine: CBLKTR
/// Solve a block tridiagonal system of linear equations (usually resulting from the discretization of separable two-dimensional elliptic equations).
///
/// Original SLATEC routine `CBLKTR`; source: <https://www.netlib.org/slatec/fishfft/cblktr.f>; source SHA-256: `34bdeeae64c2379ac2b5b47e9d9c47511eb68b4551da04e404aa6149901b9aba`; native symbol: `cblktr_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_f32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `IFLG`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `NP`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `AN`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `BN`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CN`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `MP`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `M`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `AM`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `BM`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CM`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IDIMY`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `Y`: Fortran `COMPLEX` rank-2 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IERROR`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `W`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::cblktr;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "cblktr_"]
    pub fn cblktr(
        iflg: *mut crate::FortranInteger,
        np: *mut crate::FortranInteger,
        n: *mut crate::FortranInteger,
        an: *mut f32,
        bn: *mut f32,
        cn: *mut f32,
        mp: *mut crate::FortranInteger,
        m: *mut crate::FortranInteger,
        am: *mut crate::Complex32,
        bm: *mut crate::Complex32,
        cm: *mut crate::Complex32,
        idimy: *mut crate::FortranInteger,
        y: *mut crate::Complex32,
        ierror: *mut crate::FortranInteger,
        w: *mut f32,
    );
}

// raw-api-routine: CMGNBN
/// Solve a complex block tridiagonal linear system of equations by a cyclic reduction algorithm.
///
/// Original SLATEC routine `CMGNBN`; source: <https://www.netlib.org/slatec/fishfft/cmgnbn.f>; source SHA-256: `d90dfb81074b2cf34af266ec88a8db535d14c663127e154f79a4d0461e0062d7`; native symbol: `cmgnbn_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_complex32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `NPEROD`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `MPEROD`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `M`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `A`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `B`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `C`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IDIMY`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `Y`: Fortran `COMPLEX` rank-2 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IERROR`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `W`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::cmgnbn;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "cmgnbn_"]
    pub fn cmgnbn(
        nperod: *mut crate::FortranInteger,
        n: *mut crate::FortranInteger,
        mperod: *mut crate::FortranInteger,
        m: *mut crate::FortranInteger,
        a: *mut crate::Complex32,
        b: *mut crate::Complex32,
        c: *mut crate::Complex32,
        idimy: *mut crate::FortranInteger,
        y: *mut crate::Complex32,
        ierror: *mut crate::FortranInteger,
        w: *mut crate::Complex32,
    );
}
