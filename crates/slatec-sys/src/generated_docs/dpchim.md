# Purpose

DPCHIM: Piecewise Cubic Hermite Interpolation to Monotone data. Sets derivatives needed to determine a monotone piecewise cubic Hermite interpolant to the data given in X and F. Default boundary conditions are provided which are compatible with monotonicity. (See DPCHIC if user control of boundary con- ditions is desired.) If the data are only piecewise monotonic, the interpolant will have an extremum at each point where monotonicity switches direc- tion. (See DPCHIC if user control is desired in such cases.) To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. Calling sequence: PARAMETER (INCFD = ...) INTEGER N, IERR DOUBLE PRECISION X(N), F(INCFD,N), D(INCFD,N) CALL DPCHIM (N, X, F, D, INCFD, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHIM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHIM](https://www.netlib.org/slatec/pchip/dpchim.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of data points. (Error return if N. LT. 2. ) If N=2, simply does linear interpolation.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(input) real*8 array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ) DPCHIM is designed for monotonic data, but it will work for any F-array.

## `F`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real*8 array of dependent variable values to be interpolated. F(1+(I-1)*INCFD) is value corresponding to.

## `D`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(output) real*8 array of derivative values at the data points. If the data are monotonic, these values will determine a monotone cubic Hermite function. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed.

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). Warning error: IERR. GT. 0 means that IERR switches in the direction of monotonicity were detected. "Recoverable" errors: -1 if N.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchim`
- Original SLATEC routine: `DPCHIM`
- Native symbol: `dpchim_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHIM](https://www.netlib.org/slatec/pchip/dpchim.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
