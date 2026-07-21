# Purpose

SGEDI computes the determinant and inverse of a matrix using the factors computed by SGECO or SGEFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SGEDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGEDI](https://www.netlib.org/slatec/lin/sgedi.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

REAL(LDA, N) the output from SGECO or SGEFA. inverse of original matrix if requested. Otherwise unchanged.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `IPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from SGECO or SGEFA.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) work vector. Contents destroyed.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument
- `DET`: not a workspace argument
- `WORK`: REAL(N) work vector. Contents destroyed.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgedi`
- Original SLATEC routine: `SGEDI`
- Native symbol: `sgedi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SGEDI](https://www.netlib.org/slatec/lin/sgedi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
