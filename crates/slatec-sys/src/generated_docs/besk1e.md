# Purpose

BESK1E(X) computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order one for real argument X .GT. 0.0, i.e., EXP(X)*K1(X). Series for BK1 on the interval 0. to 4.00000D+00 with weighted error 7.02E-18 log weighted error 17.15 significant figures required 16.73 decimal places required 17.67 Series for AK1 on the interval 1.25000D-01 to 5.00000D-01 with weighted error 6.06E-17 log weighted error 16.22 significant figures required 15.41 decimal places required 16.83 Series for AK12 on the interval 0. to 1.25000D-01 with weighted error 2.58E-17 log weighted error 16.59 significant figures required 15.22 decimal places required 17.16

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK1E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK1E](https://www.netlib.org/slatec/fnlib/besk1e.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order one for real argument .GT. 0.0, i.e., EXP(X)*K1(X). Series for BK1        on the interval  0.          to  4.00000D+00 with weighted error   7.02E-18 log weighted error  17.15 significant figures required  16.73 decimal places required  17.67 Series for AK1        on the interval  1.25000D-01 to  5.00000D-01 with weighted error   6.06E-17 log weighted error  16.22 significant figures required  15.41 decimal places required  16.83 Series for AK12       on the interval  0.          to  1.25000D-01 with weighted error   2.58E-17 log weighted error  16.59 significant figures required  15.22 decimal places required  17.16 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk1e`
- Original SLATEC routine: `BESK1E`
- Native symbol: `besk1e_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESK1E](https://www.netlib.org/slatec/fnlib/besk1e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
