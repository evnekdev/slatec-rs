# Purpose

DPPDI computes the determinant and inverse of a double precision symmetric positive definite matrix using the factors computed by DPPCO or DPPFA . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPPDI](https://www.netlib.org/slatec/lin/dppdi.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION (N*(N+1)/2) the output from DPPCO or DPPFA. the upper triangular half of the inverse. The strict lower triangle is unaltered.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `DET`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

DOUBLE PRECISION(2) determinant of original matrix if requested. Otherwise not referenced. DETERMINANT = DET(1) * 10. 0**DET(2) with 1. 0. LE.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::dppdi`
- Original SLATEC routine: `DPPDI`
- Native symbol: `dppdi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPPDI](https://www.netlib.org/slatec/lin/dppdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
