# Purpose

Abstract PPQAD computes the integral on (X1,X2) of a K-th order B-spline using the piecewise polynomial representation (C,XI,LXI,K). Here the Taylor expansion about the left

# Description

This canonical unsafe binding exposes original SLATEC routine `PPQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PPQAD](https://www.netlib.org/slatec/src/ppqad.f).

# Arguments

## 1. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. leading dimension of matrix C, LDC .GE. K leading dimension of matrix C, LDC .GE. K leading dimension of matrix C, LDC .GE. K not a workspace argument

## 2. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDC, *). Here the Taylor expansion about the left right Taylor derivatives at XI(J), I=1,K , J=1,LXI not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `XI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Here the Taylor expansion about the left th interval is integrated and break point array of length LXI+1 .LE. X .LE. XI(LXI+1) Here the Taylor expansion about the left th interval is integrated and break point array of length LXI+1 .LE. X .LE. XI(LXI+1) not applicable or not stated by selected source not a workspace argument

## 4. `LXI`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Here the Taylor expansion about the left number of polynomial pieces not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. th order B-spline using the piecewise polynomial (PP) representation. th order B-spline using the piecewise polynomial representation Here the Taylor expansion about the left order of B-spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `X1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. th order B-spline using the piecewise polynomial (PP) representation. th order B-spline using the piecewise polynomial representation are formed by included break points.  Integration outside (XI(1),XI(LXI+1)) is permitted. end points of quadrature interval, normally in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `X2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. th order B-spline using the piecewise polynomial (PP) representation. th order B-spline using the piecewise polynomial representation are formed by included break points.  Integration outside (XI(1),XI(LXI+1)) is permitted. end points of quadrature interval, normally in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `PQUAD`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. integral of the PP representation over (X1,X2) not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error

# Workspace and array requirements

- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument
- `LXI`: not a workspace argument
- `K`: not a workspace argument
- `X1`: not a workspace argument
- `X2`: not a workspace argument
- `PQUAD`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::ppqad`
- Original SLATEC routine: `PPQAD`
- Native symbol: `ppqad_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [PPQAD](https://www.netlib.org/slatec/src/ppqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
