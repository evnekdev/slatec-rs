# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPDR is the BSPLDR routine of the reference. DBSPDR uses the B-representation (T,A,N,K) to construct a divided difference table ADIF preparatory to a (right) derivative calculation in DBSPEV. The lower triangular matrix ADIF is stored in vector AD by columns. The arrays are related by ADIF(I,J) = AD(I-J+1 + (2*N-J+2)*(J-1)/2) I = J,N , J=1,NDERIV.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPDR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPDR](https://www.netlib.org/slatec/src/dbspdr.f).

# Arguments

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

knot vector of length N+K.

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

B-spline coefficient vector of length N.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of B-spline coefficients sum of knot multiplicities-K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the spline, K. GE. 1.

## `NDERIV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of derivatives, 1. LE. NDERIV. K. 1 gives the zero-th derivative = function value Output AD is double precision.

## `AD`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

table of differences in a vector of length (2*N-NDERIV+1)*NDERIV/2 for input to DBSPEV.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `AD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbspdr`
- Original SLATEC routine: `DBSPDR`
- Native symbol: `dbspdr_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSPDR](https://www.netlib.org/slatec/src/dbspdr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
