# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPDR is the BSPLDR routine of the reference. BSPDR uses the B-representation (T,A,N,K) to construct a divided difference table ADIF preparatory to a (right) derivative calculation in BSPEV. The lower triangular matrix ADIF is stored in vector AD by columns. The arrays are related by ADIF(I,J) = AD(I-J+1 + (2*N-J+2)*(J-1)/2) I = J,N , J = 1,NDERIV .

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPDR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPDR](https://www.netlib.org/slatec/src/bspdr.f).

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

order of the spline, K. GE. 1.

## `NDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of derivatives, 1. LE. NDERIV. K. 1 gives the zero-th derivative = function value.

## `AD`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

table of differences in a vector of length (2*N-NDERIV+1)*NDERIV/2 for input to BSPEV.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `AD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bspdr`
- Original SLATEC routine: `BSPDR`
- Native symbol: `bspdr_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSPDR](https://www.netlib.org/slatec/src/bspdr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
