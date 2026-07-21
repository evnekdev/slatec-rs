# Purpose

CPPSL solves the complex Hermitian positive definite system A * X = B using the factors computed by CPPCO or CPPFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPPSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPPSL](https://www.netlib.org/slatec/lin/cppsl.f).

# Arguments

## `AP`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX (N*(N+1)/2) the output from CPPCO or CPPFA.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER the order of the matrix A.

## `B`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) the right hand side vector. the solution vector X. Error Condition A division by zero will occur if the input factor contains a zero on the diagonal. Technically this indicates singularity but it is usually caused by improper subroutine arguments. It will not occur if the subroutines are called correctly and INFO. EQ.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AP`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cppsl`
- Original SLATEC routine: `CPPSL`
- Native symbol: `cppsl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1)`
- Exact Netlib source file: [CPPSL](https://www.netlib.org/slatec/lin/cppsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
