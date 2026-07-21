# Purpose

Evaluate the incomplete gamma function defined by DGAMI = integral from T = 0 to X of EXP(-T) * T**(A-1.0) . DGAMI is evaluated for positive values of A and non-negative values of X. A slight deterioration of 2 or 3 digits accuracy will occur when DGAMI is very large or very small, because logarithmic variables are used. The function and both arguments are double precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMI](https://www.netlib.org/slatec/fnlib/dgami.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate the incomplete Gamma function

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate the incomplete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgami`
- Original SLATEC routine: `DGAMI`
- Native symbol: `dgami_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMI](https://www.netlib.org/slatec/fnlib/dgami.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
