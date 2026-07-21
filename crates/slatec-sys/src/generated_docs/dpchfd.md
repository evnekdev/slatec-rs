# Purpose

DPCHFD: Piecewise Cubic Hermite Function and Derivative evaluator

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHFD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHFD](https://www.netlib.org/slatec/pchip/dpchfd.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) number of data points.  (Error return if N.LT.2 .) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). (input) real*8 array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `F`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). and D-arrays. (input) real*8 array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `D`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). (input) real*8 array of derivative values.  D(1+(I-1)*INCFD) is the value corresponding to X(I). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  N, NE, IERR DOUBLE PRECISION  X(N), F(INCFD,N), D(INCFD,N), XE(NE), FE(NE), (input) increment between successive values in F and D. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SKIP`

input-output `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. (input/output) logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed (say, in DPCHIM or DPCHIC). will be set to .TRUE. on normal return. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `NE`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. LOGICAL  SKIP CALL  DPCHFD (N, X, F, D, INCFD, SKIP, NE, XE, FE, DE, IERR) Parameters: (input) number of evaluation points.  (Error return if .) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `XE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). 1(1)NE. If only function values are required, use DPCHFE, instead. To provide compatibility with DPCHIM and DPCHIC, includes an (input) real*8 array of points at which the functions are to be evaluated. NOTES: 1. The evaluation will be most efficient if the elements of XE are increasing relative to X; that is,   XE(J) .GE. X(I) implies    XE(K) .GE. X(I),  all K.GE.J . 2. If any of the XE are outside the interval \[X(1),X(N)\], values are extrapolated from the nearest extreme cubic, and a warning error is returned. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `FE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). (output) real*8 array of values of the cubic Hermite function defined by  N, X, F, D  at the points  XE. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `DE`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). LOGICAL  SKIP CALL  DPCHFD (N, X, F, D, INCFD, SKIP, NE, XE, FE, DE, IERR) Parameters: (output) real*8 array of values of the first derivative of the same function at the points  XE. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (output) error flag. Normal return: 0  (no errors). Warning error: means that extrapolation was performed at points. "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if NE.LT.1 . (Output arrays have not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 5  if an error has occurred in the lower-level routine DCHFDV.  NB: this should never happen. Notify the author **IMMEDIATELY** if it does. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `INCFD`: not a workspace argument
- `SKIP`: not a workspace argument
- `NE`: not a workspace argument
- `XE`: not a workspace argument
- `FE`: not a workspace argument
- `DE`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchfd`
- Original SLATEC routine: `DPCHFD`
- Native symbol: `dpchfd_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPCHFD](https://www.netlib.org/slatec/pchip/dpchfd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
