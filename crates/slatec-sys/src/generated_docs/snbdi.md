# Purpose

SNBDI computes the determinant of a band matrix using the factors computed by SNBCO or SNBFA. If the inverse is needed, use SNBSL N times. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SNBDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SNBDI](https://www.netlib.org/slatec/src/snbdi.f).

# Arguments

## `ABE`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

REAL(LDA, NC) the output from SNBCO or SNBFA. NC must be. GE. 2*ML+MU+1.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array ABE.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the original matrix.

## `ML`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER number of diagonals below the main diagonal.

## `MU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER number of diagonals above the main diagonal.

## `IPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from SNBCO or SNBFA.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::snbdi`
- Original SLATEC routine: `SNBDI`
- Native symbol: `snbdi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SNBDI](https://www.netlib.org/slatec/src/snbdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
