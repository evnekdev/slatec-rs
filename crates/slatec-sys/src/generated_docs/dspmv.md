# Purpose

DSPMV performs the matrix-vector operation

# Description

This canonical unsafe binding exposes original SLATEC routine `DSPMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSPMV](https://www.netlib.org/slatec/lin/dspmv.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed 'U' or 'u'   The upper triangular part of A is supplied in AP. 'L' or 'l'   The lower triangular part of A is supplied in AP. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) 'L' or 'l', the array AP must contain the lower triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed 'U' or 'u'   The upper triangular part of A is supplied in AP. 'L' or 'l'   The lower triangular part of A is supplied in AP. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) 'L' or 'l', the array AP must contain the lower triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit. 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n not applicable or not stated by selected source not a workspace argument

## 3. `ALPHA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are scalars, x and y are n element vectors and A is an n by n symmetric matrix, supplied in packed form. Parameters ========== DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `AP`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION array of DIMENSION at least ( ( n*( n + 1))/2). contain a( 1, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. and a( 2, 2 ) respectively, and so on. contain a( 2, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. and a( 3, 1 ) respectively, and so on. Unchanged on exit. Unchanged on exit. DOUBLE PRECISION array of DIMENSION at least ( ( n*( n + 1))/2). contain a( 1, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. and a( 2, 2 ) respectively, and so on. contain a( 2, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. and a( 3, 1 ) respectively, and so on. Unchanged on exit. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION array of dimension at least must not be zero. Unchanged on exit. DOUBLE PRECISION array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 7. `BETA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are scalars, x and y are n element vectors and A is an n by n symmetric matrix, supplied in packed form. Parameters ========== DOUBLE PRECISION. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `Y`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). = alpha*A*x + beta*y, DOUBLE PRECISION array of dimension at least is overwritten by the updated is overwritten by the updated vector y. vector y. must not be zero. Unchanged on exit. = alpha*A*x + beta*y, DOUBLE PRECISION array of dimension at least is overwritten by the updated is overwritten by the updated vector y. vector y. must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 9. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCY specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

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
- `AP`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument
- `BETA`: not a workspace argument
- `Y`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::dspmv`
- Original SLATEC routine: `DSPMV`
- Native symbol: `dspmv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSPMV](https://www.netlib.org/slatec/lin/dspmv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
