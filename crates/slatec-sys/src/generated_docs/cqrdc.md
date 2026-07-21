# Purpose

CQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. Column pivoting based on the 2-norms of the reduced columns may be performed at the users option. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CQRDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CQRDC](https://www.netlib.org/slatec/lin/cqrdc.f).

# Arguments

## `X`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDX, *).

COMPLEX(LDX,P), where LDX. GE. N. contains the matrix whose decomposition is to be computed. X contains in its upper triangle the upper triangular matrix R of the QR factorization. Below its diagonal X contains information from which the unitary part of the decomposition can be recovered.

## `LDX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array X.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of rows of the matrix X.

## `P`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of columns of the matrix X.

## `QRAUX`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). contains further information required to recover the unitary part of the decomposition.

## `JPVT`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(P). contains integers that control the selection of the pivot columns. The K-th column X(K) of X is placed in one of three classes according to the value of JVPT(K). If JVPT(K). GT. 0, then X(K) is an initial column.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). is a work array. WORK is not referenced if.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

EQ. 0. INTEGER. is an integer that initiates column pivoting. If JOB. 0, no pivoting is done.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `QRAUX`: not a workspace argument
- `JPVT`: not a workspace argument
- `WORK`: COMPLEX(P). is a work array. WORK is not referenced if.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cqrdc`
- Original SLATEC routine: `CQRDC`
- Native symbol: `cqrdc_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CQRDC](https://www.netlib.org/slatec/lin/cqrdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
