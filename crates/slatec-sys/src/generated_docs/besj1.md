# Purpose

BESJ1(X) calculates the Bessel function of the first kind of order one for real argument X. Series for BJ1 on the interval 0. to 1.60000D+01 with weighted error 4.48E-17 log weighted error 16.35 significant figures required 15.77 decimal places required 16.89 Series for BM1 on the interval 0. to 6.25000D-02 with weighted error 5.61E-17 log weighted error 16.25 significant figures required 14.97 decimal places required 16.91 Series for BTH1 on the interval 0. to 6.25000D-02 with weighted error 4.10E-17 log weighted error 16.39 significant figures required 15.96 decimal places required 17.08

# Description

This canonical unsafe binding exposes original SLATEC routine `BESJ1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESJ1](https://www.netlib.org/slatec/fnlib/besj1.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the Bessel function of the first kind of order one for real argument X. Series for BJ1        on the interval  0.          to  1.60000D+01 with weighted error   4.48E-17 log weighted error  16.35 significant figures required  15.77 decimal places required  16.89 Series for BM1        on the interval  0.          to  6.25000D-02 with weighted error   5.61E-17 log weighted error  16.25 significant figures required  14.97 decimal places required  16.91 Series for BTH1       on the interval  0.          to  6.25000D-02 with weighted error   4.10E-17 log weighted error  16.39 significant figures required  15.96 decimal places required  17.08 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besj1`
- Original SLATEC routine: `BESJ1`
- Native symbol: `besj1_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESJ1](https://www.netlib.org/slatec/fnlib/besj1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
