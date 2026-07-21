# Purpose

Evaluate Tricomi's incomplete gamma function defined by

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMIT](https://www.netlib.org/slatec/fnlib/gamit.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. 1.) for A .GT. 0.0 and by analytic continuation for A .LE. 0.0. negative values of X (even though GAMIT is defined for X .LT. slight deterioration of 2 or 3 digits accuracy will occur when GAMIT is very large or very small in absolute value, because log- arithmic variables are used.  Also, if the parameter  A  is very close to a negative integer (but not a negative integer), there is loss of accuracy, which is reported if the result is less than half machine precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. A)/GAMMA(A) * integral from 0 to X of EXP(-T) * is the complete gamma function of X. 0 and A .LE. 0.0, GAMIT is infinite, which is a fatal error. The function and both arguments are REAL. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::gamit`
- Original SLATEC routine: `GAMIT`
- Native symbol: `gamit_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [GAMIT](https://www.netlib.org/slatec/fnlib/gamit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
