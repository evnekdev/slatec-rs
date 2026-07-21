# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPPP is the BSPLPP routine of the reference. BSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with PPVAL. Here XI(*), the break point array of length LXI, is the knot array T(*) with multiplicities removed. The columns of the matrix C(I,J) contain the right Taylor derivatives for the polynomial expansion about XI(J) for the intervals

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPPP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPPP](https://www.netlib.org/slatec/src/bsppp.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

knot vector of length N+K.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

B-spline coefficient vector of length N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of B-spline coefficients sum of knot multiplicities-K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the B-spline, K. GE. 1.

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

leading dimension of C, LDC. GE. K.

## `C`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDC, *).

matrix of dimension at least (K,LXI) containing right derivatives at break points.

## `XI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

LE. X. XI(J+1), I=1,K, J=1,LXI. Function PPVAL makes this evaluation at a specified point X in. XI(LXI(1). XI+1) XI break point vector of length LXI+1.

## `LXI`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of break points, LXI. LE. N-K+1.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

work vector of length K*(N+3).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument
- `WORK`: work vector of length K*(N+3)

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bsppp`
- Original SLATEC routine: `BSPPP`
- Native symbol: `bsppp_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSPPP](https://www.netlib.org/slatec/src/bsppp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
