# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPDR is the BSPLDR routine of the reference. DBSPDR uses the B-representation (T,A,N,K) to construct a divided difference table ADIF preparatory to a (right) derivative calculation in DBSPEV. The lower triangular matrix ADIF is stored in vector AD by columns. The arrays are related by

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPDR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPDR](https://www.netlib.org/slatec/src/dbspdr.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are double precision knot vector of length N+K are double precision knot vector of length N+K not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are double precision B-spline coefficient vector of length N are double precision B-spline coefficient vector of length N not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of B-spline coefficients K NDERIV+1)*NDERIV/2 for input to DBSPEV not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order of the spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `NDERIV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of derivatives, 1 .LE. NDERIV .LE. K. th derivative = function value Output     AD is double precision not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `AD`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). J+1 + (2*N-J+2)*(J-1)/2) I = J,N   ,   J=1,NDERIV. table of differences in a vector of length J+1 + (2*N-J+2)*(J-1)/2) I = J,N   ,   J=1,NDERIV. table of differences in a vector of length not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `NDERIV`: not a workspace argument
- `AD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbspdr`
- Original SLATEC routine: `DBSPDR`
- Native symbol: `dbspdr_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBSPDR](https://www.netlib.org/slatec/src/dbspdr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
