# Purpose

DGEMM performs one of the matrix-matrix operations

# Description

This canonical unsafe binding exposes original SLATEC routine `DGEMM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGEMM](https://www.netlib.org/slatec/lin/dgemm.f).

# Arguments

## `TRANSA`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n', op( A ) = A. 'T' or 't', op( A ) = A'. 'C' or 'c', op( A ) = A'. Unchanged on exit.

## `TRANSB`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: 'N' or 'n', op( B ) = B. 'T' or 't', op( B ) = B'. 'C' or 'c', op( B ) = B'. Unchanged on exit.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, M specifies the number of rows of the matrix op( A ) and of the matrix C. M must be at least zero. Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the number of columns of the matrix op( B ) and the number of columns of the matrix C. N must be at least zero. Unchanged on exit. when TRANSB = 'N' or 'n', and is k otherwise. Before entry with TRANSB = 'N' or 'n', the leading k by n part of the array B must contain the matrix B, otherwise the leading n by k part of the array B must contain the matrix B.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, K specifies the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least zero. Unchanged on exit. when TRANSA = 'N' or 'n', and is m otherwise. Before entry with TRANSA = 'N' or 'n', the leading m by k part of the array A must contain the matrix A, otherwise the leading k by m part of the array A must contain the matrix A.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

scalars, and A, B and C are matrices, with op( A ) an m by k matrix, op( B ) a k by n matrix and C an m by n matrix. DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When TRANSA = 'N' or 'n' then must be at least max( 1, m ), otherwise LDA must be at least max( 1, k ). Unchanged on exit.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDB, *).

DOUBLE PRECISION array of DIMENSION ( LDB, kb ), where kb is.

## `LDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. When TRANSB = 'N' or 'n' then must be at least max( 1, k ), otherwise LDB must be at least max( 1, n ). Unchanged on exit.

## `BETA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

scalars, and A, B and C are matrices, with op( A ) an m by k matrix, op( B ) a k by n matrix and C an m by n matrix. DOUBLE PRECISION. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then C need not be set on input. Unchanged on exit.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDC, *).

= alpha*op( A )*op( B ) + beta*C, where op( X ) is one of op( X ) = X or op( X ) = X', DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading m by n part of the array C must contain the matrix C, except when beta is zero, in which case C need not be set on entry. On exit, the array C is overwritten by the m by n matrix ( alpha*op( A )*op( B ) + beta*C ).

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, m ). Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument
- `LDB`: not a workspace argument
- `C`: not a workspace argument
- `LDC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::dgemm`
- Original SLATEC routine: `DGEMM`
- Native symbol: `dgemm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGEMM](https://www.netlib.org/slatec/lin/dgemm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
