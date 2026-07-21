# Purpose

Evaluate the incomplete gamma function defined by

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMI](https://www.netlib.org/slatec/fnlib/dgami.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative values of X.  A slight deterioration of 2 or 3 digits accuracy will occur when DGAMI is very large or very small, because logarithmic variables are used.  The function and both arguments are double precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. T) * T**(A-1.0) . not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::special::gamma::dgami`
- Original SLATEC routine: `DGAMI`
- Native symbol: `dgami_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMI](https://www.netlib.org/slatec/fnlib/dgami.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
