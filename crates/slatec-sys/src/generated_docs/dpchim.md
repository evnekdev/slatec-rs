# Purpose

DPCHIM: Piecewise Cubic Hermite Interpolation to Monotone data. Sets derivatives needed to determine a monotone piecewise cubic Hermite interpolant to the data given in X and F. Default boundary conditions are provided which are compatible with monotonicity. (See DPCHIC if user control of boundary con- ditions is desired.) If the data are only piecewise monotonic, the interpolant will have an extremum at each point where monotonicity switches direc- tion. (See DPCHIC if user control is desired in such cases.) To facilitate two-dimensional applications, includes an increment

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHIM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHIM](https://www.netlib.org/slatec/pchip/dpchim.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) number of data points.  (Error return if N.LT.2 .) 2, simply does linear interpolation. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). (input) real*8 array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. DPCHIM is designed for monotonic data, but it will not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `F`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. (input) real*8 array of dependent variable values to be 1)*INCFD) is value corresponding to array.  It will force extrema at points where monotonicity switches direction.  If some other treatment of switch points is desired, DPCHIC should be used instead. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `D`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). (output) real*8 array of derivative values at the data points.  If the data are monotonic, these values will determine a monotone cubic Hermite function. The value corresponding to X(I) is stored in 1)*INCFD),  I=1(1)N. No other entries in D are changed. array has not been changed in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  N, IERR DOUBLE PRECISION  X(N), F(INCFD,N), D(INCFD,N) CALL  DPCHIM (N, X, F, D, INCFD, IERR) Parameters: (input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (output) error flag. Normal return: 0  (no errors). Warning error: means that IERR switches in the direction of monotonicity were detected. "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchim`
- Original SLATEC routine: `DPCHIM`
- Native symbol: `dpchim_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHIM](https://www.netlib.org/slatec/pchip/dpchim.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
