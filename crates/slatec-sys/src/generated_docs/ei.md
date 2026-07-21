# Purpose

EI calculates the single precision exponential integral, Ei(X), for positive single precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X, Ei(X) = -E1(-X) E1(X) = -Ei(-X).

# Description

This canonical unsafe binding exposes original SLATEC routine `EI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EI](https://www.netlib.org/slatec/fnlib/ei.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponential integral Ei(X)

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::ei`
- Original SLATEC routine: `EI`
- Native symbol: `ei_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [EI](https://www.netlib.org/slatec/fnlib/ei.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
