# Purpose

GAMMA computes the gamma function at X, where X is not 0, -1, -2, .... GAMMA and X are single precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMMA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMMA](https://www.netlib.org/slatec/fnlib/gamma.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::gamma`
- Original SLATEC routine: `GAMMA`
- Native symbol: `gamma_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [GAMMA](https://www.netlib.org/slatec/fnlib/gamma.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
