# Purpose

Calculate the minimum and maximum legal bounds for X in GAMMA(X).

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMLIM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMLIM](https://www.netlib.org/slatec/fnlib/gamlim.f).

# Arguments

## 1. `XMIN`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. trivial ones to calculate. Output Arguments -- minimum legal value of X in GAMMA(X).  Any smaller value of X might result in underflow. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `XMAX`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. trivial ones to calculate. Output Arguments -- maximum legal value of X in GAMMA(X).  Any larger value will cause overflow. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `XMIN`: not a workspace argument
- `XMAX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamlim`
- Original SLATEC routine: `GAMLIM`
- Native symbol: `gamlim_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32)`
- Exact Netlib source file: [GAMLIM](https://www.netlib.org/slatec/fnlib/gamlim.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
