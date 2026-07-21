# Purpose

DSYMM performs one of the matrix-matrix operations

# Description

This canonical unsafe binding exposes original SLATEC routine `DSYMM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSYMM](https://www.netlib.org/slatec/lin/dsymm.f).

# Arguments

## `SIDE`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, SIDE specifies whether the symmetric matrix A appears on the left or right in the operation as follows: 'L' or 'l' C := alpha*A*B + beta*C, 'R' or 'r' C := alpha*B*A + beta*C, Unchanged on exit.

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the symmetric matrix A is to be referenced as follows: 'U' or 'u' Only the upper triangular part of the symmetric matrix is to be referenced. 'L' or 'l' Only the lower triangular part of the Unchanged on exit.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, M specifies the number of rows of the matrix C. must be at least zero. Unchanged on exit. when SIDE = 'L' or 'l' and is n otherwise. Before entry with SIDE = 'L' or 'l', the m by m part of the array A must contain the symmetric matrix, such that when UPLO = 'U' or 'u', the leading m by m upper triangular part of the array A must contain the upper triangular part of the symmetric matrix and the strictly lower triangular part of A is not referenced, and when UPLO = 'L' or 'l', the leading m by m lower triangular part of the array A must contain the lower triangular part of the symmetric matrix and the strictly upper triangular part of A is not referenced.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, N specifies the number of columns of the matrix C. must be at least zero. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When SIDE = 'L' or 'l' then must be at least max( 1, m ), otherwise LDA must be at least max( 1, n ). Unchanged on exit.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDB, *).

DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry, the leading m by n part of the array B must contain the matrix B. Unchanged on exit.

## `LDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. LDB must be at least max( 1, m ). Unchanged on exit.

## `BETA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then C need not be set on input. Unchanged on exit.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDC, *).

= alpha*A*B + beta*C, or = alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and are m by n matrices. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry, the leading m by n part of the array C must contain the matrix C, except when beta is zero, in which case C need not be set on entry. On exit, the array C is overwritten by the m by n updated matrix.

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

- Canonical Rust path: `slatec_sys::blas::level3::dsymm`
- Original SLATEC routine: `DSYMM`
- Native symbol: `dsymm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSYMM](https://www.netlib.org/slatec/lin/dsymm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
