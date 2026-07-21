# Purpose

BESJ0(X) calculates the Bessel function of the first kind of order zero for real argument X. Series for BJ0 on the interval 0. to 1.60000D+01 with weighted error 7.47E-18 log weighted error 17.13 significant figures required 16.98 decimal places required 17.68 Series for BM0 on the interval 0. to 6.25000D-02 with weighted error 4.98E-17 log weighted error 16.30 significant figures required 14.97 decimal places required 16.96 Series for BTH0 on the interval 0. to 6.25000D-02 with weighted error 3.67E-17 log weighted error 16.44 significant figures required 15.53 decimal places required 17.13

# Description

This canonical unsafe binding exposes original SLATEC routine `BESJ0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESJ0](https://www.netlib.org/slatec/fnlib/besj0.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Bessel function of the first kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besj0`
- Original SLATEC routine: `BESJ0`
- Native symbol: `besj0_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESJ0](https://www.netlib.org/slatec/fnlib/besj0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
