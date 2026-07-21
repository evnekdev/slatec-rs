# Purpose

ACOSH(X) computes the arc hyperbolic cosine of X.

# Description

This canonical unsafe binding exposes original SLATEC routine `ACOSH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ACOSH](https://www.netlib.org/slatec/fnlib/acosh.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the arc hyperbolic cosine

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::acosh`
- Original SLATEC routine: `ACOSH`
- Native symbol: `acosh_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [ACOSH](https://www.netlib.org/slatec/fnlib/acosh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
