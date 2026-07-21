# Purpose

SGER performs the rank 1 operation

# Description

This canonical unsafe binding exposes original SLATEC routine `SGER`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGER](https://www.netlib.org/slatec/lin/sger.f).

# Arguments

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, M specifies the number of rows of the matrix A. must be at least zero. Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the number of columns of the matrix A. must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL array of dimension at least ( 1 + ( m - 1)*abs( INCX)). Before entry, the incremented array X must contain the m element vector x. Unchanged on exit.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit.

## `Y`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL array of dimension at least ( 1 + ( n - 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n element vector y. Unchanged on exit.

## `INCY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

= alpha*x*y' + A, where alpha is a scalar, x is an m element vector, y is an n element vector and A is an m by n matrix. REAL array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. On exit, A is overwritten by the updated matrix.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::sger`
- Original SLATEC routine: `SGER`
- Native symbol: `sger_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SGER](https://www.netlib.org/slatec/lin/sger.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
