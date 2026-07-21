# Purpose

DBI(X) calculates the double precision Airy function of the second kind for double precision argument X. Series for BIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.45E-32 log weighted error 31.84 significant figures required 30.85 decimal places required 32.40 Series for BIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 1.29E-33 log weighted error 32.89 significant figures required 31.48 decimal places required 33.45 Series for BIF2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 6.08E-32 log weighted error 31.22 approx significant figures required 30.8 decimal places required 31.80 Series for BIG2 on the interval 1.00000E+00 to 8.00000E+00 with weighted error 4.91E-33 log weighted error 32.31 approx significant figures required 31.6 decimal places required 32.90

# Description

This canonical unsafe binding exposes original SLATEC routine `DBI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBI](https://www.netlib.org/slatec/fnlib/dbi.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision Airy function of the second kind for double precision argument X. Series for BIF        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.45E-32 log weighted error  31.84 significant figures required  30.85 decimal places required  32.40 Series for BIG        on the interval -1.00000E+00 to  1.00000E+00 with weighted error   1.29E-33 log weighted error  32.89 significant figures required  31.48 decimal places required  33.45 Series for BIF2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   6.08E-32 log weighted error  31.22 approx significant figures required  30.8 decimal places required  31.80 Series for BIG2       on the interval  1.00000E+00 to  8.00000E+00 with weighted error   4.91E-33 log weighted error  32.31 approx significant figures required  31.6 decimal places required  32.90 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::dbi`
- Original SLATEC routine: `DBI`
- Native symbol: `dbi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DBI](https://www.netlib.org/slatec/fnlib/dbi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
