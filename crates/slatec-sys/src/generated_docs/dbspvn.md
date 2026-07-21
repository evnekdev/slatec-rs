# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVN is the BSPLVN routine of the reference. DBSPVN calculates the value of all (possibly) nonzero basis

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSPVN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSPVN](https://www.netlib.org/slatec/src/dbspvn.f).

# Arguments

## 1. `T`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). IWORK is set inside the routine on are double precision knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT.  T(ILEFT+1) IWORK is set inside the routine on are double precision knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K .LE. X .LE. T(N+1) .LE. X .LT.  T(ILEFT+1) not applicable or not stated by selected source not a workspace argument

## 2. `JHIGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1)), where T(K) K and INDEX=1 can be faster than calling DBSPVD, but extra coding is required for derivatives order of B-spline, 1 .LE. JHIGH .LE. K not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `K`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. highest possible order .LE. X .LE. T(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INDEX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1)), where T(K) 1.  ILEFT is such that T(ILEFT) .LE. 2) and DBSPVD is set up for this purpose. Left limiting values are set up as described in DBSPVD. INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with work, IWORK values saved for subsequent calls to DBSPVN. 2. 1 exclusively, these are scratch variables and can be used for other purposes. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1)), where T(K) IWORK is set inside the routine on .LT. T(ILEFT+1).  A call to DINTRV(T,N+1,X,ILO,ILEFT,MFLAG) produces the proper ILEFT.  DBSPVN calculates using the basic algorithm needed in DBSPVD.  If only basis functions are are double precision argument of basis functions, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ILEFT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. largest integer such that .LE. X .LT.  T(ILEFT+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `VNIKX`

output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are double precision vector of length K for spline values. are double precision vector of length K for spline values. not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are double precision a work vector of length 2*K are double precision a work vector of length 2*K not applicable or not stated by selected source

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
- `WORK`: are double precision a work vector of length 2*K
- `IWORK`: a work parameter.  Both WORK and IWORK contain

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbspvn`
- Original SLATEC routine: `DBSPVN`
- Native symbol: `dbspvn_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DBSPVN](https://www.netlib.org/slatec/src/dbspvn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
