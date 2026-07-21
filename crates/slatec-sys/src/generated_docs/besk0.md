# Purpose

BESK0(X) calculates the modified (hyperbolic) Bessel function of the third kind of order zero for real argument X .GT. 0.0. Series for BK0 on the interval 0. to 4.00000D+00 with weighted error 3.57E-19 log weighted error 18.45 significant figures required 17.99 decimal places required 18.97

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK0](https://www.netlib.org/slatec/fnlib/besk0.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the modified (hyperbolic) Bessel function of the third kind of order zero for real argument X .GT. 0.0. Series for BK0        on the interval  0.          to  4.00000D+00 with weighted error   3.57E-19 log weighted error  18.45 significant figures required  17.99 decimal places required  18.97 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk0`
- Original SLATEC routine: `BESK0`
- Native symbol: `besk0_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESK0](https://www.netlib.org/slatec/fnlib/besk0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
