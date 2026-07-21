# Purpose

CPODI computes the determinant and inverse of a certain complex Hermitian positive definite matrix (see below) using the factors computed by CPOCO, CPOFA or CQRDC. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPODI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPODI](https://www.netlib.org/slatec/lin/cpodi.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the output A from CPOCO or CPOFA or the output X from CQRDC. If CPOCO or CPOFA was used to factor A then CPODI produces the upper half of INVERSE(A). If CQRDC was used to decompose X then CPODI produces the upper half of INVERSE(CTRANS(X)*X) where CTRANS(X) is the conjugate transpose. Elements of A below the diagonal are unchanged. If the units digit of JOB is zero, A is unchanged.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of A or of CTRANS(X)*X if requested. Otherwise not referenced. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpodi`
- Original SLATEC routine: `CPODI`
- Native symbol: `cpodi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_f32_array_rank1,mut_i32)`
- Exact Netlib source file: [CPODI](https://www.netlib.org/slatec/lin/cpodi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
