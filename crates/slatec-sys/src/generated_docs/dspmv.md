# Purpose

DSPMV performs the matrix-vector operation

# Description

This canonical unsafe binding exposes original SLATEC routine `DSPMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSPMV](https://www.netlib.org/slatec/lin/dspmv.f).

# Arguments

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed array AP as follows: 'U' or 'u' The upper triangular part of A is supplied in AP. 'L' or 'l' The lower triangular part of A is Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `AP`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION array of DIMENSION at least ( ( n*( n + 1))/2). Before entry with UPLO = 'U' or 'u', the array AP must contain the upper triangular part of the symmetric matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. Before entry with UPLO = 'L' or 'l', the array AP must contain the lower triangular part of the symmetric matrix contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. Unchanged on exit.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION array of dimension at least ( 1 + ( n - 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit.

## `BETA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

= alpha*A*x + beta*y, where alpha and beta are scalars, x and y are n element vectors and A is an n by n symmetric matrix, supplied in packed form. DOUBLE PRECISION array of dimension at least ( 1 + ( n - 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n element vector y. On exit, Y is overwritten by the updated.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `X`: not a workspace argument
- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::dspmv`
- Original SLATEC routine: `DSPMV`
- Native symbol: `dspmv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSPMV](https://www.netlib.org/slatec/lin/dspmv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
