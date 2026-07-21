# Purpose

Written by Carl de Boor and modified by D. E. Amos INTRV is the INTERV routine of the reference. INTRV computes the largest integer ILEFT in 1 .LE. ILEFT .LE.

# Description

This canonical unsafe binding exposes original SLATEC routine `INTRV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INTRV](https://www.netlib.org/slatec/src/intrv.f).

# Arguments

## 1. `XT`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a subdivision is a subdivision of the X interval. of the X interval. is a subdivision of is a subdivision of the X interval.  Precisely, the X interval.  Precisely, 1 I  , MFLAG=0 I  , MFLAG=0 .LE. X                         LXT        1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. XT is a knot or break point vector of length LXT is a subdivision is a subdivision of the X interval. of the X interval. is a subdivision of is a subdivision of the X interval.  Precisely, the X interval.  Precisely, 1 I  , MFLAG=0 I  , MFLAG=0 .LE. X                         LXT        1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. XT is a knot or break point vector of length LXT not applicable or not stated by selected source not a workspace argument

## 2. `LXT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a subdivision of the X interval.  Precisely, .LE. X                         LXT        1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. length of the XT vector is a subdivision of the X interval.  Precisely, .LE. X                         LXT        1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. length of the XT vector not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a subdivision of the X interval. is a subdivision of the X interval.  Precisely, 1 I  , MFLAG=0 argument not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ILO`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. an initialization parameter which must be set to 1 the first time the spline array XT is processed by INTRV. ILO contains information for efficient process- ing after the initial call, and ILO must not be changed by the user.  Distinct splines require distinct ILO parameters. an initialization parameter which must be set to 1 the first time the spline array XT is processed by INTRV. ILO contains information for efficient process- ing after the initial call, and ILO must not be changed by the user.  Distinct splines require distinct ILO parameters. not applicable or not stated by selected source not a workspace argument

## 5. `ILEFT`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a subdivision of the X interval. is a subdivision of the X interval.  Precisely, I  , MFLAG=0 largest integer satisfying XT(ILEFT) .LE. X not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `MFLAG`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. signals when X lies out of bounds not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

None

# Workspace and array requirements

- `XT`: not a workspace argument
- `LXT`: not a workspace argument
- `X`: not a workspace argument
- `ILO`: not a workspace argument
- `ILEFT`: not a workspace argument
- `MFLAG`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::intrv`
- Original SLATEC routine: `INTRV`
- Native symbol: `intrv_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [INTRV](https://www.netlib.org/slatec/src/intrv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
