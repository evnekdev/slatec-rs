# Purpose

BESY1(X) calculates the Bessel function of the second kind of order one for real argument X. Series for BY1 on the interval 0. to 1.60000D+01 with weighted error 1.87E-18 log weighted error 17.73 significant figures required 17.83 decimal places required 18.30 Series for BM1 on the interval 0. to 6.25000D-02 with weighted error 5.61E-17 log weighted error 16.25 significant figures required 14.97 decimal places required 16.91 Series for BTH1 on the interval 0. to 6.25000D-02 with weighted error 4.10E-17 log weighted error 16.39 significant figures required 15.96 decimal places required 17.08

# Description

This canonical unsafe binding exposes original SLATEC routine `BESY1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESY1](https://www.netlib.org/slatec/fnlib/besy1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Bessel function of the second kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besy1`
- Original SLATEC routine: `BESY1`
- Native symbol: `besy1_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESY1](https://www.netlib.org/slatec/fnlib/besy1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
