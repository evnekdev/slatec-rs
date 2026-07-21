# Purpose

CGEMV performs one of the matrix-vector operations

# Description

This canonical unsafe binding exposes original SLATEC routine `CGEMV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGEMV](https://www.netlib.org/slatec/lin/cgemv.f).

# Arguments

## `TRANS`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = alpha*A*x + beta*y. 'T' or 't' y := alpha*A'*x + beta*y. 'C' or 'c' y := alpha*conjg( A' )*x + beta*y. Unchanged on exit.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= alpha*A*x + beta*y. INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

COMPLEX. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. Unchanged on exit.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit.

## `X`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX array of DIMENSION at least ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. Before entry, the incremented array X must contain the vector x. Unchanged on exit.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit.

## `BETA`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

COMPLEX. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit.

## `Y`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

= alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, or = alpha*conjg( A' )*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n matrix. = alpha*A*x + beta*y. COMPLEX array of DIMENSION at least ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. Before entry with BETA non-zero, the incremented array Y must contain the vector y. On exit, Y is overwritten by the updated vector y.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `X`: not a workspace argument
- `Y`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::cgemv`
- Original SLATEC routine: `CGEMV`
- Native symbol: `cgemv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CGEMV](https://www.netlib.org/slatec/lin/cgemv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
