# Purpose

COT(X) calculates the cotangent of the real argument X. X is in units of radians. Series for COT on the interval 0. to 6.25000D-02 with weighted error 3.76E-17 log weighted error 16.42 significant figures required 15.51 decimal places required 16.88

# Description

This canonical unsafe binding exposes original SLATEC routine `COT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COT](https://www.netlib.org/slatec/fnlib/cot.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the cotangent

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::cot`
- Original SLATEC routine: `COT`
- Native symbol: `cot_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [COT](https://www.netlib.org/slatec/fnlib/cot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
