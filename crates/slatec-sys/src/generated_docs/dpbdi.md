# Purpose

DPBDI computes the determinant of a double precision symmetric positive definite band matrix using the factors computed by DPBCO or DPBFA. If the inverse is needed, use DPBSL N times. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPBDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPBDI](https://www.netlib.org/slatec/lin/dpbdi.f).

# Arguments

## `ABD`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, N) the output from DPBCO or DPBFA.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array ABD.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the number of diagonals above the main diagonal.

## `DET`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

DOUBLE PRECISION(2) determinant of original matrix in the form DETERMINANT = DET(1) * 10. 0**DET(2) with 1. 0. LE. DET(1). LT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dpbdi`
- Original SLATEC routine: `DPBDI`
- Native symbol: `dpbdi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPBDI](https://www.netlib.org/slatec/lin/dpbdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
