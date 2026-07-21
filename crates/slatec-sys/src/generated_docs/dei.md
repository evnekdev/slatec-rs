# Purpose

DEI calculates the double precision exponential integral, Ei(X), for positive double precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X, Ei(X) = -E1(-X) E1(X) = -Ei(-X).

# Description

This canonical unsafe binding exposes original SLATEC routine `DEI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEI](https://www.netlib.org/slatec/fnlib/dei.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponential integral Ei(X)

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dei`
- Original SLATEC routine: `DEI`
- Native symbol: `dei_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DEI](https://www.netlib.org/slatec/fnlib/dei.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
