# Purpose

PCHIA: Piecewise Cubic Hermite Integrator, Arbitrary limits Evaluates the definite integral of the cubic Hermite function defined by N, X, F, D over the interval \[A, B\]. To provide compatibility with PCHIM and PCHIC, includes an increment between successive values of the F- and D-arrays. Calling sequence: PARAMETER (INCFD = ...) INTEGER N, IERR REAL X(N), F(INCFD,N), D(INCFD,N), A, B REAL VALUE, PCHIA LOGICAL SKIP VALUE = PCHIA (N, X, F, D, INCFD, SKIP, A, B, IERR)

# Description

This canonical unsafe binding exposes original SLATEC routine `PCHIA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCHIA](https://www.netlib.org/slatec/pchip/pchia.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) number of data points. (Error return if N. LT. 2. ).

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

(input) real array of independent variable values. The elements of X must be strictly increasing:. LT. X(I), I = 2(1)N. (Error return if not. ).

## `F`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real array of function values. F(1+(I-1)*INCFD) is the value corresponding to X(I).

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (INCFD, *).

(input) real array of derivative values. D(1+(I-1)*INCFD) is the value corresponding to X(I).

## `INCFD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(input) increment between successive values in F and D. (Error return if INCFD. LT. 1. ).

## `SKIP`

**Direction:** `output`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** scalar.

(input/output) logical variable which should be set to. TRUE. if the user wishes to skip checks for validity of preceding parameters, or to. FALSE. otherwise. This will save time in case these checks have already been performed (say, in PCHIM or PCHIC).

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) the limits of integration. NOTE: There is no requirement that \[A,B\] be contained in \[X(1),X(N)\]. However, the resulting integral value will be highly suspect, if not.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

(input) the limits of integration. NOTE: There is no requirement that \[A,B\] be contained in \[X(1),X(N)\]. However, the resulting integral value will be highly suspect, if not.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(output) error flag. Normal return: 0 (no errors). Warning errors: 1 if A is outside the interval \[X(1),X(N)\]. 2 if B is outside the interval \[X(1),X(N)\]. 3 if both of the above are true. (Note that this means that either \[A,B\] contains data interval or the intervals do not intersect at all.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_f32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `X`: not a workspace argument
- `F`: not a workspace argument
- `D`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::pchia`
- Original SLATEC routine: `PCHIA`
- Native symbol: `pchia_`
- ABI fingerprint: `function:f32(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [PCHIA](https://www.netlib.org/slatec/pchip/pchia.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
