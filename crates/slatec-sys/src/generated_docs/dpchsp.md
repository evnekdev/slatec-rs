# Purpose

DPCHSP: Piecewise Cubic Hermite Spline Computes the Hermite representation of the cubic spline inter- polant to the data given in X and F satisfying the boundary conditions specified by IC and VC. To facilitate two-dimensional applications, includes an increment between successive values of the F- and D-arrays. The resulting piecewise cubic Hermite function may be evaluated by DPCHFE or DPCHFD. NOTE: This is a modified version of C. de Boor's cubic spline routine CUBSPL. Calling sequence: PARAMETER (INCFD = ...) INTEGER IC(2), N, NWK, IERR DOUBLE PRECISION VC(2), X(N), F(INCFD,N), D(INCFD,N), WK(NWK) CALL DPCHSP (IC, VC, N, X, F, D, INCFD, WK, NWK, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHSP](https://www.netlib.org/slatec/pchip/dpchsp.f).

# Arguments

## `IC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (2).

(input) integer array of length 2 specifying desired boundary conditions: IBEG, desired condition at beginning of data. IEND, desired condition at end of data. IBEG = 0 to set D(1) so that the third derivative is con- tinuous at X(2). This is the "not a knot" condition provided by de Boor's cubic spline routine CUBSPL. < This is the default boundary condition. > IBEG = 1 if first derivative at X(1) is given in VC(1).

## `VC`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (2).

(input) real*8 array of length 2 specifying desired boundary values, as indicated above. VC(1) need be set only if IC(1) = 1 or 2. VC(2) need be set only if IC(2) = 1 or 2.

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

(output) real*8 array of derivative values at the data points. These values will determine the cubic spline interpolant with the requested boundary conditions. The value corresponding to X(I) is stored in I=1(1)N. No other entries in D are changed.

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) increment between successive values in F and D. This argument is provided primarily for 2-D applications. (Error return if INCFD. LT. 1. ).

## `WK`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (2, *).

(scratch) real*8 array of working storage.

## `NWK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) length of work array. (Error return if NWK. LT. 2*N. ).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if N. LT. 2. -2 if INCFD.

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

- Canonical Rust path: `slatec_sys::interpolation::dpchsp`
- Original SLATEC routine: `DPCHSP`
- Native symbol: `dpchsp_`
- ABI fingerprint: `subroutine:void(mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHSP](https://www.netlib.org/slatec/pchip/dpchsp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
