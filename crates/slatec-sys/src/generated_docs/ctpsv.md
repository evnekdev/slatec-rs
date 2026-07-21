# Purpose

CTPSV solves one of the systems of equations A*x = b, or A'*x = b, or conjg( A')*x = b, where b and x are n element vectors and A is an n by n unit, or non-unit, upper or lower triangular matrix, supplied in packed form. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine.

# Description

This canonical unsafe binding exposes original SLATEC routine `CTPSV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTPSV](https://www.netlib.org/slatec/lin/ctpsv.f).

# Arguments

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u' A is an upper triangular matrix. 'L' or 'l' A is a lower triangular matrix. Unchanged on exit.

## `TRANS`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANS specifies the equations to be solved as follows: 'N' or 'n' A*x = b. 'T' or 't' A'*x = b. 'C' or 'c' conjg( A' )*x = b. Unchanged on exit.

## `DIAG`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: 'U' or 'u' A is assumed to be unit triangular. 'N' or 'n' A is not assumed to be unit Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit.

## `AP`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). Before entry with UPLO = 'U' or 'u', the array AP must contain the upper triangular matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), AP( 2 ) and AP( 3 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. Before entry with UPLO = 'L' or 'l', the array AP must contain the lower triangular matrix packed sequentially, AP( 2 ) and AP( 3 ) contain a( 2, 1 ) and a( 3, 1 ) Note that when DIAG = 'U' or 'u', the diagonal elements of A are not referenced, but are assumed to be unity. Unchanged on exit.

## `X`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX array of dimension at least ( 1 + ( n - 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element right-hand side vector b. On exit, X is overwritten with the solution vector x.

## `INCX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::ctpsv`
- Original SLATEC routine: `CTPSV`
- Native symbol: `ctpsv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CTPSV](https://www.netlib.org/slatec/lin/ctpsv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
