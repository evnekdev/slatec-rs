# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. DBVALU evaluates the B-representation (T,A,N,K) of a B-spline

# Description

This canonical unsafe binding exposes original SLATEC routine `DBVALU`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBVALU](https://www.netlib.org/slatec/src/dbvalu.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV are double precision knot vector of length N+K 1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV are double precision knot vector of length N+K not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). is outside of this interval. To compute left derivatives or left limiting values at a are double precision B-spline coefficient vector of length N is outside of this interval. To compute left derivatives or left limiting values at a are double precision B-spline coefficient vector of length N not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV number of B-spline coefficients K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1.  Right limiting values (right derivatives) are returned except at the right end order of the B-spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IDERIV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 0 or any of its 1.  Right limiting values (right derivatives) are returned except at the right end order of the derivative, 0 .LE. IDERIV .LE. K-1 spline value not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 0 or any of its T(N+1) where left limiting values are computed.  The spline is defined on T(K) .LE. X .LE. T(N+1).  DBVALU returns is outside of this interval. To compute left derivatives or left limiting values at a are double precision argument, T(K) .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INBV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. an initialization parameter which must be set to 1 the first time DBVALU is called. Output     WORK,DBVALU are double precision INBV contains information for efficient process- ing after the initial call and INBV must not be changed by the user.  Distinct splines require distinct INBV parameters. an initialization parameter which must be set to 1 the first time DBVALU is called. Output     WORK,DBVALU are double precision INBV contains information for efficient process- ing after the initial call and INBV must not be changed by the user.  Distinct splines require distinct INBV parameters. not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). work vector of length 3*K. DBVALU  - value of the IDERIV-th derivative at X work vector of length 3*K. DBVALU  - value of the IDERIV-th derivative at X not applicable or not stated by selected source

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

An improper input is a fatal error

# Workspace and array requirements

- `T`: not a workspace argument
- `A`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `IDERIV`: not a workspace argument
- `X`: not a workspace argument
- `INBV`: not a workspace argument
- `WORK`: work vector of length 3*K. DBVALU  - value of the IDERIV-th derivative at X

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbvalu`
- Original SLATEC routine: `DBVALU`
- Native symbol: `dbvalu_`
- ABI fingerprint: `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBVALU](https://www.netlib.org/slatec/src/dbvalu.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
