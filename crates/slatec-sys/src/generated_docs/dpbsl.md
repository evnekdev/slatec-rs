# Purpose

DPBSL solves the double precision symmetric positive definite band system A*X = B using the factors computed by DPBCO or DPBFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPBSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPBSL](https://www.netlib.org/slatec/lin/dpbsl.f).

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

## `B`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

DOUBLE PRECISION(N) the right hand side vector. the solution vector X. Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically this indicates singularity, but it is usually caused by improper subroutine arguments. It will not occur if the subroutines are called correctly, and INFO. EQ.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::dpbsl`
- Original SLATEC routine: `DPBSL`
- Native symbol: `dpbsl_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DPBSL](https://www.netlib.org/slatec/lin/dpbsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
