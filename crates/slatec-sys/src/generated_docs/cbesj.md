# Purpose

On KODE=1, CBESJ computes an N member sequence of complex Bessel functions CY(L)=J(FNU+L-1,Z) for real nonnegative orders FNU+L-1, L=1,...,N and complex Z in the cut plane -pi<arg(Z)<=pi. On KODE=2, CBESJ returns the scaled functions

# Description

This canonical unsafe binding exposes original SLATEC routine `CBESJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBESJ](https://www.netlib.org/slatec/src/cbesj.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Argument of type COMPLEX.

## `FNU`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Initial order of type REAL, FNU>=0.

## `KODE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

A parameter to indicate the scaling option 1 returns.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of terms in the sequence, N>=1.

## `CY`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (N).

exp(-abs(Y))*J(FNU+L-1,Z), L=1,. ,N and Y=Im(Z) which remove the exponential growth in both the upper and lower half planes as Z goes to infinity. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1). =2 returns J(FNU+L-1,Z)*exp(-abs(Y)), L=1,. ,N where Y=Im(Z) Result vector of type COMPLEX.

## `NZ`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Number of underflows set to zero NZ=0 Normal return NZ>0 CY(L)=0, L=N-NZ+1,. ,N.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Error flag 0 Normal return - COMPUTATION COMPLETED 1 Input error - NO COMPUTATION 2 Overflow - NO COMPUTATION (Im(Z) too large on KODE=1) 3 Precision warning - COMPUTATION COMPLETED (Result has half precision or less because abs(Z) or FNU+N-1 is large) 4 Precision error - NO COMPUTATION (Result has no precision because abs(Z) or FNU+N-1 is too large) 5 Algorithmic error - NO COMPUTATION (Termination condition not met).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `NZ` | `0` | Normal return , L=N-NZ+1,...,N |

# Workspace and array requirements

- `CY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cbesj`
- Original SLATEC routine: `CBESJ`
- Native symbol: `cbesj_`
- ABI fingerprint: `subroutine:void(mut_complex32,mut_f32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CBESJ](https://www.netlib.org/slatec/src/cbesj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
