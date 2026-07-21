# Purpose

BI(X) calculates the Airy function of the second kind for real argument X. Series for BIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.88E-19 log weighted error 18.72 significant figures required 17.74 decimal places required 19.20 Series for BIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 2.61E-17 log weighted error 16.58 significant figures required 15.17 decimal places required 17.03 Series for BIF2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.11E-17 log weighted error 16.95 approx significant figures required 16.5 decimal places required 17.45 Series for BIG2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.19E-18 log weighted error 17.92 approx significant figures required 17.2 decimal places required 18.42

# Description

This canonical unsafe binding exposes original SLATEC routine `BI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BI](https://www.netlib.org/slatec/fnlib/bi.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the Airy function of the second kind for real argument X. Series for BIF        on the interval -1.00000D+00 to  1.00000D+00 with weighted error   1.88E-19 log weighted error  18.72 significant figures required  17.74 decimal places required  19.20 Series for BIG        on the interval -1.00000D+00 to  1.00000D+00 with weighted error   2.61E-17 log weighted error  16.58 significant figures required  15.17 decimal places required  17.03 Series for BIF2       on the interval  1.00000D+00 to  8.00000D+00 with weighted error   1.11E-17 log weighted error  16.95 approx significant figures required  16.5 decimal places required  17.45 Series for BIG2       on the interval  1.00000D+00 to  8.00000D+00 with weighted error   1.19E-18 log weighted error  17.92 approx significant figures required  17.2 decimal places required  18.42 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::bi`
- Original SLATEC routine: `BI`
- Native symbol: `bi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [BI](https://www.netlib.org/slatec/fnlib/bi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
