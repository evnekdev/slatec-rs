# Purpose

CHPR performs the hermitian rank 1 operation A := alpha*x*conjg( x') + A, where alpha is a real scalar, x is an n element vector and A is an n by n hermitian matrix, supplied in packed form.

# Description

This canonical unsafe binding exposes original SLATEC routine `CHPR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHPR](https://www.netlib.org/slatec/lin/chpr.f).

# Arguments

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the matrix A is supplied in the packed array AP as follows: 'U' or 'u' The upper triangular part of A is supplied in AP. 'L' or 'l' The lower triangular part of A is Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `X`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX array of dimension at least ( 1 + ( n - 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. Unchanged on exit.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit.

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). Before entry with UPLO = 'U' or 'u', the array AP must contain the upper triangular part of the hermitian matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. On exit, the array is overwritten by the upper triangular part of the updated matrix. Before entry with UPLO = 'L' or 'l', the array AP must contain the lower triangular part of the hermitian matrix contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. On exit, the array is overwritten by the lower triangular part of the Note that the imaginary parts of the diagonal elements need not be set, they are assumed to be zero, and on exit they are set to zero.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `AP`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::chpr`
- Original SLATEC routine: `CHPR`
- Native symbol: `chpr_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CHPR](https://www.netlib.org/slatec/lin/chpr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
