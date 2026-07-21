# Purpose

AI(X) computes the Airy function Ai(X) Series for AIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.09E-19 log weighted error 18.96 significant figures required 17.76 decimal places required 19.44 Series for AIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.51E-17 log weighted error 16.82 significant figures required 15.19 decimal places required 17.27

# Description

This canonical unsafe binding exposes original SLATEC routine `AI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [AI](https://www.netlib.org/slatec/fnlib/ai.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Evaluate the Airy function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::ai`
- Original SLATEC routine: `AI`
- Native symbol: `ai_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [AI](https://www.netlib.org/slatec/fnlib/ai.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
