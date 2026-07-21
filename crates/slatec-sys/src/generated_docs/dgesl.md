# Purpose

DGESL solves the double precision system A * X = B or TRANS(A) * X = B using the factors computed by DGECO or DGEFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DGESL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGESL](https://www.netlib.org/slatec/lin/dgesl.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDA, *).

DOUBLE PRECISION(LDA, N) the output from DGECO or DGEFA.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `IPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from DGECO or DGEFA.

## `B`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) the right hand side vector. the solution vector X. Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA. It will not occur if the subroutines are called correctly and if DGECO has set RCOND. GT.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 to solve A*X = B , = nonzero to solve TRANS(A)*X = B where TRANS(A) is the transpose.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dgesl`
- Original SLATEC routine: `DGESL`
- Native symbol: `dgesl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DGESL](https://www.netlib.org/slatec/lin/dgesl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
