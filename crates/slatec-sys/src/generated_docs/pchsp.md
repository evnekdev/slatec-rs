# Purpose

PCHSP: Piecewise Cubic Hermite Spline Computes the Hermite representation of the cubic spline inter- polant to the data given in X and F satisfying the boundary conditions specified by IC and VC. To facilitate two-dimensional applications, includes an increment

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHSP](https://www.netlib.org/slatec/pchip/pchsp.f).

# Arguments

## 1. `IC`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (2). (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. 1 or 2 . 1 or 2 . (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. 1 or 2 . 1 or 2 . not applicable or not stated by selected source not a workspace argument

## 2. `VC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (2). 0. IEND may take on the same values as IBEG, but applied to 0. (input) real array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . 0. IEND may take on the same values as IBEG, but applied to 0. (input) real array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 or 2, the value is given in VC(2). NOTES: 1. An error return is taken if IEND is out of range. 2. For the "natural" boundary condition, use IEND=2 and (input) number of data points.  (Error return if N.LT.2 .) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1 or 2, the value is given in VC(2). NOTES: 1. An error return is taken if IEND is out of range. 2. For the "natural" boundary condition, use IEND=2 and (input) real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, *). and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. NOTE:  This is a modified version of C. de Boor's cubic spline routine CUBSPL. (input) real array of dependent variable values to be inter- 1)*INCFD) is value corresponding to X(I). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `D`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, *). tinuous at X(2).  This is the "not a knot" condition provided by de Boor's cubic spline routine CUBSPL. < This is the default boundary condition. > IBEG = 1  if first derivative at X(1) is given in VC(1). IBEG = 2  if second derivative at X(1) is given in VC(1). IBEG = 3  to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4  to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) NOTES: 1. An error return is taken if IBEG is out of range. 2. For the "natural" boundary condition, use IBEG=2 and (output) real array of derivative values at the data points. These values will determine the cubic spline interpolant with the requested boundary conditions. The value corresponding to X(I) is stored in 1)*INCFD),  I=1(1)N. No other entries in D are changed. array has not been changed in any of these cases.) array may have been changed in this case.) (             Do **NOT** use it!                ) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  IC(2), N, NWK, IERR REAL  VC(2), X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL  PCHSP (IC, VC, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `WK`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (2, *). (scratch) real array of working storage. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `NWK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) length of work array. (input) length of work array. not applicable or not stated by selected source

## 10. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (output) error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if IBEG.LT.0 or IBEG.GT.4 . 5  if IEND.LT.0 of IEND.GT.4 . 6  if both of the above are true. 7  if NWK is too small. NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. 8  in case of trouble solving the linear system for the interior derivative values. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `IC`: not a workspace argument
- `VC`: not a workspace argument
- `N`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `INCFD`: not a workspace argument
- `WK`: not a workspace argument
- `NWK`: (input) length of work array.
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchsp`
- Original SLATEC routine: `PCHSP`
- Native symbol: `pchsp_`
- ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32)`
- Exact Netlib source file: [PCHSP](https://www.netlib.org/slatec/pchip/pchsp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
