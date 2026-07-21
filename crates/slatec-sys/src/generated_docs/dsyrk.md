# Purpose

DSYRK performs one of the symmetric rank k operations

# Description

This canonical unsafe binding exposes original SLATEC routine `DSYRK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSYRK](https://www.netlib.org/slatec/lin/dsyrk.f).

# Arguments

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the array C is to be referenced as follows: 'U' or 'u' Only the upper triangular part of C is to be referenced. 'L' or 'l' Only the lower triangular part of C Unchanged on exit.

## `TRANS`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = alpha*A*A' + beta*C. 'T' or 't' C := alpha*A'*A + beta*C. Unchanged on exit. 'T' or 't' or 'C' or 'c', K specifies the number of rows of the matrix A. K must be at least zero.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= alpha*A*A' + beta*C. INTEGER. On entry, N specifies the order of the matrix C. N must be at least zero. Unchanged on exit.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry with TRANS = 'N' or 'n', K specifies the number of columns of the matrix A, and on entry with when TRANS = 'N' or 'n', and is n otherwise. Before entry with TRANS = 'N' or 'n', the leading n by k part of the array A must contain the matrix A, otherwise the leading k by n part of the array A must contain the matrix A. Unchanged on exit.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION array of DIMENSION ( LDA, ka ), where ka is.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When TRANS = 'N' or 'n' then LDA must be at least max( 1, n ), otherwise LDA must be at least max( 1, k ). Unchanged on exit.

## `BETA`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

DOUBLE PRECISION. On entry, BETA specifies the scalar beta. Unchanged on exit.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDC, *).

= alpha*A*A' + beta*C, or = alpha*A'*A + beta*C, where alpha and beta are scalars, C is an n by n symmetric matrix and A is an n by k matrix in the first case and a k by n matrix in the second case. = alpha*A*A' + beta*C. = alpha*A'*A + beta*C. Unchanged on exit. DOUBLE PRECISION array of DIMENSION ( LDC, n ). Before entry with UPLO = 'U' or 'u', the leading n by n upper triangular part of the array C must contain the upper triangular part of the symmetric matrix and the strictly lower triangular part of C is not referenced.

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, n ). Unchanged on exit.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `C`: not a workspace argument
- `LDC`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level3::dsyrk`
- Original SLATEC routine: `DSYRK`
- Native symbol: `dsyrk_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSYRK](https://www.netlib.org/slatec/lin/dsyrk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
