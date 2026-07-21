# Purpose

SCHDC computes the Cholesky decomposition of a positive definite matrix. A pivoting option allows the user to estimate the condition of a positive definite matrix or determine the rank of a positive semidefinite matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SCHDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCHDC](https://www.netlib.org/slatec/lin/schdc.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

REAL(LDA,P). contains the matrix whose decomposition is to be computed. Only the upper half of A need be stored. The lower part of the array A is not referenced. A contains in its upper half the Cholesky factor of the matrix A as it has been permuted by pivoting.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array A.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the order of the matrix.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL. is a work array.

## `JPVT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(P). contains integers that control the selection of the pivot elements, if pivoting has been requested. Each diagonal element A(K,K) is placed in one of three classes according to the value of JPVT(K). If JPVT(K). GT. 0, then X(K) is an initial element.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

EQ. 0. INTEGER. is an integer that initiates column pivoting. If JOB. 0, no pivoting is done.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

contains the index of the last positive diagonal element of the Cholesky factor. For positive definite matrices INFO = P is the normal return. For pivoting with positive semidefinite matrices INFO will in general be less than P. However, INFO may be greater than the rank of A, since rounding error can cause an otherwise zero element to be positive. Indefinite systems will always cause INFO to be less than P.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`INFO` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `WORK`: REAL. is a work array.
- `JPVT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::schdc`
- Original SLATEC routine: `SCHDC`
- Native symbol: `schdc_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SCHDC](https://www.netlib.org/slatec/lin/schdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
