# Purpose

DBIE(X) calculates the double precision Airy function of the second kind or the double precision exponentially scaled Airy function of the second kind, depending on the value of the double precision argument X. Evaluate BI(X) for X .LE. 0.0 and BI(X)*EXP(-ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for BIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.45E-32 log weighted error 31.84 significant figures required 30.85 decimal places required 32.40 Series for BIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.29E-33 log weighted error 32.89 significant figures required 31.48 decimal places required 33.45 Series for BIF2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 6.08E-32 log weighted error 31.22 approx significant figures required 30.8 decimal places required 31.80 Series for BIG2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 4.91E-33 log weighted error 32.31 approx significant figures required 31.6 decimal places required 32.90 Series for BIP1 on the interval 1.25000E-01 to 3.53553E-01 with weighted error 1.06E-32 log weighted error 31.98 significant figures required 30.61 decimal places required 32.81 Series for BIP2 on the interval 0. to 1.25000E-01 with weighted error 4.04E-33 log weighted error 32.39 significant figures required 31.15 decimal places required 33.37

# Description

This canonical unsafe binding exposes original SLATEC routine `DBIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBIE](https://www.netlib.org/slatec/fnlib/dbie.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision Airy function of the second kind or the double precision exponentially scaled Airy function of the second kind, depending on the value of the double precision argument X. ZETA)  where ZETA)  where ZETA)  where ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 ZETA = 2/3 * X**(3/2)  for X .GE. 0.0 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.45E-32 with weighted error   1.45E-32 with weighted error   1.45E-32 log weighted error  31.84 log weighted error  31.84 log weighted error  31.84 significant figures required  30.85 significant figures required  30.85 significant figures required  30.85 decimal places required  32.40 decimal places required  32.40 decimal places required  32.40 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.29E-33 with weighted error   1.29E-33 with weighted error   1.29E-33 log weighted error  32.89 log weighted error  32.89 log weighted error  32.89 significant figures required  31.48 significant figures required  31.48 significant figures required  31.48 decimal places required  33.45 decimal places required  33.45 decimal places required  33.45 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   6.08E-32 with weighted error   6.08E-32 with weighted error   6.08E-32 log weighted error  31.22 log weighted error  31.22 log weighted error  31.22 approx significant figures required  30.8 approx significant figures required  30.8 approx significant figures required  30.8 decimal places required  31.80 decimal places required  31.80 decimal places required  31.80 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   4.91E-33 with weighted error   4.91E-33 with weighted error   4.91E-33 log weighted error  32.31 log weighted error  32.31 log weighted error  32.31 approx significant figures required  31.6 approx significant figures required  31.6 approx significant figures required  31.6 decimal places required  32.90 decimal places required  32.90 decimal places required  32.90 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 Series for BIP1       on the interval  1.25000E-01 to  3.53553E-01 with weighted error   1.06E-32 with weighted error   1.06E-32 with weighted error   1.06E-32 log weighted error  31.98 log weighted error  31.98 log weighted error  31.98 significant figures required  30.61 significant figures required  30.61 significant figures required  30.61 decimal places required  32.81 decimal places required  32.81 decimal places required  32.81 Series for BIP2       on the interval  0.          to  1.25000E-01 Series for BIP2       on the interval  0.          to  1.25000E-01 Series for BIP2       on the interval  0.          to  1.25000E-01 with weighted error   4.04E-33 with weighted error   4.04E-33 with weighted error   4.04E-33 log weighted error  32.39 log weighted error  32.39 log weighted error  32.39 significant figures required  31.15 significant figures required  31.15 significant figures required  31.15 decimal places required  33.37 decimal places required  33.37 decimal places required  33.37 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::dbie`
- Original SLATEC routine: `DBIE`
- Native symbol: `dbie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DBIE](https://www.netlib.org/slatec/fnlib/dbie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
