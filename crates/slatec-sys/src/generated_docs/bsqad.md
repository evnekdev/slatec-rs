# Purpose

Abstract BSQAD computes the integral on (X1,X2) of a K-th order B-spline using the B-representation (T,BCOEF,N,K). Orders K as high as 20 are permitted by applying a 2, 6, or 10 point Gauss formula on subintervals of (X1,X2) which are formed by included (distinct) knots. If orders K greater than 20 are needed, use BFQAD with F(X) = 1.

# Description

This canonical unsafe binding exposes original SLATEC routine `BSQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSQAD](https://www.netlib.org/slatec/src/bsqad.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). knot array of length N+K .LE. X .LE. T(N+1) knot array of length N+K .LE. X .LE. T(N+1) not applicable or not stated by selected source not a workspace argument

## 2. `BCOEF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). B-spline coefficient array of length N B-spline coefficient array of length N not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. length of coefficient array length of coefficient array not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. th order B-spline using the B-representation. th order B-spline using the B-representation (T,BCOEF,N,K).  Orders as high as 20 are permitted by applying a 2, 6, or 10 point Gauss formula on subintervals of (X1,X2) which are formed by included (distinct) knots. If orders K greater than 20 are needed, use BFQAD with F(X) = 1. order of B-spline, 1 .LE. K .LE. 20 .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `X1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. th order B-spline using the B-representation (T,BCOEF,N,K).  Orders end points of quadrature interval in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `X2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. th order B-spline using the B-representation (T,BCOEF,N,K).  Orders end points of quadrature interval in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `BQUAD`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. integral of the B-spline over (X1,X2) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). work vector of length 3*K work vector of length 3*K not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error

# Workspace and array requirements

- `T`: not a workspace argument
- `BCOEF`: not a workspace argument
- `N`: not a workspace argument
- `K`: not a workspace argument
- `X1`: not a workspace argument
- `X2`: not a workspace argument
- `BQUAD`: not a workspace argument
- `WORK`: work vector of length 3*K

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::bsqad`
- Original SLATEC routine: `BSQAD`
- Native symbol: `bsqad_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSQAD](https://www.netlib.org/slatec/src/bsqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
