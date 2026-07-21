# Purpose

DSIDI computes the determinant, inertia and inverse of a double precision symmetric matrix using the factors from DSIFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DSIDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSIDI](https://www.netlib.org/slatec/lin/dsidi.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA,N) the output from DSIFA. contains the upper triangle of the inverse of the original matrix. The strict lower triangle is never referenced.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from DSIFA.

## `DET`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

DOUBLE PRECISION(2) determinant of original matrix. DETERMINANT = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)).

## `INERT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (3).

INTEGER(3) the inertia of the original matrix. number of positive eigenvalues. number of negative eigenvalues. number of zero eigenvalues. Error Condition A division by zero may occur if the inverse is requested and DSICO has set RCOND. EQ.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) work vector. Contents destroyed.

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
- `WORK`: DOUBLE PRECISION(N) work vector. Contents destroyed.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsidi`
- Original SLATEC routine: `DSIDI`
- Native symbol: `dsidi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DSIDI](https://www.netlib.org/slatec/lin/dsidi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
