# Purpose

BESK0E(X) computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order zero for real argument X .GT. 0.0, i.e., EXP(X)*K0(X). Series for BK0 on the interval 0. to 4.00000D+00 with weighted error 3.57E-19 log weighted error 18.45 significant figures required 17.99 decimal places required 18.97 Series for AK0 on the interval 1.25000D-01 to 5.00000D-01 with weighted error 5.34E-17 log weighted error 16.27 significant figures required 14.92 decimal places required 16.89 Series for AK02 on the interval 0. to 1.25000D-01 with weighted error 2.34E-17 log weighted error 16.63 significant figures required 14.67 decimal places required 17.20

# Description

This canonical unsafe binding exposes original SLATEC routine `BESK0E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESK0E](https://www.netlib.org/slatec/fnlib/besk0e.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computes the exponentially scaled modified (hyperbolic) Bessel function of third kind of order zero for real argument .GT. 0.0, i.e., EXP(X)*K0(X). Series for BK0        on the interval  0.          to  4.00000D+00 with weighted error   3.57E-19 log weighted error  18.45 significant figures required  17.99 decimal places required  18.97 Series for AK0        on the interval  1.25000D-01 to  5.00000D-01 with weighted error   5.34E-17 log weighted error  16.27 significant figures required  14.92 decimal places required  16.89 Series for AK02       on the interval  0.          to  1.25000D-01 with weighted error   2.34E-17 log weighted error  16.63 significant figures required  14.67 decimal places required  17.20 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besk0e`
- Original SLATEC routine: `BESK0E`
- Native symbol: `besk0e_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESK0E](https://www.netlib.org/slatec/fnlib/besk0e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
