# Purpose

CHIDI computes the determinant, inertia and inverse of a complex Hermitian matrix using the factors from CHIFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CHIDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHIDI](https://www.netlib.org/slatec/lin/chidi.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA,N) the output from CHIFA. contains the upper triangle of the inverse of the original matrix. The strict lower triangle is never referenced.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from CHIFA.

## `DET`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (2).

REAL(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

## `INERT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (3).

INTEGER(3) the inertia of the original matrix. number of positive eigenvalues. number of negative eigenvalues. number of zero eigenvalues. Error Condition A division by zero may occur if the inverse is requested and CHICO has set RCOND. EQ.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) work vector. Contents destroyed.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER JOB has the decimal expansion ABC where if C. NE. 0, the inverse is computed, if B. 0, the determinant is computed, if A. 0, the inertia is computed. For example, JOB = 111 gives all three.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `KPVT`: not a workspace argument
- `DET`: not a workspace argument
- `INERT`: not a workspace argument
- `WORK`: COMPLEX(N) work vector. Contents destroyed.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chidi`
- Original SLATEC routine: `CHIDI`
- Native symbol: `chidi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_f32_array_rank1,mut_i32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CHIDI](https://www.netlib.org/slatec/lin/chidi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
