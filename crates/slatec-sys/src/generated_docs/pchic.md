# Purpose

PCHIC: Piecewise Cubic Hermite Interpolation Coefficients. Sets derivatives needed to determine a piecewise monotone piece- wise cubic interpolant to the data given in X and F satisfying the boundary conditions specified by IC and VC. The treatment of points where monotonicity switches direction is controlled by argument SWITCH. To facilitate two-dimensional applications, includes an increment

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHIC](https://www.netlib.org/slatec/pchip/pchic.f).

# Arguments

## 1. `IC`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (2). (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0  for the default boundary condition (the same as used by PCHIM). If IBEG.NE.0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG.GT.0  if no adjustment is to be performed. IBEG.LT.0  if the derivative is to be adjusted for monotonicity. Allowable values for the magnitude of IBEG are: IBEG = 1  if first derivative at X(1) is given in VC(1). IBEG = 2  if second derivative at X(1) is given in VC(1). IBEG = 3  to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4  to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) 1 or 2 . 1 or 2 . (input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0  for the default boundary condition (the same as used by PCHIM). If IBEG.NE.0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG.GT.0  if no adjustment is to be performed. IBEG.LT.0  if the derivative is to be adjusted for monotonicity. Allowable values for the magnitude of IBEG are: IBEG = 1  if first derivative at X(1) is given in VC(1). IBEG = 2  if second derivative at X(1) is given in VC(1). IBEG = 3  to use the 3-point difference formula for D(1). (Reverts to the default b.c. if N.LT.3 .) IBEG = 4  to use the 4-point difference formula for D(1). (Reverts to the default b.c. if N.LT.4 .) 1 or 2 . 1 or 2 . not applicable or not stated by selected source not a workspace argument

## 2. `VC`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (2). (input) real array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . (input) real array of length 2 specifying desired boundary values, as indicated above. 1 or 2 . 1 or 2 . not applicable or not stated by selected source not a workspace argument

## 3. `SWITCH`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (input) indicates desired treatment of points where direction of monotonicity switches: tonic in each interval, regardless of monotonicity of data. NOTES: 1. This will cause D to be set to zero at all switch points, thus forcing extrema there. 2. The result of using this option with the default boun- dary conditions will be identical to using PCHIM, but will generally cost more compute time. This option is provided only to facilitate comparison of different switch and/or boundary conditions. point difference formula in the vicinity of switch points. If SWITCH is positive, the interpolant on each interval containing an extremum is controlled to not deviate from the data by more than SWITCH*DFLOC, where DFLOC is the maximum of the change of F on this interval and its two immediate neighbors. If SWITCH is negative, no such control is to be imposed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 or 2, the value is given in VC(2). NOTES (IEND): 1. An error return is taken if ABS(IEND).GT.5 . 2. Only in case  IEND.LE.0  is it guaranteed that the interpolant will be monotonic in the last interval. 1)*INCFD) lies between 1), the interpolant will be monotonic. This is **NOT** checked if IEND.GT.0 . 1)*INCFD) had to be changed to achieve monotonicity, a warning error is returned. (input) number of data points.  (Error return if N.LT.2 .) 1+I) = SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) 1. 1)*INCFD) had to be adjusted for monotonicity. ting local monotone piecewise cubic interpolants, SIAM Journal on Scientific and Statistical Computing 5, 2 (June 1984), pp. 300-304. 3. F. N. Fritsch and R. E. Carlson, Monotone piecewise cubic interpolation, SIAM Journal on Numerical Ana- lysis 17, 2 (April 1980), pp. 238-246. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1 or 2, the value is given in VC(2). NOTES (IEND): 1. An error return is taken if ABS(IEND).GT.5 . 2. Only in case  IEND.LE.0  is it guaranteed that the interpolant will be monotonic in the last interval. (input) real array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. X(I) ; not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `F`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, *). and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by PCHFE or PCHFD. (input) real array of dependent variable values to be inter- 1)*INCFD) is value corresponding to X(I). ting local monotone piecewise cubic interpolants, SIAM Journal on Scientific and Statistical Computing 5, 2 (June 1984), pp. 300-304. 3. F. N. Fritsch and R. E. Carlson, Monotone piecewise cubic interpolation, SIAM Journal on Numerical Ana- lysis 17, 2 (April 1980), pp. 238-246. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `D`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (INCFD, *). tinuous at X(2). (Reverts to the default b.c. if N.LT.4.) This option is somewhat analogous to the "not a knot" boundary condition provided by PCHSP. NOTES (IBEG): 1. An error return is taken if ABS(IBEG).GT.5 . 2. Only in case  IBEG.LE.0  is it guaranteed that the interpolant will be monotonic in the first interval. If the returned value of D(1) lies between zero and 3*SLOPE(1), the interpolant will be monotonic.  This is **NOT** checked if IBEG.GT.0 . tonicity, a warning error is returned. IEND may take on the same values as IBEG, but applied to 1)*INCFD) lies between 1)*INCFD) had to be changed to achieve monotonicity, a warning error is returned. (output) real array of derivative values at the data points. These values will determine a monotone cubic Hermite func- tion on each subinterval on which the data are monotonic, except possibly adjacent to switches in monotonicity. The value corresponding to X(I) is stored in 1)*INCFD),  I=1(1)N. No other entries in D are changed. 1)*INCFD) had to be adjusted for monotonicity. array has not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  IC(2), N, NWK, IERR REAL  VC(2), SWITCH, X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL  PCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR) Parameters: (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `WK`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NWK). (scratch) real array of working storage.  The user may wish to know that the returned values are: X(I) ; 1+I) = SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NWK`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) length of work array. (input) length of work array. not applicable or not stated by selected source

## 11. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (output) error flag. Normal return: 0  (no errors). Warning errors: 1  if IBEG.LT.0 and D(1) had to be adjusted for monotonicity. 1)*INCFD) had to be adjusted for monotonicity. 3  if both of the above are true. "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if ABS(IBEG).GT.5 . 5  if ABS(IEND).GT.5 . 6  if both of the above are true. 7  if NWK.LT.2*(N-1) . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `IC`: not a workspace argument
- `VC`: not a workspace argument
- `SWITCH`: not a workspace argument
- `N`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `INCFD`: not a workspace argument
- `WK`: not a workspace argument
- `NWK`: (input) length of work array.
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchic`
- Original SLATEC routine: `PCHIC`
- Native symbol: `pchic_`
- ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [PCHIC](https://www.netlib.org/slatec/pchip/pchic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
