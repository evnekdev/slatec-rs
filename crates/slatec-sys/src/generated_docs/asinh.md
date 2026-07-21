# Purpose

ASINH(X) computes the arc hyperbolic sine of X. Series for ASNH on the interval 0. to 1.00000D+00 with weighted error 2.19E-17 log weighted error 16.66 significant figures required 15.60 decimal places required 17.31

# Description

This canonical unsafe binding exposes original SLATEC routine `ASINH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ASINH](https://www.netlib.org/slatec/fnlib/asinh.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the arc hyperbolic sine

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::asinh`
- Original SLATEC routine: `ASINH`
- Native symbol: `asinh_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [ASINH](https://www.netlib.org/slatec/fnlib/asinh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
