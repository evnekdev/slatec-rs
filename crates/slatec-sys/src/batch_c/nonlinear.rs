//! Generated Batch C canonical raw declarations.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-c --offline`.

// raw-api-routine: CPQR79
/// Find the zeros of a polynomial with complex coefficients.
///
/// Original SLATEC routine `CPQR79`; source: <https://www.netlib.org/slatec/src/cpqr79.f>; source SHA-256: `d0986bc46418a96dfb6faa0371fd8186882b38fd75a2b49ce2c6d65643a936b8`; native symbol: `cpqr79_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `NDEG`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `COEFF`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `ROOT`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IERR`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `WORK`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::cpqr79;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "cpqr79_"]
    pub fn cpqr79(
        ndeg: *mut crate::FortranInteger,
        coeff: *mut crate::Complex32,
        root: *mut crate::Complex32,
        ierr: *mut crate::FortranInteger,
        work: *mut f32,
    );
}

// raw-api-routine: CPZERO
/// Find the zeros of a polynomial with complex coefficients.
///
/// Original SLATEC routine `CPZERO`; source: <https://www.netlib.org/slatec/src/cpzero.f>; source SHA-256: `dc957dad2aaab08454bd429ba4b5c4d16c8c2c8aee7c76c9f27c638940b318ff`; native symbol: `cpzero_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `IN`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `A`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `R`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `T`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IFLG`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `S`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::cpzero;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "cpzero_"]
    pub fn cpzero(
        r#in: *mut crate::FortranInteger,
        a: *mut crate::Complex32,
        r: *mut crate::Complex32,
        t: *mut crate::Complex32,
        iflg: *mut crate::FortranInteger,
        s: *mut f32,
    );
}

// raw-api-routine: RPQR79
/// Find the zeros of a polynomial with real coefficients.
///
/// Original SLATEC routine `RPQR79`; source: <https://www.netlib.org/slatec/src/rpqr79.f>; source SHA-256: `90d572aa5e1df644b6f8266dcaa50f1b21026645a47eead8f3e294308643c243`; native symbol: `rpqr79_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `NDEG`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `COEFF`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `ROOT`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IERR`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `WORK`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::rpqr79;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "rpqr79_"]
    pub fn rpqr79(
        ndeg: *mut crate::FortranInteger,
        coeff: *mut f32,
        root: *mut crate::Complex32,
        ierr: *mut crate::FortranInteger,
        work: *mut f32,
    );
}

// raw-api-routine: RPZERO
/// Find the zeros of a polynomial with real coefficients.
///
/// Original SLATEC routine `RPZERO`; source: <https://www.netlib.org/slatec/src/rpzero.f>; source SHA-256: `6590b9b9bbec7e4cd22d960289ffb8daf8835f5551a658d24601d2506097a8a8`; native symbol: `rpzero_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_argument_or_output`; fingerprint `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `A`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `R`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `T`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `IFLG`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `S`: Fortran `REAL` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-arguments")]
#[doc(inline)]
pub use crate::generated::complex_arguments::rpzero;
#[cfg(not(feature = "raw-ffi-complex-arguments"))]
unsafe extern "C" {
    #[link_name = "rpzero_"]
    pub fn rpzero(
        n: *mut crate::FortranInteger,
        a: *mut f32,
        r: *mut crate::Complex32,
        t: *mut crate::Complex32,
        iflg: *mut crate::FortranInteger,
        s: *mut f32,
    );
}
