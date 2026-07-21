# Purpose

CGEMM performs one of the matrix-matrix operations

# Description

This canonical unsafe binding exposes original SLATEC routine `CGEMM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGEMM](https://www.netlib.org/slatec/lin/cgemm.f).

# Arguments

## 1. `TRANSA`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( A ) = A. 'T' or 't',  op( A ) = A'. 'C' or 'c',  op( A ) = conjg( A' ). Unchanged on exit. 'N' or 'n',  and is  m  otherwise. 'N' or 'n',  the leading  m by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( A ) = A. 'T' or 't',  op( A ) = A'. 'C' or 'c',  op( A ) = conjg( A' ). Unchanged on exit. 'N' or 'n',  and is  m  otherwise. 'N' or 'n',  the leading  m by k part of the array  A  must contain the matrix  A,  otherwise 'N' or 'n' then not applicable or not stated by selected source not a workspace argument

## 2. `TRANSB`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( B ) = B. 'T' or 't',  op( B ) = B'. 'C' or 'c',  op( B ) = conjg( B' ). Unchanged on exit. 'N' or 'n',  and is  k  otherwise. 'N' or 'n',  the leading  k by n part of the array  B  must contain the matrix  B,  otherwise 'N' or 'n' then CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: 'N' or 'n',  op( B ) = B. 'T' or 't',  op( B ) = B'. 'C' or 'c',  op( B ) = conjg( B' ). Unchanged on exit. 'N' or 'n',  and is  k  otherwise. 'N' or 'n',  the leading  k by n part of the array  B  must contain the matrix  B,  otherwise 'N' or 'n' then not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry,  M  specifies  the number  of rows  of the  matrix must  be at least  zero. Unchanged on exit. contain  the matrix A. Unchanged on exit. INTEGER. On entry,  M  specifies  the number  of rows  of the  matrix must  be at least  zero. Unchanged on exit. contain  the matrix A. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry,  N  specifies the number  of columns of the matrix must be at least zero. Unchanged on exit. 'N' or 'n',  and is  k  otherwise. contain  the matrix B. Unchanged on exit. INTEGER. On entry,  N  specifies the number  of columns of the matrix must be at least zero. Unchanged on exit. 'N' or 'n',  and is  k  otherwise. contain  the matrix B. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry,  K  specifies  the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least  zero. Unchanged on exit. 'N' or 'n',  and is  m  otherwise. contain  the matrix A. Unchanged on exit. contain  the matrix B. Unchanged on exit. INTEGER. On entry,  K  specifies  the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least  zero. Unchanged on exit. 'N' or 'n',  and is  m  otherwise. contain  the matrix A. Unchanged on exit. contain  the matrix B. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 6. `ALPHA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. are scalars, and A, B and C are matrices, with op( A ) an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. Parameters ========== COMPLEX         . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). must  be at least  zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. must  be at least  zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDA, ka ), where ka is contain  the matrix A. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. not applicable or not stated by selected source not a workspace argument

## 8. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, k ). Unchanged on exit. not a workspace argument

## 9. `B`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDB, *). must be at least zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDB, kb ), where kb is contain  the matrix B. Unchanged on exit. must be at least zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDB, kb ), where kb is contain  the matrix B. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 10. `LDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDB specifies the first dimension of B as declared must be at least  max( 1, k ), otherwise  LDB must be at least  max( 1, n ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared must be at least  max( 1, k ), otherwise  LDB must be at least  max( 1, n ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared must be at least  max( 1, k ), otherwise  LDB must be at least  max( 1, n ). Unchanged on exit. not a workspace argument

## 11. `BETA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. are scalars, and A, B and C are matrices, with op( A ) an m by k matrix,  op( B )  a  k by n matrix and  C an m by n matrix. Parameters ========== COMPLEX         . On entry,  BETA  specifies the scalar  beta.  When  BETA  is supplied as zero then C need not be set on input. Unchanged on exit. is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n  matrix ( alpha*op( A )*op( B ) + beta*C ). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `C`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDC, *). = alpha*op( A )*op( B ) + beta*C, where  op( X ) is one of op( X ) = X   or   op( X ) = X'   or   op( X ) = conjg( X' ), must  be at least  zero. Unchanged on exit. must be at least zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n  matrix ( alpha*op( A )*op( B ) + beta*C ). = alpha*op( A )*op( B ) + beta*C, where  op( X ) is one of op( X ) = X   or   op( X ) = X'   or   op( X ) = conjg( X' ), must  be at least  zero. Unchanged on exit. must be at least zero. Unchanged on exit. COMPLEX          array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n  matrix ( alpha*op( A )*op( B ) + beta*C ). not applicable or not stated by selected source not a workspace argument

## 13. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `TRANSA`: not a workspace argument
- `TRANSB`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument
- `LDB`: not a workspace argument
- `BETA`: not a workspace argument
- `C`: not a workspace argument
- `LDC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::cgemm`
- Original SLATEC routine: `CGEMM`
- Native symbol: `cgemm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CGEMM](https://www.netlib.org/slatec/lin/cgemm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
