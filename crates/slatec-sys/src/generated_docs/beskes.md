# Purpose

BESKES computes a sequence of exponentially scaled (i.e., multipled by EXP(X)) modified Bessel functions of the third kind of order XNU + I at X, where X .GT. 0,

# Description

This canonical unsafe binding exposes original SLATEC routine `BESKES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESKES](https://www.netlib.org/slatec/fnlib/beskes.f).

# Arguments

## 1. `XNU`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1,1), and I = 0, 1, ... , NIN - 1, if NIN is positive not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Input evaluation point. `X` must be strictly positive; the returned sequence is evaluated at this point and native code does not modify it. Input evaluation point. `X` must be strictly positive; the returned sequence is evaluated at this point and native code does not modify it. not applicable or not stated by selected source not a workspace argument

## 3. `NIN`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is negative.  On return, the is negative.  On return, the vector BKE(.) contains the results at X for order starting at XNU. vector BKE(.) contains the results at X for order starting at XNU. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `BKE`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Writable output array with at least `abs(NIN)` elements. On return `BKE\[I\]` holds `exp(X) * K_(XNU+I)(X)` for the requested order sequence; native code retains no pointer. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XNU`: not a workspace argument
- `X`: not a workspace argument
- `NIN`: not a workspace argument
- `BKE`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::beskes`
- Original SLATEC routine: `BESKES`
- Native symbol: `beskes_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BESKES](https://www.netlib.org/slatec/fnlib/beskes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
