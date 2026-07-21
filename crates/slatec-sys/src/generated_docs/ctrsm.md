# Purpose

CTRSM solves one of the matrix equations op( A )*X = alpha*B, or X*op( A ) = alpha*B, where alpha is a scalar, X and B are m by n matrices, A is a unit, or non-unit, upper or lower triangular matrix and op( A ) is one of op( A ) = A or op( A ) = A' or op( A ) = conjg( A' ). The matrix X is overwritten on B.

# Description

This canonical unsafe binding exposes original SLATEC routine `CTRSM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTRSM](https://www.netlib.org/slatec/lin/ctrsm.f).

# Arguments

## `SIDE`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, SIDE specifies whether op( A ) appears on the left or right of X as follows: 'L' or 'l' op( A )*X = alpha*B. 'R' or 'r' X*op( A ) = alpha*B. Unchanged on exit.

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the matrix A is an upper or lower triangular matrix as follows: 'U' or 'u' A is an upper triangular matrix. 'L' or 'l' A is a lower triangular matrix. Unchanged on exit.

## `TRANSA`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n' op( A ) = A. 'T' or 't' op( A ) = A'. 'C' or 'c' op( A ) = conjg( A' ). Unchanged on exit.

## `DIAG`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: 'U' or 'u' A is assumed to be unit triangular. 'N' or 'n' A is not assumed to be unit Unchanged on exit.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, M specifies the number of rows of B. M must be at least zero. Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the number of columns of B. N must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

COMPLEX. On entry, ALPHA specifies the scalar alpha. When alpha is zero then A is not referenced and B need not be set before entry. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX array of DIMENSION ( LDA, k ), where k is m when SIDE = 'L' or 'l' and is n when SIDE = 'R' or 'r'. Before entry with UPLO = 'U' or 'u', the leading k by k upper triangular part of the array A must contain the upper triangular matrix and the strictly lower triangular part of is not referenced. Before entry with UPLO = 'L' or 'l', the leading k by k lower triangular part of the array A must contain the lower triangular matrix and the strictly upper triangular part of Note that when DIAG = 'U' or 'u', the diagonal elements of are not referenced either, but are assumed to be unity. Unchanged on exit.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When SIDE = 'L' or 'l' then must be at least max( 1, m ), when SIDE = 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit.

## `B`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDB, *).

COMPLEX array of DIMENSION ( LDB, n ). Before entry, the leading m by n part of the array B must contain the right-hand side matrix B, and on exit is overwritten by the solution matrix X.

## `LDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. LDB must be at least max( 1, m ). Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument
- `LDB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::ctrsm`
- Original SLATEC routine: `CTRSM`
- Native symbol: `ctrsm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CTRSM](https://www.netlib.org/slatec/lin/ctrsm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
