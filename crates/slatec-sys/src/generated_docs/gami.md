# Purpose

Evaluate the incomplete gamma function defined by GAMI = integral from T = 0 to X of EXP(-T) * T**(A-1.0) . GAMI is evaluated for positive values of A and non-negative values of X. A slight deterioration of 2 or 3 digits accuracy will occur when GAMI is very large or very small, because logarithmic variables are used. GAMI, A, and X are single precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMI](https://www.netlib.org/slatec/fnlib/gami.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate the incomplete Gamma function

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate the incomplete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::gami`
- Original SLATEC routine: `GAMI`
- Native symbol: `gami_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [GAMI](https://www.netlib.org/slatec/fnlib/gami.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
