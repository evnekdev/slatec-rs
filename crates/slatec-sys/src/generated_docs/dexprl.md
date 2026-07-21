# Purpose

Evaluate EXPREL(X) = (EXP(X) - 1.0) / X. For small ABS(X) the Taylor series is used. If X is negative the reflection formula EXPREL(X) = EXP(X) * EXPREL(ABS(X)) may be used. This reflection formula will be of use when the evaluation for small ABS(X) is done by Chebyshev series rather than Taylor series.

# Description

This canonical unsafe binding exposes original SLATEC routine `DEXPRL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DEXPRL](https://www.netlib.org/slatec/fnlib/dexprl.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the relative error exponential (EXP(X)-1)/X

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::dexprl`
- Original SLATEC routine: `DEXPRL`
- Native symbol: `dexprl_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DEXPRL](https://www.netlib.org/slatec/fnlib/dexprl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
