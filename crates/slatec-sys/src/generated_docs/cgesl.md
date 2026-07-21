# Purpose

CGESL solves the complex system A * X = B or CTRANS(A) * X = B using the factors computed by CGECO or CGEFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGESL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGESL](https://www.netlib.org/slatec/lin/cgesl.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDA, *).

COMPLEX(LDA, N) the output from CGECO or CGEFA.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `IPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from CGECO or CGEFA.

## `B`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) the right hand side vector. the solution vector X. Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically this indicates singularity but it is often caused by improper arguments or improper setting of LDA. It will not occur if the subroutines are called correctly and if CGECO has set RCOND. GT.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 to solve A*X = B , = nonzero to solve CTRANS(A)*X = B where CTRANS(A) is the conjugate transpose.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `IPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cgesl`
- Original SLATEC routine: `CGESL`
- Native symbol: `cgesl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGESL](https://www.netlib.org/slatec/lin/cgesl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
