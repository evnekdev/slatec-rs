# Purpose

Evaluates the logarithm of the absolute value of the gamma function.

# Description

This canonical unsafe binding exposes original SLATEC routine `ALGAMS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ALGAMS](https://www.netlib.org/slatec/fnlib/algams.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. input argument not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `ALGAM`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. result not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `SGNGAM`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is set to the sign of GAMMA(X) and will be returned at +1.0 or -1.0. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `ALGAM`: not a workspace argument
- `SGNGAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::algams`
- Original SLATEC routine: `ALGAMS`
- Native symbol: `algams_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [ALGAMS](https://www.netlib.org/slatec/fnlib/algams.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
