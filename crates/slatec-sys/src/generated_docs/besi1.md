# Purpose

BESI1(X) calculates the modified (hyperbolic) Bessel function of the first kind of order one for real argument X. Series for BI1 on the interval 0. to 9.00000D+00 with weighted error 2.40E-17 log weighted error 16.62 significant figures required 16.23 decimal places required 17.14

# Description

This canonical unsafe binding exposes original SLATEC routine `BESI1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESI1](https://www.netlib.org/slatec/fnlib/besi1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the modified (hyperbolic) Bessel function of the first kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besi1`
- Original SLATEC routine: `BESI1`
- Native symbol: `besi1_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESI1](https://www.netlib.org/slatec/fnlib/besi1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
