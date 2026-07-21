# Purpose

DGAMR(X) calculates the double precision reciprocal of the complete Gamma function for double precision argument X.

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMR](https://www.netlib.org/slatec/fnlib/dgamr.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the reciprocal of the Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgamr`
- Original SLATEC routine: `DGAMR`
- Native symbol: `dgamr_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMR](https://www.netlib.org/slatec/fnlib/dgamr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
