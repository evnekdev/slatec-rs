# Purpose

STRMM performs one of the matrix-matrix operations

# Description

This canonical unsafe binding exposes original SLATEC routine `STRMM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [STRMM](https://www.netlib.org/slatec/lin/strmm.f).

# Arguments

## 1. `SIDE`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry,  SIDE specifies whether  op( A ) multiplies B from the left or right as follows: = alpha*op( A )*B. = alpha*B*op( A ). Unchanged on exit. 'L' or 'l'  and is  n  when  SIDE = 'R' or 'r'. 'L' or 'l'  then 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. CHARACTER*1. On entry,  SIDE specifies whether  op( A ) multiplies B from the left or right as follows: = alpha*op( A )*B. = alpha*B*op( A ). Unchanged on exit. 'L' or 'l'  and is  n  when  SIDE = 'R' or 'r'. 'L' or 'l'  then 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 2. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u',  the  leading  k by k upper triangular part of the array  A must contain the upper triangular matrix  and the strictly lower triangular part of 'L' or 'l',  the  leading  k by k lower triangular part of the array  A must contain the lower triangular matrix  and the strictly upper triangular part of CHARACTER*1. is an upper or lower triangular matrix as follows: 'U' or 'u'   A is an upper triangular matrix. 'L' or 'l'   A is a lower triangular matrix. Unchanged on exit. 'U' or 'u',  the  leading  k by k upper triangular part of the array  A must contain the upper triangular matrix  and the strictly lower triangular part of 'L' or 'l',  the  leading  k by k lower triangular part of the array  A must contain the lower triangular matrix  and the strictly upper triangular part of not applicable or not stated by selected source not a workspace argument

## 3. `TRANSA`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n'   op( A ) = A. 'T' or 't'   op( A ) = A'. 'C' or 'c'   op( A ) = A'. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DIAG`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. is unit triangular as follows: 'U' or 'u'   A is assumed to be unit triangular. 'N' or 'n'   A is not assumed to be unit triangular. Unchanged on exit. 'U' or 'u',  the diagonal elements of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. must be at must be at least zero. least zero. Unchanged on exit. Unchanged on exit. 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. must be at must be at least zero. least zero. Unchanged on exit. Unchanged on exit. 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. must be must be at least zero. at least zero. Unchanged on exit. Unchanged on exit. INTEGER. must be must be at least zero. at least zero. Unchanged on exit. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 7. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL            . On entry,  ALPHA specifies the scalar  alpha. When  alpha is zero then  A is not referenced and  B need not be set before entry. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). A   or   op( A ) = A'. Parameters ========== is an upper or lower triangular matrix as follows: is unit triangular as follows: REAL             array of DIMENSION ( LDA, k ), where k is m is not referenced. is not referenced. are not referenced either,  but are assumed to be  unity. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. A   or   op( A ) = A'. Parameters ========== is an upper or lower triangular matrix as follows: is unit triangular as follows: REAL             array of DIMENSION ( LDA, k ), where k is m is not referenced. is not referenced. are not referenced either,  but are assumed to be  unity. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. not applicable or not stated by selected source not a workspace argument

## 9. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. not a workspace argument

## 10. `B`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDB, *). = alpha*op( A )*B,   or   B := alpha*B*op( A ), where  alpha  is a scalar,  B  is an m by n matrix,  A  is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of = alpha*op( A )*B. = alpha*B*op( A ). Unchanged on exit. must be at least zero. Unchanged on exit. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDB, n ). Before entry,  the leading  m by n part of the array  B must contain the matrix  B,  and  on exit  is overwritten  by the transformed matrix. = alpha*op( A )*B,   or   B := alpha*B*op( A ), where  alpha  is a scalar,  B  is an m by n matrix,  A  is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of = alpha*op( A )*B. = alpha*B*op( A ). Unchanged on exit. must be at least zero. Unchanged on exit. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDB, n ). Before entry,  the leading  m by n part of the array  B must contain the matrix  B,  and  on exit  is overwritten  by the transformed matrix. not applicable or not stated by selected source not a workspace argument

## 11. `LDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `SIDE`: not a workspace argument
- `UPLO`: not a workspace argument
- `TRANSA`: not a workspace argument
- `DIAG`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument
- `LDB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::strmm`
- Original SLATEC routine: `STRMM`
- Native symbol: `strmm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [STRMM](https://www.netlib.org/slatec/lin/strmm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
