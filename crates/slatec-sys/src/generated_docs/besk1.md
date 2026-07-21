# Purpose

BESK1(X) computes the modified (hyperbolic) Bessel function of third kind of order one for real argument X, where X .GT. 0. Series for BK1 on the interval 0. to 4.00000D+00 with weighted error 7.02E-18 log weighted error 17.15 significant figures required 16.73 decimal places required 17.67

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK1](https://www.netlib.org/slatec/fnlib/besk1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the modified (hyperbolic) Bessel function of the third kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk1`
- Original SLATEC routine: `BESK1`
- Native symbol: `besk1_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESK1](https://www.netlib.org/slatec/fnlib/besk1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
