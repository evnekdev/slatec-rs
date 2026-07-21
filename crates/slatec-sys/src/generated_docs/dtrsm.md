# Purpose

DTRSM solves one of the matrix equations

# Description

This canonical unsafe binding exposes original SLATEC routine `DTRSM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DTRSM](https://www.netlib.org/slatec/lin/dtrsm.f).

# Arguments

## 1. `SIDE`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, SIDE specifies whether op( A ) appears on the left or right of X as follows: 'L' or 'l'   op( A )*X = alpha*B. 'R' or 'r'   X*op( A ) = alpha*B. Unchanged on exit. 'L' or 'l'  and is  n  when  SIDE = 'R' or 'r'. 'L' or 'l'  then 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. CHARACTER*1. On entry, SIDE specifies whether op( A ) appears on the left or right of X as follows: 'L' or 'l'   op( A )*X = alpha*B. 'R' or 'r'   X*op( A ) = alpha*B. Unchanged on exit. 'L' or 'l'  and is  n  when  SIDE = 'R' or 'r'. 'L' or 'l'  then 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. not applicable or not stated by selected source not a workspace argument

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

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are m by n matrices, A is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of DOUBLE PRECISION. On entry,  ALPHA specifies the scalar  alpha. When  alpha is zero then  A is not referenced and  B need not be set before entry. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDA, *). alpha*B,   or   X*op( A ) = alpha*B, are m by n matrices, A is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of A   or   op( A ) = A'. The matrix X is overwritten on B. Parameters ========== is an upper or lower triangular matrix as follows: is unit triangular as follows: DOUBLE PRECISION array of DIMENSION ( LDA, k ), where k is m is not referenced. is not referenced. are not referenced either,  but are assumed to be  unity. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. alpha*B,   or   X*op( A ) = alpha*B, are m by n matrices, A is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of A   or   op( A ) = A'. The matrix X is overwritten on B. Parameters ========== is an upper or lower triangular matrix as follows: is unit triangular as follows: DOUBLE PRECISION array of DIMENSION ( LDA, k ), where k is m is not referenced. is not referenced. are not referenced either,  but are assumed to be  unity. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. not applicable or not stated by selected source not a workspace argument

## 9. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. not a workspace argument

## 10. `B`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDB, *). are m by n matrices, A is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of must be at least zero. Unchanged on exit. must be at least zero. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry,  the leading  m by n part of the array  B must contain  the  right-hand  side  matrix  B,  and  on exit  is overwritten by the solution matrix  X. are m by n matrices, A is a unit, or non-unit,  upper or lower triangular matrix  and  op( A )  is one  of must be at least zero. Unchanged on exit. must be at least zero. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry,  the leading  m by n part of the array  B must contain  the  right-hand  side  matrix  B,  and  on exit  is overwritten by the solution matrix  X. not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::blas::level3::dtrsm`
- Original SLATEC routine: `DTRSM`
- Native symbol: `dtrsm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DTRSM](https://www.netlib.org/slatec/lin/dtrsm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
