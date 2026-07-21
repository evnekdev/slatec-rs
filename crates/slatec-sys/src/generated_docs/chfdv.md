# Purpose

CHFDV: Cubic Hermite Function and Derivative Evaluator Evaluates the cubic polynomial determined by function values F1,F2 and derivatives D1,D2 on interval (X1,X2), together with its first derivative, at the points XE(J), J=1(1)NE. If only function values are required, use CHFEV, instead. Calling sequence: INTEGER NE, NEXT(2), IERR REAL X1, X2, F1, F2, D1, D2, XE(NE), FE(NE), DE(NE) CALL CHFDV (X1,X2, F1,F2, D1,D2, NE, XE, FE, DE, NEXT, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `CHFDV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHFDV](https://www.netlib.org/slatec/pchip/chfdv.f).

# Arguments

## `X1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) endpoints of interval of definition of cubic. (Error return if X1. EQ. X2. ).

## `X2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) endpoints of interval of definition of cubic. (Error return if X1. EQ. X2. ).

## `F1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) values of function at X1 and X2, respectively.

## `F2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) values of function at X1 and X2, respectively.

## `D1`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) values of derivative at X1 and X2, respectively.

## `D2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) values of derivative at X1 and X2, respectively.

## `NE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of evaluation points. (Error return if NE. LT. 1. ).

## `XE`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

(input) real array of points at which the functions are to be evaluated. If any of the XE are outside the interval \[X1,X2\], a warning error is returned in NEXT.

## `FE`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

(output) real array of values of the cubic function defined by X1,X2, F1,F2, D1,D2 at the points XE.

## `DE`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

(output) real array of values of the first derivative of the same function at the points XE.

## `NEXT`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (2).

(output) integer array indicating number of extrapolation points: number of evaluation points to left of interval. number of evaluation points to right of interval.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if NE. LT. 1. -2 if X1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `XE`: not a workspace argument
- `FE`: not a workspace argument
- `DE`: not a workspace argument
- `NEXT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::chfdv`
- Original SLATEC routine: `CHFDV`
- Native symbol: `chfdv_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [CHFDV](https://www.netlib.org/slatec/pchip/chfdv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
