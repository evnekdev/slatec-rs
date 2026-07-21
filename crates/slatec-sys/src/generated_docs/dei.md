# Purpose

DEI calculates the double precision exponential integral, Ei(X), for positive double precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X,

# Description

This canonical unsafe binding exposes original SLATEC routine `DEI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEI](https://www.netlib.org/slatec/fnlib/dei.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. E1(-X) or Ei(-X). not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dei`
- Original SLATEC routine: `DEI`
- Native symbol: `dei_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DEI](https://www.netlib.org/slatec/fnlib/dei.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
