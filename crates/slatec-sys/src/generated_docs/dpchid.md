# Purpose

DPCHID: Piecewise Cubic Hermite Integrator, Data limits Evaluates the definite integral of the cubic Hermite function defined by N, X, F, D over the interval \[X(IA), X(IB)\]. To provide compatibility with DPCHIM and DPCHIC, includes an

# Description

This canonical unsafe binding exposes original SLATEC routine `DPCHID`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPCHID](https://www.netlib.org/slatec/pchip/dpchid.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) number of data points.  (Error return if N.LT.2 .) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). (input) real*8 array of independent variable values.  The 1) .LT. X(I),  I = 2(1)N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `F`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). and D-arrays. (input) real*8 array of function values.  F(1+(I-1)*INCFD) is the value corresponding to X(I). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `D`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (INCFD, *). (input) real*8 array of derivative values.  D(1+(I-1)*INCFD) is the value corresponding to X(I). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCFD`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ...) INTEGER  N, IA, IB, IERR DOUBLE PRECISION  X(N), F(INCFD,N), D(INCFD,N) LOGICAL  SKIP VALUE = DPCHID (N, X, F, D, INCFD, SKIP, IA, IB, IERR) Parameters: VALUE -- (output) value of the requested integral. (input) increment between successive values in F and D. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SKIP`

input-output `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. (input/output) logical variable which should be set to .TRUE. if the user wishes to skip checks for validity of preceding parameters, or to .FALSE. otherwise. This will save time in case these checks have already been performed (say, in DPCHIM or DPCHIC). 4. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `IA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) indices in X-array for the limits of integration. both must be in the range \[1,N\].  (Error return if not.) No restrictions on their relative values. (input) indices in X-array for the limits of integration. both must be in the range \[1,N\].  (Error return if not.) No restrictions on their relative values. not applicable or not stated by selected source not a workspace argument

## 8. `IB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (input) indices in X-array for the limits of integration. both must be in the range \[1,N\].  (Error return if not.) No restrictions on their relative values. (input) indices in X-array for the limits of integration. both must be in the range \[1,N\].  (Error return if not.) No restrictions on their relative values. not applicable or not stated by selected source not a workspace argument

## 9. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 4. (output) error flag. Normal return: 0  (no errors). "Recoverable" errors: 1  if N.LT.2 . 2  if INCFD.LT.1 . 3  if the X-array is not strictly increasing. 4  if IA or IB is out of range. (VALUE will be zero in any of these cases.) NOTE:  The above errors are checked in the order listed, and following arguments have **NOT** been validated. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_i32,mut_i32)`. It has no separate Rust `Result` status channel.

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
- `SKIP`: not a workspace argument
- `IA`: not a workspace argument
- `IB`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dpchid`
- Original SLATEC routine: `DPCHID`
- Native symbol: `dpchid_`
- ABI fingerprint: `function:f64(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_i32,mut_fortran_logical_i32,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [DPCHID](https://www.netlib.org/slatec/pchip/dpchid.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
