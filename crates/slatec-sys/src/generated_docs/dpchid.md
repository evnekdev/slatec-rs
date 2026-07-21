# Purpose

DPCHID: Piecewise Cubic Hermite Integrator, Data limits Evaluates the definite integral of the cubic Hermite function defined by N, X, F, D over the interval \[X(IA), X(IB)\]. To provide compatibility with DPCHIM and DPCHIC, includes an increment between successive values of the F- and D-arrays. Calling sequence: PARAMETER (INCFD = ...) INTEGER N, IA, IB, IERR DOUBLE PRECISION X(N), F(INCFD,N), D(INCFD,N) LOGICAL SKIP VALUE = DPCHID (N, X, F, D, INCFD, SKIP, IA, IB, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHID`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHID](https://www.netlib.org/slatec/pchip/dpchid.f).

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

**Direction:** `output`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

(input/output) logical variable which should be set to. TRUE. if the user wishes to skip checks for validity of preceding parameters, or to. FALSE. otherwise. This will save time in case these checks have already been performed (say, in DPCHIM or DPCHIC).

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) indices in X-array for the limits of integration. both must be in the range \[1,N\]. (Error return if not. ) No restrictions on their relative values.

## `IB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) indices in X-array for the limits of integration. both must be in the range \[1,N\]. (Error return if not. ) No restrictions on their relative values.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if N. LT. 2. -2 if INCFD.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_i32,mut_i32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchid`
- Original SLATEC routine: `DPCHID`
- Native symbol: `dpchid_`
- ABI fingerprint: `function:f64(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHID](https://www.netlib.org/slatec/pchip/dpchid.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
