# Purpose

Written by Carl de Boor and modified by D. E. Amos BSPVN is the BSPLVN routine of the reference. BSPVN calculates the value of all (possibly) nonzero basis

# Description

This canonical unsafe binding exposes original SLATEC routine `BSPVN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BSPVN](https://www.netlib.org/slatec/src/bspvn.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). IWORK is set inside the routine IWORK is set inside the routine knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT. T(ILEFT+1) IWORK is set inside the routine IWORK is set inside the routine knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT. T(ILEFT+1) not applicable or not stated by selected source not a workspace argument

## 2. `JHIGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1)), where K and INDEX=1 can be faster than calling BSPVD, but extra coding is required for derivatives order of B-spline, 1 .LE. JHIGH .LE. K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IWORK is set inside the routine highest possible order .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INDEX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1)), where 1.  ILEFT is such that T(ILEFT) .LE. X .LT. T(ILEFT+1).  A call to INTRV(T,N+1,X,ILO,ILEFT, MFLAG) produces the proper ILEFT.  BSPVN calculates using the basic algorithm needed in BSPVD.  If only basis functions are 2) and BSPVD is set up for this purpose. Left limiting values are set up as described in BSPVD. INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with WORK, IWORK values saved for subsequent calls to BSPVN. 2. 1 exclusively, these are scratch variables and can be used for other purposes. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1)), where IWORK is set inside the routine argument of basis functions, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ILEFT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. largest integer such that .LE. X .LT. T(ILEFT+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `VNIKX`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). vector of length K for spline values. vector of length K for spline values. not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a work vector of length 2*K a work vector of length 2*K not applicable or not stated by selected source

## 9. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a work parameter.  Both WORK and IWORK contain not stated by selected source not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

Improper input is a fatal error.

# Workspace and array requirements

- `T`: not a workspace argument
- `JHIGH`: not a workspace argument
- `K`: not a workspace argument
- `INDEX`: not a workspace argument
- `X`: not a workspace argument
- `ILEFT`: not a workspace argument
- `VNIKX`: not a workspace argument
- `WORK`: a work vector of length 2*K
- `IWORK`: a work parameter.  Both WORK and IWORK contain

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::bspvn`
- Original SLATEC routine: `BSPVN`
- Native symbol: `bspvn_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [BSPVN](https://www.netlib.org/slatec/src/bspvn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
