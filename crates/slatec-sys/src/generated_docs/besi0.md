# Purpose

BESI0(X) computes the modified (hyperbolic) Bessel function of the first kind of order zero and real argument X. Series for BI0 on the interval 0. to 9.00000D+00 with weighted error 2.46E-18 log weighted error 17.61 significant figures required 17.90 decimal places required 18.15

# Description

This canonical unsafe binding exposes original SLATEC routine `BESI0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESI0](https://www.netlib.org/slatec/fnlib/besi0.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the hyperbolic Bessel function of the first kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besi0`
- Original SLATEC routine: `BESI0`
- Native symbol: `besi0_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESI0](https://www.netlib.org/slatec/fnlib/besi0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
