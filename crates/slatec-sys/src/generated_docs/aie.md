# Purpose

AIE(X) computes the exponentially scaled Airy function for non-negative X. It evaluates AI(X) for X .LE. 0.0 and EXP(ZETA)*AI(X) for X .GE. 0.0 where ZETA = (2.0/3.0)*(X**1.5). Series for AIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.09E-19 log weighted error 18.96 significant figures required 17.76 decimal places required 19.44 Series for AIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.51E-17 log weighted error 16.82 significant figures required 15.19 decimal places required 17.27 Series for AIP on the interval 0. to 1.00000D+00 with weighted error 5.10E-17 log weighted error 16.29 significant figures required 14.41 decimal places required 17.06

# Description

This canonical unsafe binding exposes original SLATEC routine `AIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [AIE](https://www.netlib.org/slatec/fnlib/aie.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::aie`
- Original SLATEC routine: `AIE`
- Native symbol: `aie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [AIE](https://www.netlib.org/slatec/fnlib/aie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
