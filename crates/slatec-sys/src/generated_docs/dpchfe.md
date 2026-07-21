# Purpose

DPCHFE: Piecewise Cubic Hermite Function Evaluator Evaluates the cubic Hermite function defined by N, X, F, D at the points XE(J), J=1(1)NE. To provide compatibility with DPCHIM and DPCHIC, includes an increment between successive values of the F- and D-arrays. Calling sequence: PARAMETER (INCFD = ...) INTEGER N, NE, IERR DOUBLE PRECISION X(N), F(INCFD,N), D(INCFD,N), XE(NE), FE(NE) LOGICAL SKIP CALL DPCHFE (N, X, F, D, INCFD, SKIP, NE, XE, FE, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHFE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHFE](https://www.netlib.org/slatec/pchip/dpchfe.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of data points. (Error return if N. LT. 2. ).

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(input) real*8 array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ).

## `F`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real*8 array of function values. F(1+(I-1)*INCFD) is the value corresponding to X(I).

## `D`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real*8 array of derivative values. D(1+(I-1)*INCFD) is the value corresponding to X(I).

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) increment between successive values in F and D. (Error return if INCFD. LT. 1. ).

## `SKIP`

**Direction:** `input-output`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

(input/output) logical variable which should be set to. TRUE. if the user wishes to skip checks for validity of preceding parameters, or to. FALSE. otherwise. This will save time in case these checks have already been performed (say, in DPCHIM or DPCHIC).

## `NE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of evaluation points. (Error return if NE. LT. 1. ).

## `XE`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(input) real*8 array of points at which the function is to be evaluated. NOTES: 1. The evaluation will be most efficient if the elements of XE are increasing relative to X; that is, XE(J). GE. X(I) implies XE(K). X(I), all K.

## `FE`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(output) real*8 array of values of the cubic Hermite function defined by N, X, F, D at the points XE.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). Warning error: IERR. GT. 0 means that extrapolation was performed at IERR points. "Recoverable" errors: -1 if N.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument
- `XE`: not a workspace argument
- `FE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchfe`
- Original SLATEC routine: `DPCHFE`
- Native symbol: `dpchfe_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPCHFE](https://www.netlib.org/slatec/pchip/dpchfe.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
