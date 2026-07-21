# Purpose

DSISL solves the double precision symmetric system A * X = B using the factors computed by DSIFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DSISL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSISL](https://www.netlib.org/slatec/lin/dsisl.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA,N) the output from DSIFA.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from DSIFA.

## `B`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) the right hand side vector. the solution vector X. Error Condition A division by zero may occur if DSICO has set RCOND. EQ. 0. 0 or DSIFA has set INFO.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `KPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dsisl`
- Original SLATEC routine: `DSISL`
- Native symbol: `dsisl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DSISL](https://www.netlib.org/slatec/lin/dsisl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
