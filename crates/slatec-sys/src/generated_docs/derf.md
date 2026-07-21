# Purpose

DERF(X) calculates the double precision error function for double precision argument X. Series for ERF on the interval 0. to 1.00000E+00 with weighted error 1.28E-32 log weighted error 31.89 significant figures required 31.05 decimal places required 32.55

# Description

This canonical unsafe binding exposes original SLATEC routine `DERF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DERF](https://www.netlib.org/slatec/fnlib/derf.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the error function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::error::derf`
- Original SLATEC routine: `DERF`
- Native symbol: `derf_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DERF](https://www.netlib.org/slatec/fnlib/derf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
