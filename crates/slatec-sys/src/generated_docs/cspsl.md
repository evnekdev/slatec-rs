# Purpose

CSISL solves the complex symmetric system A * X = B using the factors computed by CSPFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSPSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSPSL](https://www.netlib.org/slatec/lin/cspsl.f).

# Arguments

## `AP`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N*(N+1)/2) the output from CSPFA.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `KPVT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

INTEGER(N) the pivot vector from CSPFA.

## `B`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) the right hand side vector. the solution vector X. Error Condition A division by zero may occur if CSPCO has set RCOND. EQ. 0. 0 or CSPFA has set INFO.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `KPVT`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cspsl`
- Original SLATEC routine: `CSPSL`
- Native symbol: `cspsl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1)`
- Exact Netlib source file: [CSPSL](https://www.netlib.org/slatec/lin/cspsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
