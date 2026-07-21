# Purpose

BESI0E(X) calculates the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero for real argument X; i.e., EXP(-ABS(X))*I0(X). Series for BI0 on the interval 0. to 9.00000D+00 with weighted error 2.46E-18 log weighted error 17.61 significant figures required 17.90 decimal places required 18.15 Series for AI0 on the interval 1.25000D-01 to 3.33333D-01 with weighted error 7.87E-17 log weighted error 16.10 significant figures required 14.69 decimal places required 16.76 Series for AI02 on the interval 0. to 1.25000D-01 with weighted error 3.79E-17 log weighted error 16.42 significant figures required 14.86 decimal places required 17.09

# Description

This canonical unsafe binding exposes original SLATEC routine `BESI0E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESI0E](https://www.netlib.org/slatec/fnlib/besi0e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besi0e`
- Original SLATEC routine: `BESI0E`
- Native symbol: `besi0e_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESI0E](https://www.netlib.org/slatec/fnlib/besi0e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
