# Purpose

DBESKS computes a sequence of modified Bessel functions of the third

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESKS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESKS](https://www.netlib.org/slatec/fnlib/dbesks.f).

# Arguments

## 1. `XNU`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. 1,1), 1,1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Input evaluation point. `X` must be strictly positive; native code does not modify it. Input evaluation point. `X` must be strictly positive; native code does not modify it. not applicable or not stated by selected source not a workspace argument

## 3. `NIN`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, if NIN is positive and I = 0, 1, ... , is negative.  On return, the vector BK(.) contains is negative.  On return, the vector BK(.) contains the results at X for order starting at XNU.  XNU, X, and BK are the results at X for order starting at XNU.  XNU, X, and BK are double precision.  NIN is an integer. double precision.  NIN is an integer. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `BK`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Writable output array with at least `abs(NIN)` elements. On return `BK\[I\]` holds the modified Bessel K value at order `XNU+I`; native code retains no pointer. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `BK`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesks`
- Original SLATEC routine: `DBESKS`
- Native symbol: `dbesks_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBESKS](https://www.netlib.org/slatec/fnlib/dbesks.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
