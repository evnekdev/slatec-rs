# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPVD is the BSPLVD routine of the reference. BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPVD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPVD](https://www.netlib.org/slatec/src/bspvd.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). .LE. X .LT. T(ILEFT+1).  A call to INTRV(T,N+1,X, 1,N. 1,N. N produces left limiting values at the right end point knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT. T(ILEFT+1) .LE. X .LT. T(ILEFT+1).  A call to INTRV(T,N+1,X, 1,N. 1,N. N produces left limiting values at the right end point knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT. T(ILEFT+1) not applicable or not stated by selected source not a workspace argument

## 2. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. th basis 1,N. order of the B-spline, K .GE. 1 .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `NDERIV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. of all basis functions which do not vanish at X. 1 right derivatives at X, I=1,K, J=1,NDERIV. number of derivatives = NDERIV-1, 1 .LE. NDERIV .LE. K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is input such that T(ILEFT+1) then VNIKX contains left limiting values T(N+1). To obtain left limiting values at T(I), I=K+1,N+1, next lower distinct knot, call INTRV to get ILEFT, T(I), and then call BSPVD. argument of basis functions, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `ILEFT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is input such that .LE. X .LT. T(ILEFT+1).  A call to INTRV(T,N+1,X, will produce the proper ILEFT.  The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and K+I, I=1,K, th basis N N produces left limiting values at the right end point produces left limiting values at the right end point largest integer such that .LE. X .LT. T(ILEFT+1) is input such that .LE. X .LT. T(ILEFT+1).  A call to INTRV(T,N+1,X, will produce the proper ILEFT.  The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and K+I, I=1,K, th basis N N produces left limiting values at the right end point produces left limiting values at the right end point largest integer such that .LE. X .LT. T(ILEFT+1) not applicable or not stated by selected source not a workspace argument

## 6. `LDVNIK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. leading dimension of matrix VNIKX leading dimension of matrix VNIKX leading dimension of matrix VNIKX not a workspace argument

## 7. `VNIKX`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDVNIK, *). matrix of dimension at least (K,NDERIV) contain- ing the nonzero basis functions at X and their derivatives columnwise. matrix of dimension at least (K,NDERIV) contain- ing the nonzero basis functions at X and their derivatives columnwise. not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a work vector of length (K+1)*(K+2)/2 a work vector of length (K+1)*(K+2)/2 not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error

# Workspace and array requirements

- `T`: not a workspace argument
- `K`: not a workspace argument
- `NDERIV`: not a workspace argument
- `X`: not a workspace argument
- `ILEFT`: not a workspace argument
- `LDVNIK`: not a workspace argument
- `VNIKX`: not a workspace argument
- `WORK`: a work vector of length (K+1)*(K+2)/2

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bspvd`
- Original SLATEC routine: `BSPVD`
- Native symbol: `bspvd_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSPVD](https://www.netlib.org/slatec/src/bspvd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
