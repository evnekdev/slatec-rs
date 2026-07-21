# Purpose

DACOSH(X) calculates the double precision arc hyperbolic cosine for double precision argument X. The result is returned on the positive branch.

# Description

This canonical unsafe binding exposes original SLATEC routine `DACOSH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DACOSH](https://www.netlib.org/slatec/fnlib/dacosh.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the arc hyperbolic cosine

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dacosh`
- Original SLATEC routine: `DACOSH`
- Native symbol: `dacosh_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DACOSH](https://www.netlib.org/slatec/fnlib/dacosh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
