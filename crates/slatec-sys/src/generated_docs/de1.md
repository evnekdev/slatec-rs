# Purpose

DE1 calculates the double precision exponential integral, E1(X), for positive double precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X,

# Description

This canonical unsafe binding exposes original SLATEC routine `DE1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DE1](https://www.netlib.org/slatec/fnlib/de1.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Ei(-X) or E1(-X). Series for AE10       on the interval -3.12500E-02 to  0. with weighted error   4.62E-32 log weighted error  31.34 significant figures required  29.70 decimal places required  32.18 Series for AE11       on the interval -1.25000E-01 to -3.12500E-02 with weighted error   2.22E-32 log weighted error  31.65 significant figures required  30.75 decimal places required  32.54 Series for AE12       on the interval -2.50000E-01 to -1.25000E-01 with weighted error   5.19E-32 log weighted error  31.28 significant figures required  30.82 decimal places required  32.09 Series for E11        on the interval -4.00000E+00 to -1.00000E+00 with weighted error   8.49E-34 log weighted error  33.07 significant figures required  34.13 decimal places required  33.80 Series for E12        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   8.08E-33 log weighted error  32.09 approx significant figures required  30.4 decimal places required  32.79 Series for AE13       on the interval  2.50000E-01 to  1.00000E+00 with weighted error   6.65E-32 log weighted error  31.18 significant figures required  30.69 decimal places required  32.03 Series for AE14       on the interval  0.          to  2.50000E-01 with weighted error   5.07E-32 log weighted error  31.30 significant figures required  30.40 decimal places required  32.20 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::de1`
- Original SLATEC routine: `DE1`
- Native symbol: `de1_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DE1](https://www.netlib.org/slatec/fnlib/de1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
