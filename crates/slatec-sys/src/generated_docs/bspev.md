# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPEV is the BSPLEV routine of the reference. BSPEV calculates the value of the spline and its derivatives

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPEV](https://www.netlib.org/slatec/src/bspev.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1 and set X=T(I), I=K+1,N+1. BSPEV calls INTRV, BSPVN knot vector of length N+K 1 and set X=T(I), I=K+1,N+1. BSPEV calls INTRV, BSPVN knot vector of length N+K not applicable or not stated by selected source not a workspace argument

## 2. `AD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a vector of length (2*N-NDERIV+1)*NDERIV/2 containing the difference table from BSPDR. T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a vector of length (2*N-NDERIV+1)*NDERIV/2 containing the difference table from BSPDR. not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a 1 and set X=T(I), I=K+1,N+1. BSPEV calls INTRV, BSPVN number of B-spline coefficients K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a order of the B-spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `NDERIV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a number of derivatives, 1 .LE. NDERIV .LE. K. th derivative = function value 1 derivatives in the remaining components. T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a number of derivatives, 1 .LE. NDERIV .LE. K. th derivative = function value 1 derivatives in the remaining components. not applicable or not stated by selected source not a workspace argument

## 6. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. representation (T,A,N,K) and returns them T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a argument, T(K) .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INEV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. an initialization parameter which must be set to 1 the first time BSPEV is called. INEV contains information for efficient process- ing after the initial call and INEV must not be changed by the user.  Distinct splines require distinct INEV parameters. an initialization parameter which must be set to 1 the first time BSPEV is called. INEV contains information for efficient process- ing after the initial call and INEV must not be changed by the user.  Distinct splines require distinct INEV parameters. not applicable or not stated by selected source not a workspace argument

## 8. `SVALUE`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,NDERIV, T(K) .LE. X .LE. T(N+1).  AD(I) can be the B-spline coefficients A(I), I=1,N if NDERIV=1.  Other- wise AD must be computed before hand by a call to BSPDR (T,A, vector of length NDERIV containing the spline 1 derivatives in the remaining components. 1,NDERIV, T(K) .LE. X .LE. T(N+1).  AD(I) can be the B-spline coefficients A(I), I=1,N if NDERIV=1.  Other- wise AD must be computed before hand by a call to BSPDR (T,A, vector of length NDERIV containing the spline 1 derivatives in the remaining components. not applicable or not stated by selected source not a workspace argument

## 9. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). work vector of length 3*K work vector of length 3*K not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error.

# Workspace and array requirements

- `T`: not a workspace argument
- `AD`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `NDERIV`: not a workspace argument
- `X`: not a workspace argument
- `INEV`: not a workspace argument
- `SVALUE`: not a workspace argument
- `WORK`: work vector of length 3*K

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bspev`
- Original SLATEC routine: `BSPEV`
- Native symbol: `bspev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSPEV](https://www.netlib.org/slatec/src/bspev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
