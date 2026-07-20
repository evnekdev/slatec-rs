//! Generated Batch C canonical raw declarations.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-c --offline`.

// raw-api-routine: CDCDOT
/// Compute the inner product of two vectors with extended precision accumulation.
///
/// Original SLATEC routine `CDCDOT`; source: <https://www.netlib.org/slatec/lin/cdcdot.f>; source SHA-256: `e005ea4248a742f5de1b8330b849106c40931fa8e9109c060c53194bb983d995`; native symbol: `cdcdot_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_return`; fingerprint `function:complex32(mut_i32,mut_complex32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CB`: Fortran `COMPLEX` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CX`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCX`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CY`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCY`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - Return: compiler-validated Fortran `COMPLEX` value.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-returns")]
#[doc(inline)]
pub use crate::generated::complex_returns::cdcdot;
#[cfg(not(feature = "raw-ffi-complex-returns"))]
unsafe extern "C" {
    #[link_name = "cdcdot_"]
    pub fn cdcdot(
        n: *mut crate::FortranInteger,
        cb: *mut crate::Complex32,
        cx: *mut crate::Complex32,
        incx: *mut crate::FortranInteger,
        cy: *mut crate::Complex32,
        incy: *mut crate::FortranInteger,
    ) -> crate::Complex32;
}

// raw-api-routine: CDOTC
/// Dot product of two complex vectors using the complex conjugate of the first vector.
///
/// Original SLATEC routine `CDOTC`; source: <https://www.netlib.org/slatec/lin/cdotc.f>; source SHA-256: `ec14a8ca9dfbc18a9e9728e39e4a9d87514f5cbb63a11781d28113d881a05289`; native symbol: `cdotc_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_return`; fingerprint `function:complex32(mut_i32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CX`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCX`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CY`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCY`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - Return: compiler-validated Fortran `COMPLEX` value.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-returns")]
#[doc(inline)]
pub use crate::generated::complex_returns::cdotc;
#[cfg(not(feature = "raw-ffi-complex-returns"))]
unsafe extern "C" {
    #[link_name = "cdotc_"]
    pub fn cdotc(
        n: *mut crate::FortranInteger,
        cx: *mut crate::Complex32,
        incx: *mut crate::FortranInteger,
        cy: *mut crate::Complex32,
        incy: *mut crate::FortranInteger,
    ) -> crate::Complex32;
}

// raw-api-routine: CDOTU
/// Compute the inner product of two vectors.
///
/// Original SLATEC routine `CDOTU`; source: <https://www.netlib.org/slatec/lin/cdotu.f>; source SHA-256: `720691230d2327a3750d6c43043c8f1ce82b0275201ad7530757a430327aef41`; native symbol: `cdotu_`.
/// Supported ABI: GNU MinGW `x86_64-pc-windows-gnu`; Batch C class `complex_return`; fingerprint `function:complex32(mut_i32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`.
/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.
///
/// # Arguments
///
/// - `N`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CX`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCX`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `CY`: Fortran `COMPLEX` rank-1 array. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - `INCY`: Fortran `INTEGER` scalar. Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.
/// - Return: compiler-validated Fortran `COMPLEX` value.
///
/// # Safety
///
/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.
#[cfg(feature = "raw-ffi-complex-returns")]
#[doc(inline)]
pub use crate::generated::complex_returns::cdotu;
#[cfg(not(feature = "raw-ffi-complex-returns"))]
unsafe extern "C" {
    #[link_name = "cdotu_"]
    pub fn cdotu(
        n: *mut crate::FortranInteger,
        cx: *mut crate::Complex32,
        incx: *mut crate::FortranInteger,
        cy: *mut crate::Complex32,
        incy: *mut crate::FortranInteger,
    ) -> crate::Complex32;
}
