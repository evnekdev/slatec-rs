# Purpose

CTRSV solves one of the systems of equations

# Description

This canonical unsafe binding exposes original SLATEC routine `CTRSV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTRSV](https://www.netlib.org/slatec/lin/ctrsv.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u', the leading n by n upper triangular part of the array A must contain the upper triangular matrix and the strictly lower triangular part of 'L' or 'l', the leading n by n lower triangular part of the array A must contain the lower triangular matrix and the strictly upper triangular part of CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u', the leading n by n upper triangular part of the array A must contain the upper triangular matrix and the strictly lower triangular part of 'L' or 'l', the leading n by n lower triangular part of the array A must contain the lower triangular matrix and the strictly upper triangular part of not applicable or not stated by selected source not a workspace argument

## 2. `TRANS`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANS specifies the equations to be solved as follows: 'N' or 'n'   A*x = b. 'T' or 't'   A'*x = b. 'C' or 'c'   conjg( A' )*x = b. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DIAG`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. is unit triangular as follows: 'U' or 'u'   A is assumed to be unit triangular. 'N' or 'n'   A is not assumed to be unit triangular. Unchanged on exit. 'U' or 'u', the diagonal elements of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element right-hand side vector b. On exit, X is overwritten with the solution vector x. is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element right-hand side vector b. On exit, X is overwritten with the solution vector x. not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== is unit triangular as follows: COMPLEX          array of DIMENSION ( LDA, n ). is not referenced. is not referenced. are not referenced either, but are assumed to be unity. Unchanged on exit. b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== is unit triangular as follows: COMPLEX          array of DIMENSION ( LDA, n ). is not referenced. is not referenced. are not referenced either, but are assumed to be unity. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, n ). Unchanged on exit. not a workspace argument

## 7. `X`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== COMPLEX          array of dimension at least must not be zero. Unchanged on exit. b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== COMPLEX          array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 8. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. INTEGER. On entry, INCX specifies the increment for the elements of must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `UPLO`: not a workspace argument
- `TRANS`: not a workspace argument
- `DIAG`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::ctrsv`
- Original SLATEC routine: `CTRSV`
- Native symbol: `ctrsv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CTRSV](https://www.netlib.org/slatec/lin/ctrsv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
