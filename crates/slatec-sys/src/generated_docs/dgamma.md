# Purpose

DGAMMA(X) calculates the double precision complete Gamma function for double precision argument X. Series for GAM on the interval 0. to 1.00000E+00 with weighted error 5.79E-32 log weighted error 31.24 significant figures required 30.00 decimal places required 32.05

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMMA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMMA](https://www.netlib.org/slatec/fnlib/dgamma.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgamma`
- Original SLATEC routine: `DGAMMA`
- Native symbol: `dgamma_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMMA](https://www.netlib.org/slatec/fnlib/dgamma.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
