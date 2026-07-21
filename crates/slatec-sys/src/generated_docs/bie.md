# Purpose

Evaluate BI(X) for X .LE. 0 and BI(X)*EXP(ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for BIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.88E-19 log weighted error 18.72 significant figures required 17.74 decimal places required 19.20 Series for BIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 2.61E-17 log weighted error 16.58 significant figures required 15.17 decimal places required 17.03 Series for BIF2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.11E-17 log weighted error 16.95 approx significant figures required 16.5 decimal places required 17.45 Series for BIG2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.19E-18 log weighted error 17.92 approx significant figures required 17.2 decimal places required 18.42 Series for BIP on the interval 1.25000D-01 to 3.53553D-01 with weighted error 1.91E-17 log weighted error 16.72 significant figures required 15.35 decimal places required 17.41 Series for BIP2 on the interval 0. to 1.25000D-01 with weighted error 1.05E-18 log weighted error 17.98 significant figures required 16.74 decimal places required 18.71

# Description

This canonical unsafe binding exposes original SLATEC routine `BIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BIE](https://www.netlib.org/slatec/fnlib/bie.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the double precision Airy function of the second kind or the double precision exponentially scaled Airy function of the second kind, depending on the value of the double precision argument X. ZETA)  where ZETA)  where ZETA)  where ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.45E-32 with weighted error   1.45E-32 with weighted error   1.45E-32 log weighted error  31.84 log weighted error  31.84 log weighted error  31.84 significant figures required  30.85 significant figures required  30.85 significant figures required  30.85 decimal places required  32.40 decimal places required  32.40 decimal places required  32.40 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.29E-33 with weighted error   1.29E-33 with weighted error   1.29E-33 log weighted error  32.89 log weighted error  32.89 log weighted error  32.89 significant figures required  31.48 significant figures required  31.48 significant figures required  31.48 decimal places required  33.45 decimal places required  33.45 decimal places required  33.45 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   6.08E-32 with weighted error   6.08E-32 with weighted error   6.08E-32 log weighted error  31.22 log weighted error  31.22 log weighted error  31.22 approx significant figures required  30.8 approx significant figures required  30.8 approx significant figures required  30.8 decimal places required  31.80 decimal places required  31.80 decimal places required  31.80 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   4.91E-33 with weighted error   4.91E-33 with weighted error   4.91E-33 log weighted error  32.31 log weighted error  32.31 log weighted error  32.31 approx significant figures required  31.6 approx significant figures required  31.6 approx significant figures required  31.6 decimal places required  32.90 decimal places required  32.90 decimal places required  32.90 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 with weighted error   1.06E-32 with weighted error   1.06E-32 with weighted error   1.06E-32 log weighted error  31.98 log weighted error  31.98 log weighted error  31.98 significant figures required  30.61 significant figures required  30.61 significant figures required  30.61 decimal places required  32.81 decimal places required  32.81 decimal places required  32.81 Series for BIP2       on the interval  0.          to  1.25000E-01 Series for BIP2       on the interval  0.          to  1.25000E-01 Series for BIP2       on the interval  0.          to  1.25000E-01 with weighted error   4.04E-33 with weighted error   4.04E-33 with weighted error   4.04E-33 log weighted error  32.39 log weighted error  32.39 log weighted error  32.39 significant figures required  31.15 significant figures required  31.15 significant figures required  31.15 decimal places required  33.37 decimal places required  33.37 decimal places required  33.37 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::bie`
- Original SLATEC routine: `BIE`
- Native symbol: `bie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [BIE](https://www.netlib.org/slatec/fnlib/bie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
