# Purpose

DPCHIC: Piecewise Cubic Hermite Interpolation Coefficients. Sets derivatives needed to determine a piecewise monotone piece- wise cubic interpolant to the data given in X and F satisfying the boundary conditions specified by IC and VC. The treatment of points where monotonicity switches direction is controlled by argument SWITCH. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR DOUBLE PRECISION VC(2), SWITCH, X(N), F(INCFD,N), D(INCFD,N),

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHIC](https://www.netlib.org/slatec/pchip/dpchic.f).

# Arguments

## `IC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (2).

(input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0 for the default boundary condition (the same as used by DPCHIM). If IBEG. NE. 0, then its sign indicates whether the boundary derivative is to be adjusted, if necessary, to be compatible with monotonicity: IBEG.

## `VC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

(input) real*8 array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2. VC(2) need be set only if IC(2) = 1 or 2.

## `SWITCH`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

(input) indicates desired treatment of points where direction of monotonicity switches: Set SWITCH to zero if interpolant is required to be mono- tonic in each interval, regardless of monotonicity of data. NOTES: 1. This will cause D to be set to zero at all switch points, thus forcing extrema there. 2. The result of using this option with the default boun- dary conditions will be identical to using DPCHIM, but will generally cost more compute time. This option is provided only to facilitate comparison of different switch and/or boundary conditions.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of data points. (Error return if N. LT. 2. ).

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(input) real*8 array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ).

## `F`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real*8 array of dependent variable values to be interpolated. F(1+(I-1)*INCFD) is value corresponding to.

## `D`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(output) real*8 array of derivative values at the data points. These values will determine a monotone cubic Hermite function on each subinterval on which the data are monotonic, except possibly adjacent to switches in monotonicity. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed.

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ).

## `WK`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NWK).

CALL DPCHIC (IC, VC, SWITCH, N, X, F, D, INCFD, WK, NWK, IERR) (scratch) real*8 array of working storage. The user may wish to know that the returned values are: H(I) = X(I+1) - X(I) ; SLOPE(I) = (F(1,I+1) - F(1,I)) / H(I) for I = 1(1)N-1.

## `NWK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) length of work array. (Error return if NWK. LT. 2*(N-1). ).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). Warning errors: 1 if IBEG. LT. 0 and D(1) had to be adjusted for monotonicity. 2 if IEND.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `IC`: not a workspace argument
- `VC`: not a workspace argument
- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `WK`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchic`
- Original SLATEC routine: `DPCHIC`
- Native symbol: `dpchic_`
- ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHIC](https://www.netlib.org/slatec/pchip/dpchic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
