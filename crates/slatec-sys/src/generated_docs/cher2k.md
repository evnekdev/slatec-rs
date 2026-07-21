# Purpose

CHER2K performs one of the hermitian rank 2k operations

# Description

This canonical unsafe binding exposes original SLATEC routine `CHER2K`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHER2K](https://www.netlib.org/slatec/lin/cher2k.f).

# Arguments

## `UPLO`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the array C is to be referenced as follows: 'U' or 'u' Only the upper triangular part of C is to be referenced. 'L' or 'l' Only the lower triangular part of C Unchanged on exit.

## `TRANS`

**Direction:** `input`. **Fortran type:** `CHARACTER`. **Rust ABI type:** `*mut core::ffi::c_char`. **Shape:** scalar.

CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = alpha*A*conjg( B' ) + conjg( alpha )*B*conjg( A' ) + = alpha*conjg( A' )*B + conjg( alpha )*conjg( B' )*A + 'C' or 'c', K specifies the number of rows of the matrices A and B. K must be at least zero. Unchanged on exit.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= alpha*A*conjg( B' ) + conjg( alpha )*B*conjg( A' ) + INTEGER. On entry, N specifies the order of the matrix C. N must be at least zero. Unchanged on exit.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry with TRANS = 'N' or 'n', K specifies the number of columns of the matrices A and B, and on entry with when TRANS = 'N' or 'n', and is n otherwise. Before entry with TRANS = 'N' or 'n', the leading n by k part of the array A must contain the matrix A, otherwise the leading k by n part of the array A must contain the matrix A. Unchanged on exit. part of the array B must contain the matrix B, otherwise the leading k by n part of the array B must contain the matrix B.

## `ALPHA`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

COMPLEX. On entry, ALPHA specifies the scalar alpha. Unchanged on exit.

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX array of DIMENSION ( LDA, ka ), where ka is.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When TRANS = 'N' or 'n' then LDA must be at least max( 1, n ), otherwise LDA must be at least max( 1, k ). Unchanged on exit.

## `B`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDB, *).

COMPLEX array of DIMENSION ( LDB, kb ), where kb is.

## `LDB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. When TRANS = 'N' or 'n' then LDB must be at least max( 1, n ), otherwise LDB must be at least max( 1, k ). Unchanged on exit.

## `BETA`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Unchanged on exit. REAL. On entry, BETA specifies the scalar beta.

## `C`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDC, *).

= alpha*A*conjg( B' ) + conjg( alpha )*B*conjg( A' ) + beta*C, or = alpha*conjg( A' )*B + conjg( alpha )*conjg( B' )*A + beta*C, where alpha and beta are scalars with beta real, C is an n by n hermitian matrix and A and B are n by k matrices in the first case and k by n matrices in the second case. Unchanged on exit. COMPLEX array of DIMENSION ( LDC, n ). Before entry with UPLO = 'U' or 'u', the leading n by n upper triangular part of the array C must contain the upper triangular part of the hermitian matrix and the strictly lower triangular part of C is not referenced. On exit, the upper triangular part of the array C is overwritten by the upper triangular part of the updated matrix. Before entry with UPLO = 'L' or 'l', the leading n by n lower triangular part of the array C must contain the lower upper triangular part of C is not referenced.

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, n ). Unchanged on exit.

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

- Canonical Rust path: `slatec_sys::blas::level3::cher2k`
- Original SLATEC routine: `CHER2K`
- Native symbol: `cher2k_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CHER2K](https://www.netlib.org/slatec/lin/cher2k.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
