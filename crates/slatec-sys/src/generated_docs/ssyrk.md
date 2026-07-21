# Purpose

SSYRK performs one of the symmetric rank k operations

# Description

This canonical unsafe binding exposes original SLATEC routine `SSYRK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSYRK](https://www.netlib.org/slatec/lin/ssyrk.f).

# Arguments

## 1. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of the  array  C  is to be  referenced  as follows: 'U' or 'u'   Only the  upper triangular part of  C is to be referenced. 'L' or 'l'   Only the  lower triangular part of  C is to be referenced. Unchanged on exit. 'U' or 'u',  the leading  n by n upper triangular part of the array C must contain the upper triangular part  of the  symmetric matrix  and the strictly lower triangular part of C is not referenced.  On exit, the upper triangular part of the array  C is overwritten by the upper triangular part of the updated matrix. 'L' or 'l',  the leading  n by n lower triangular part of the array C must contain the lower triangular part  of the  symmetric matrix  and the strictly upper triangular part of C is not referenced.  On exit, the lower triangular part of the array  C is overwritten by the lower triangular part of the updated matrix. CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of the  array  C  is to be  referenced  as follows: 'U' or 'u'   Only the  upper triangular part of  C is to be referenced. 'L' or 'l'   Only the  lower triangular part of  C is to be referenced. Unchanged on exit. 'U' or 'u',  the leading  n by n upper triangular part of the array C must contain the upper triangular part  of the  symmetric matrix  and the strictly lower triangular part of C is not referenced.  On exit, the upper triangular part of the array  C is overwritten by the upper triangular part of the updated matrix. 'L' or 'l',  the leading  n by n lower triangular part of the array C must contain the lower triangular part  of the  symmetric matrix  and the strictly upper triangular part of C is not referenced.  On exit, the lower triangular part of the array  C is overwritten by the lower triangular part of the updated matrix. not applicable or not stated by selected source not a workspace argument

## 2. `TRANS`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry,  TRANS  specifies the operation to be performed as follows: = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. 'N' or 'n',  K  specifies  the number of  columns   of  the   matrix   A,   and  on   entry   with 'T' or 't' or 'C' or 'c',  K  specifies  the  number 'N' or 'n',  and is  n  otherwise. 'N' or 'n',  the  leading  n by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then  LDA must be at least  max( 1, n ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. CHARACTER*1. On entry,  TRANS  specifies the operation to be performed as follows: = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. 'N' or 'n',  K  specifies  the number of  columns   of  the   matrix   A,   and  on   entry   with 'T' or 't' or 'C' or 'c',  K  specifies  the  number 'N' or 'n',  and is  n  otherwise. 'N' or 'n',  the  leading  n by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then  LDA must be at least  max( 1, n ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. = alpha*A*A' + beta*C. = alpha*A*A' + beta*C. INTEGER. must be must be at least zero. at least zero. Unchanged on exit. Unchanged on exit. contain  the matrix A. Unchanged on exit. = alpha*A*A' + beta*C. = alpha*A*A' + beta*C. INTEGER. must be must be at least zero. at least zero. Unchanged on exit. Unchanged on exit. contain  the matrix A. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. must be at least zero. Unchanged on exit. 'N' or 'n',  and is  n  otherwise. contain  the matrix A. Unchanged on exit. INTEGER. must be at least zero. Unchanged on exit. 'N' or 'n',  and is  n  otherwise. contain  the matrix A. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `ALPHA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== REAL            . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. not applicable or not stated by selected source not a workspace argument

## 7. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared INTEGER. On entry, LDA specifies the first dimension of A as declared INTEGER. On entry, LDA specifies the first dimension of A as declared not a workspace argument

## 8. `BETA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== REAL            . On entry, BETA specifies the scalar beta. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `C`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDC, *). = alpha*A*A' + beta*C, or = alpha*A'*A + beta*C, is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. Unchanged on exit. Unchanged on exit. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDC, n ). = alpha*A*A' + beta*C, or = alpha*A'*A + beta*C, is an  n by n  symmetric matrix and  A  is an  n by k  matrix in the first case and a  k by n  matrix in the second case. Parameters ========== = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. Unchanged on exit. Unchanged on exit. must be at least zero. Unchanged on exit. REAL             array of DIMENSION ( LDC, n ). not applicable or not stated by selected source not a workspace argument

## 10. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, n ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, n ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `UPLO`: not a workspace argument
- `TRANS`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `BETA`: not a workspace argument
- `C`: not a workspace argument
- `LDC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::ssyrk`
- Original SLATEC routine: `SSYRK`
- Native symbol: `ssyrk_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SSYRK](https://www.netlib.org/slatec/lin/ssyrk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
