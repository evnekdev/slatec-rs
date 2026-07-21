# Purpose

CGTSL given a general tridiagonal matrix and a right hand side will find the solution. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGTSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGTSL](https://www.netlib.org/slatec/lin/cgtsl.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER is the order of the tridiagonal matrix.

## `C`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) is the subdiagonal of the tridiagonal matrix. C(2) through C(N) should contain the subdiagonal. On output C is destroyed.

## `D`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) is the diagonal of the tridiagonal matrix. On output D is destroyed.

## `E`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) is the superdiagonal of the tridiagonal matrix. E(1) through E(N-1) should contain the superdiagonal. On output E is destroyed.

## `B`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) is the right hand side vector. is the solution vector.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER = 0 normal value. = K if the K-th element of the diagonal becomes exactly zero. The subroutine returns when this is detected.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal value. = K if the K-th element of the diagonal becomes exactly zero. The subroutine returns when this is detected. |

# Workspace and array requirements

- `C`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cgtsl`
- Original SLATEC routine: `CGTSL`
- Native symbol: `cgtsl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGTSL](https://www.netlib.org/slatec/lin/cgtsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
