# Purpose

CHFEV: Cubic Hermite Function EValuator Evaluates the cubic polynomial determined by function values

# Description

This canonical unsafe binding exposes original SLATEC routine `CHFEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHFEV](https://www.netlib.org/slatec/pchip/chfev.f).

# Arguments

## 1. `X1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (input) endpoints of interval of definition of cubic. is returned in NEXT. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (input) endpoints of interval of definition of cubic. is returned in NEXT. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `F1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. and derivatives D1,D2 on interval (X1,X2) at the points (input) values of function at X1 and X2, respectively. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `F2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. and derivatives D1,D2 on interval (X1,X2) at the points (input) values of function at X1 and X2, respectively. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `D1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (input) values of derivative at X1 and X2, respectively. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `D2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (input) values of derivative at X1 and X2, respectively. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) number of evaluation points.  (Error return if .) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `XE`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1(1)NE. (input) real array of points at which the function is to be evaluated.  If any of the XE are outside the interval not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `FE`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). (output) real array of values of the cubic function defined by  X1,X2, F1,F2, D1,D2  at the points  XE. array has not been changed in either case.) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NEXT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (2). (output) integer array indicating number of extrapolation points: number of evaluation points to left of interval. number of evaluation points to right of interval. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (output) error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if NE.LT.1 . 2  if X1.EQ.X2 . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X1`: not a workspace argument
- `X2`: not a workspace argument
- `F1`: not a workspace argument
- `F2`: not a workspace argument
- `D1`: not a workspace argument
- `D2`: not a workspace argument
- `NE`: not a workspace argument
- `XE`: not a workspace argument
- `FE`: not a workspace argument
- `NEXT`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::chfev`
- Original SLATEC routine: `CHFEV`
- Native symbol: `chfev_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [CHFEV](https://www.netlib.org/slatec/pchip/chfev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
