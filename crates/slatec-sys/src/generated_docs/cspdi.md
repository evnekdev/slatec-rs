# Purpose

CSPDI computes the determinant and inverse of a complex symmetric matrix using the factors from CSPFA, where the matrix is stored in packed form. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSPDI](https://www.netlib.org/slatec/lin/cspdi.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the output from CSPFA. contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from CSPFA.

## `DET`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (2).

COMPLEX(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) work vector. Contents ignored.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER JOB has the decimal expansion AB where if B. NE. 0, the inverse is computed, if A. 0, the determinant is computed. For example, JOB = 11 gives both.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `KPVT`: not a workspace argument
- `DET`: not a workspace argument
- `WORK`: COMPLEX(N) work vector. Contents ignored.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cspdi`
- Original SLATEC routine: `CSPDI`
- Native symbol: `cspdi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CSPDI](https://www.netlib.org/slatec/lin/cspdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
