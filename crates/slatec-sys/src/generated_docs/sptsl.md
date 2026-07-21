# Purpose

SPTSL given a positive definite tridiagonal matrix and a right hand side will find the solution. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPTSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPTSL](https://www.netlib.org/slatec/lin/sptsl.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the order of the tridiagonal matrix.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) is the diagonal of the tridiagonal matrix. On output, D is destroyed.

## `E`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) is the offdiagonal of the tridiagonal matrix. E(1) through E(N-1) should contain the offdiagonal.

## `B`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(N) is the right hand side vector. contains the solution.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `D`: not a workspace argument
- `E`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::sptsl`
- Original SLATEC routine: `SPTSL`
- Native symbol: `sptsl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SPTSL](https://www.netlib.org/slatec/lin/sptsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
