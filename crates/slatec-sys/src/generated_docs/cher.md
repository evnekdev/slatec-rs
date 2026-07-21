# Purpose

CHER performs the hermitian rank 1 operation

# Description

This canonical unsafe binding exposes original SLATEC routine `CHER`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHER](https://www.netlib.org/slatec/lin/cher.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the array A is to be referenced as follows: 'U' or 'u'   Only the upper triangular part of A is to be referenced. 'L' or 'l'   Only the lower triangular part of A is to be referenced. Unchanged on exit. 'U' or 'u', the leading n by n upper triangular part of the array A must contain the upper triangular part of the hermitian matrix and the strictly lower triangular part of A is not referenced. On exit, the upper triangular part of the array A is overwritten by the upper triangular part of the updated matrix. 'L' or 'l', the leading n by n lower triangular part of the array A must contain the lower triangular part of the hermitian matrix and the strictly upper triangular part of A is not referenced. On exit, the lower triangular part of the array A is overwritten by the lower triangular part of the updated matrix. Note that the imaginary parts of the diagonal elements need not be set, they are assumed to be zero, and on exit they are set to zero. CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the array A is to be referenced as follows: 'U' or 'u'   Only the upper triangular part of A is to be referenced. 'L' or 'l'   Only the lower triangular part of A is to be referenced. Unchanged on exit. 'U' or 'u', the leading n by n upper triangular part of the array A must contain the upper triangular part of the hermitian matrix and the strictly lower triangular part of A is not referenced. On exit, the upper triangular part of the array A is overwritten by the upper triangular part of the updated matrix. 'L' or 'l', the leading n by n lower triangular part of the array A must contain the lower triangular part of the hermitian matrix and the strictly upper triangular part of A is not referenced. On exit, the lower triangular part of the array A is overwritten by the lower triangular part of the updated matrix. Note that the imaginary parts of the diagonal elements need not be set, they are assumed to be zero, and on exit they are set to zero. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. by n hermitian matrix. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. by n hermitian matrix. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 3. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX          array of dimension at least must not be zero. Unchanged on exit. COMPLEX          array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `A`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). = alpha*x*conjg( x') + A, where alpha is a real scalar, x is an n element vector and A is an COMPLEX          array of DIMENSION ( LDA, n ). = alpha*x*conjg( x') + A, where alpha is a real scalar, x is an n element vector and A is an COMPLEX          array of DIMENSION ( LDA, n ). not applicable or not stated by selected source not a workspace argument

## 7. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `UPLO`: not a workspace argument
- `N`: not a workspace argument
- `ALPHA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::cher`
- Original SLATEC routine: `CHER`
- Native symbol: `cher_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CHER](https://www.netlib.org/slatec/lin/cher.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
