# Purpose

Evaluate BI(X) for X .LE. 0 and BI(X)*EXP(ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for BIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.88E-19 log weighted error 18.72 significant figures required 17.74 decimal places required 19.20 Series for BIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 2.61E-17 log weighted error 16.58 significant figures required 15.17 decimal places required 17.03 Series for BIF2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.11E-17 log weighted error 16.95 approx significant figures required 16.5 decimal places required 17.45 Series for BIG2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.19E-18 log weighted error 17.92 approx significant figures required 17.2 decimal places required 18.42 Series for BIP on the interval 1.25000D-01 to 3.53553D-01 with weighted error 1.91E-17 log weighted error 16.72 significant figures required 15.35 decimal places required 17.41 Series for BIP2 on the interval 0. to 1.25000D-01 with weighted error 1.05E-18 log weighted error 17.98 significant figures required 16.74 decimal places required 18.71

# Description

This canonical unsafe binding exposes original SLATEC routine `BIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BIE](https://www.netlib.org/slatec/fnlib/bie.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the Bairy function for a negative argument and an exponentially scaled Bairy function for a non-negative argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::bie`
- Original SLATEC routine: `BIE`
- Native symbol: `bie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [BIE](https://www.netlib.org/slatec/fnlib/bie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
