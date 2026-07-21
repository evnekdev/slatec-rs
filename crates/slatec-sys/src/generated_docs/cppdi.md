# Purpose

CPPDI computes the determinant and inverse of a complex Hermitian positive definite matrix using the factors computed by CPPCO or CPPFA . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPPDI](https://www.netlib.org/slatec/lin/cppdi.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the output from CPPCO or CPPFA. the upper triangular half of the inverse. The strict lower triangle is unaltered.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cppdi`
- Original SLATEC routine: `CPPDI`
- Native symbol: `cppdi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1,mut_i32)`
- Exact Netlib source file: [CPPDI](https://www.netlib.org/slatec/lin/cppdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
