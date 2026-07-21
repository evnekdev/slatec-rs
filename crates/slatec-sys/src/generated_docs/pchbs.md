# Purpose

Usage: INTEGER N, INCFD, KNOTYP, NKNOTS, NDIM, KORD, IERR PARAMETER (INCFD = ...) REAL X(nmax), F(INCFD,nmax), D(INCFD,nmax), T(2*nmax+4), PCHBS computes the B-spline representation of the PCH function determined by N,X,F,D. To be compatible with the rest of PCHIP, PCHBS includes INCFD, the increment between successive values of the F- and D-arrays. The output is the B-representation for the function: NKNOTS, T, BCOEF, NDIM, KORD. Caution: Since it is assumed that the input PCH function has been computed by one of the other routines in the package PCHIP, input arguments N, X, INCFD are **not** checked for validity. Restrictions/assumptions: 1. N.GE.2 . (not checked) 2. X(i).LT.X(i+1), i=1,...,N . (not checked) 3. INCFD.GT.0 . (not checked) 4. KNOTYP.LE.2 . (error return if not)

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHBS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHBS](https://www.netlib.org/slatec/pchip/pchbs.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of data points, N. ge. 2. (not checked).

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

is the real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (not checked) nmax, the dimension of X, must be. ge.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, *).

is the real array of dependent variable values. is the value corresponding to X(I). nmax, the second dimension of F, must be. ge. N.

## `D`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, *).

is the real array of derivative values at the data points. is the value corresponding to X(I). nmax, the second dimension of D, must be. ge. N.

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the increment between successive values in F and D. This argument is provided primarily for 2-D applications. It may have the value 1 for one-dimensional applications, in which case F and D may be singly-subscripted arrays.

## `KNOTYP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is a flag to control the knot sequence. The knot sequence T is normally computed from X by putting a double knot at each X and setting the end knot pairs according to the value of KNOTYP: 0: Quadruple knots at X(1) and X(N). (default) 1: Replicate lengths of extreme subintervals: 2: Periodic placement of boundary knots:.

## `NKNOTS`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of knots. If KNOTYP. GE. 0, then NKNOTS will be set to NDIM+4. LT. 0, then NKNOTS is an input variable, and an error return will be taken if it is not equal to NDIM+4.

## `T`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

(X(2)-X(1)) ; T(M+3) = X(N) + (X(N)-X(N-1)). (X(N)-X(N-1)); T(M+3) = X(N) + (X(2)-X(1)). Here M=NDIM=2*N. If the input value of KNOTYP is negative, however, it is assumed that NKNOTS and T were set in a previous call. This option is provided for improved efficiency when used in a parametric setting. is the array of 2*N+4 knots for the B-representation.

## `BCOEF`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

CALL PCHBS (N, X, F, D, INCFD, KNOTYP, NKNOTS, T, BCOEF, NDIM, KORD, IERR) is the array of 2*N B-spline coefficients.

## `NDIM`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the dimension of the B-spline space. (Set to 2*N. ).

## `KORD`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the B-spline. (Set to 4. ).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an error flag. Normal return: 0 (no errors). "Recoverable" errors: -4 if KNOTYP. GT. 2. -5 if KNOTYP.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `T`: not a workspace argument
- `BCOEF`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchbs`
- Original SLATEC routine: `PCHBS`
- Native symbol: `pchbs_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [PCHBS](https://www.netlib.org/slatec/pchip/pchbs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
