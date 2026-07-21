# Purpose

CTRCO estimates the condition of a complex triangular matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CTRCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTRCO](https://www.netlib.org/slatec/lin/ctrco.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDT, *).

COMPLEX(LDT,N) contains the triangular matrix. The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information.

## `LDT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the leading dimension of the array T.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the order of the system.

## `RCOND`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

REAL an estimate of the reciprocal condition of T. For the system T*X = B , relative perturbations in T and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND. If RCOND is so small that the logical expression 1. 0 + RCOND. EQ. 1.

## `Z`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) a work vector whose contents are usually unimportant. If T is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z).

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 T is lower triangular. = nonzero T is upper triangular.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDT`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::ctrco`
- Original SLATEC routine: `CTRCO`
- Native symbol: `ctrco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_f32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CTRCO](https://www.netlib.org/slatec/lin/ctrco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
