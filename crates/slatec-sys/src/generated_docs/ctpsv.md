# Purpose

CTPSV solves one of the systems of equations

# Description

This canonical unsafe binding exposes original SLATEC routine `CTPSV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTPSV](https://www.netlib.org/slatec/lin/ctpsv.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), 'L' or 'l', the array AP must contain the lower triangular matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u', the array AP must contain the upper triangular matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), 'L' or 'l', the array AP must contain the lower triangular matrix packed sequentially, column by column, so that AP( 1 ) contains a( 1, 1 ), not applicable or not stated by selected source not a workspace argument

## 2. `TRANS`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANS specifies the equations to be solved as follows: 'N' or 'n'   A*x = b. 'T' or 't'   A'*x = b. 'C' or 'c'   conjg( A' )*x = b. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DIAG`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: 'U' or 'u'   A is assumed to be unit triangular. 'N' or 'n'   A is not assumed to be unit triangular. Unchanged on exit. 'U' or 'u', the diagonal elements of A are not referenced, but are assumed to be unity. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an n by n unit, or non-unit, upper or lower triangular matrix, supplied in packed form. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element right-hand side vector b. On exit, X is overwritten with the solution vector x. is an n by n unit, or non-unit, upper or lower triangular matrix, supplied in packed form. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element right-hand side vector b. On exit, X is overwritten with the solution vector x. not applicable or not stated by selected source not a workspace argument

## 5. `AP`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX          array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). contain a( 1, 2 ) and a( 2, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. respectively, and so on. contain a( 2, 1 ) and a( 3, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. respectively, and so on. COMPLEX          array of DIMENSION at least ( ( n*( n + 1 ) )/2 ). contain a( 1, 2 ) and a( 2, 2 ) contain a( 1, 2 ) and a( 2, 2 ) respectively, and so on. respectively, and so on. contain a( 2, 1 ) and a( 3, 1 ) contain a( 2, 1 ) and a( 3, 1 ) respectively, and so on. respectively, and so on. not applicable or not stated by selected source not a workspace argument

## 6. `X`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix, supplied in packed form. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== COMPLEX          array of dimension at least must not be zero. Unchanged on exit. b,   or   A'*x = b,   or   conjg( A')*x = b, is an n by n unit, or non-unit, upper or lower triangular matrix, supplied in packed form. No test for singularity or near-singularity is included in this routine. Such tests must be performed before calling this routine. Parameters ========== COMPLEX          array of dimension at least must not be zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 7. `INCX`

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
- `AP`: not a workspace argument
- `X`: not a workspace argument
- `INCX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level2::ctpsv`
- Original SLATEC routine: `CTPSV`
- Native symbol: `ctpsv_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CTPSV](https://www.netlib.org/slatec/lin/ctpsv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
