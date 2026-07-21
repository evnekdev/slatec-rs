# Purpose

ATANH(X) computes the arc hyperbolic tangent of X. Series for ATNH on the interval 0. to 2.50000D-01 with weighted error 6.70E-18 log weighted error 17.17 significant figures required 16.01 decimal places required 17.76

# Description

This canonical unsafe binding exposes original SLATEC routine `ATANH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ATANH](https://www.netlib.org/slatec/fnlib/atanh.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the arc hyperbolic tangent

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::atanh`
- Original SLATEC routine: `ATANH`
- Native symbol: `atanh_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [ATANH](https://www.netlib.org/slatec/fnlib/atanh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
