# Purpose

SSPDI computes the determinant, inertia and inverse of a real symmetric matrix using the factors from SSPFA, where the matrix is stored in packed form. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SSPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSPDI](https://www.netlib.org/slatec/lin/sspdi.f).

# Arguments

## `AP`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL (N*(N+1)/2) the output from SSPFA. contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from SSPFA.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

## `INERT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (3).

INTEGER(3) the inertia of the original matrix. number of positive eigenvalues. number of negative eigenvalues. number of zero eigenvalues. Error Condition A division by zero will occur if the inverse is requested and SSPCO has set RCOND. EQ.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) work vector. Contents ignored.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER JOB has the decimal expansion ABC where If C. NE. 0, the inverse is computed, If B. 0, the determinant is computed, If A. 0, the inertia is computed. For example, JOB = 111 gives all three.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `KPVT`: not a workspace argument
- `DET`: not a workspace argument
- `INERT`: not a workspace argument
- `WORK`: REAL(N) work vector. Contents ignored.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sspdi`
- Original SLATEC routine: `SSPDI`
- Native symbol: `sspdi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SSPDI](https://www.netlib.org/slatec/lin/sspdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
