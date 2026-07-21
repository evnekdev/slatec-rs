# Purpose

SSPR performs the symmetric rank 1 operation A := alpha*x*x' + A, where alpha is a real scalar, x is an n element vector and A is an

# Description

This canonical unsafe binding exposes original SLATEC routine `SSPR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSPR](https://www.netlib.org/slatec/lin/sspr.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed 'U' or 'u'   The upper triangular part of A is supplied in AP. 'L' or 'l'   The lower triangular part of A is supplied in AP. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) 'L' or 'l', the array AP must contain the lower triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed 'U' or 'u'   The upper triangular part of A is supplied in AP. 'L' or 'l'   The lower triangular part of A is supplied in AP. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) 'L' or 'l', the array AP must contain the lower triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. by n symmetric matrix, supplied in packed form. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1)*abs( INCX)). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. by n symmetric matrix, supplied in packed form. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1)*abs( INCX)). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 3. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL             array of dimension at least must not be zero. Unchanged on exit. REAL             array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `AP`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL             array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). contain a( 1, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. On exit, the array and a( 2, 2 ) respectively, and so on. On exit, the array is overwritten by the upper triangular part of the updated matrix. contain a( 2, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. On exit, the array and a( 3, 1 ) respectively, and so on. On exit, the array is overwritten by the lower triangular part of the updated matrix. REAL             array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). contain a( 1, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. On exit, the array and a( 2, 2 ) respectively, and so on. On exit, the array is overwritten by the upper triangular part of the updated matrix. contain a( 2, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. On exit, the array and a( 3, 1 ) respectively, and so on. On exit, the array is overwritten by the lower triangular part of the updated matrix. not applicable or not stated by selected source not a workspace argument

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
- `AP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::sspr`
- Original SLATEC routine: `SSPR`
- Native symbol: `sspr_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSPR](https://www.netlib.org/slatec/lin/sspr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
