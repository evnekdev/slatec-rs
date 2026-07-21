# Purpose

DSYMM performs one of the matrix-matrix operations

# Description

This canonical unsafe binding exposes original SLATEC routine `DSYMM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSYMM](https://www.netlib.org/slatec/lin/dsymm.f).

# Arguments

## 1. `SIDE`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On entry,  SIDE  specifies whether  the  symmetric matrix  A appears on the  left or right  in the  operation as follows: = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. 'L' or 'l'  and is  n otherwise. 'L' or 'l',  the  m by m  part of the array  A  must contain the  symmetric matrix,  such that 'R' or 'r',  the  n by n  part of the array  A  must contain the  symmetric matrix,  such that 'L' or 'l'  then CHARACTER*1. On entry,  SIDE  specifies whether  the  symmetric matrix  A appears on the  left or right  in the  operation as follows: = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. 'L' or 'l'  and is  n otherwise. 'L' or 'l',  the  m by m  part of the array  A  must contain the  symmetric matrix,  such that 'R' or 'r',  the  n by n  part of the array  A  must contain the  symmetric matrix,  such that 'L' or 'l'  then not applicable or not stated by selected source not a workspace argument

## 2. `UPLO`

input `scalar` argument; Fortran declaration `CHARACTER`, Rust ABI type `*mut core::ffi::c_char`, and scalar. CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of  the  symmetric  matrix   A  is  to  be referenced as follows: 'U' or 'u'   Only the upper triangular part of the symmetric matrix is to be referenced. 'L' or 'l'   Only the lower triangular part of the symmetric matrix is to be referenced. Unchanged on exit. 'U' or 'u', the leading m by m upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'U' or 'u', the leading n by n upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. CHARACTER*1. On  entry,   UPLO  specifies  whether  the  upper  or  lower triangular  part  of  the  symmetric  matrix   A  is  to  be referenced as follows: 'U' or 'u'   Only the upper triangular part of the symmetric matrix is to be referenced. 'L' or 'l'   Only the lower triangular part of the symmetric matrix is to be referenced. Unchanged on exit. 'U' or 'u', the leading m by m upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'U' or 'u', the leading n by n upper triangular part of the array  A  must contain the upper triangular part of the  symmetric matrix and the  strictly  lower triangular 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 3. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry,  M  specifies the number of rows of the matrix  C. must be at least zero. Unchanged on exit. 'L' or 'l'  and is  n otherwise. INTEGER. On entry,  M  specifies the number of rows of the matrix  C. must be at least zero. Unchanged on exit. 'L' or 'l'  and is  n otherwise. not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, N specifies the number of columns of the matrix C. must be at least zero. Unchanged on exit. INTEGER. On entry, N specifies the number of columns of the matrix C. must be at least zero. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 5. `ALPHA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. is a symmetric matrix and  B and DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDA, *). is a symmetric matrix and  B and DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. is a symmetric matrix and  B and DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is 'L' or 'l', the leading  m by m  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. 'L' or 'l', the leading  n by n  lower triangular part  of the  array  A must  contain  the  lower triangular part  of the  symmetric matrix and the  strictly upper triangular part of  A  is not referenced. Unchanged on exit. set of level 3 basic linear algebra subprograms. ACM TOMS, Vol. 16, No. 1, pp. 1-17, March 1990. not applicable or not stated by selected source not a workspace argument

## 7. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, n ). Unchanged on exit. INTEGER. On entry, LDA specifies the first dimension of A as declared must be at least  max( 1, m ), otherwise  LDA must be at least  max( 1, n ). Unchanged on exit. not a workspace argument

## 8. `B`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDB, *). DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry, the leading  m by n part of the array  B  must contain the matrix B. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry, the leading  m by n part of the array  B  must contain the matrix B. Unchanged on exit. not applicable or not stated by selected source not a workspace argument

## 9. `LDB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDB specifies the first dimension of B as declared in  the  calling  (sub)  program.   LDB  must  be  at  least max( 1, m ). Unchanged on exit. not a workspace argument

## 10. `BETA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. is a symmetric matrix and  B and DOUBLE PRECISION. On entry,  BETA  specifies the scalar  beta.  When  BETA  is supplied as zero then C need not be set on input. Unchanged on exit. is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n updated matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `C`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDC, *). = alpha*A*B + beta*C, or = alpha*B*A + beta*C, are  m by n matrices. Parameters ========== = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n updated matrix. = alpha*A*B + beta*C, or = alpha*B*A + beta*C, are  m by n matrices. Parameters ========== = alpha*A*B + beta*C, = alpha*B*A + beta*C, Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading  m by n  part of the array  C must is zero, in which case C need not be set on entry. On exit, the array  C  is overwritten by the  m by n updated matrix. not applicable or not stated by selected source not a workspace argument

## 12. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. INTEGER. On entry, LDC specifies the first dimension of C as declared in  the  calling  (sub)  program.   LDC  must  be  at  least max( 1, m ). Unchanged on exit. not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `SIDE`: not a workspace argument
- `UPLO`: not a workspace argument
- `M`: not a workspace argument
- `N`: not a workspace argument
- `ALPHA`: not a workspace argument
- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument
- `LDB`: not a workspace argument
- `BETA`: not a workspace argument
- `C`: not a workspace argument
- `LDC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::dsymm`
- Original SLATEC routine: `DSYMM`
- Native symbol: `dsymm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSYMM](https://www.netlib.org/slatec/lin/dsymm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
