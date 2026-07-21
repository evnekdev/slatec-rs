# Purpose

DNBDI computes the determinant of a band matrix using the factors computed by DNBCO or DNBFA. If the inverse is needed, use DNBSL N times. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DNBDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DNBDI](https://www.netlib.org/slatec/src/dnbdi.f).

# Arguments

## `ABE`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, NC) the output from DNBCO or DNBFA. NC must be. GE. 2*ML+MU+1.

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

INTEGER(N) the pivot vector from DNBCO or DNBFA.

## `DET`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

DOUBLE PRECISION(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `ABE`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dnbdi`
- Original SLATEC routine: `DNBDI`
- Native symbol: `dnbdi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DNBDI](https://www.netlib.org/slatec/src/dnbdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
