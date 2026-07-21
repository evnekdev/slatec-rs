# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPPP is the BSPLPP routine of the reference. BSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with PPVAL. Here XI(*), the break point array of length LXI, is the knot array T(*) with multiplicities removed. The columns of the matrix C(I,J) contain the right Taylor derivatives for the polynomial expansion about XI(J) for the intervals

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPPP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPPP](https://www.netlib.org/slatec/src/bsppp.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). knot vector of length N+K knot vector of length N+K not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). B-spline coefficient vector of length N B-spline coefficient vector of length N not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of B-spline coefficients K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. order of the B-spline, K .GE. 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `LDC`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. leading dimension of C, LDC .GE. K leading dimension of C, LDC .GE. K leading dimension of C, LDC .GE. K not a workspace argument

## 6. `C`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDC, *). matrix of dimension at least (K,LXI) containing right derivatives at break points matrix of dimension at least (K,LXI) containing right derivatives at break points not applicable or not stated by selected source not a workspace argument

## 7. `XI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,K, J=1,LXI.  Function PPVAL 1,K, J=1,LXI.  Function PPVAL makes this evaluation at a specified point X in makes this evaluation at a specified point X in .LE. X .LE. XI(LXI(1) .LE. X .LE. XI+1) XI break point vector of length LXI+1 1,K, J=1,LXI.  Function PPVAL 1,K, J=1,LXI.  Function PPVAL makes this evaluation at a specified point X in makes this evaluation at a specified point X in .LE. X .LE. XI(LXI(1) .LE. X .LE. XI+1) XI break point vector of length LXI+1 not applicable or not stated by selected source not a workspace argument

## 8. `LXI`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of break points, LXI .LE. N-K+1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). work vector of length K*(N+3) work vector of length K*(N+3) not applicable or not stated by selected source

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
- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument
- `LXI`: not a workspace argument
- `WORK`: work vector of length K*(N+3)

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bsppp`
- Original SLATEC routine: `BSPPP`
- Native symbol: `bsppp_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BSPPP](https://www.netlib.org/slatec/src/bsppp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
