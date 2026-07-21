# Purpose

BESI1E(X) calculates the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for real argument X; i.e., EXP(-ABS(X))*I1(X). Series for BI1 on the interval 0. to 9.00000D+00 with weighted error 2.40E-17 log weighted error 16.62 significant figures required 16.23 decimal places required 17.14 Series for AI1 on the interval 1.25000D-01 to 3.33333D-01 with weighted error 6.98E-17 log weighted error 16.16 significant figures required 14.53 decimal places required 16.82 Series for AI12 on the interval 0. to 1.25000D-01 with weighted error 3.55E-17 log weighted error 16.45 significant figures required 14.69 decimal places required 17.12

# Description

This canonical unsafe binding exposes original SLATEC routine `BESI1E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BESI1E](https://www.netlib.org/slatec/fnlib/besi1e.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. calculates the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for real argument X; i.e., EXP(-ABS(X))*I1(X). Series for BI1        on the interval  0.          to  9.00000D+00 with weighted error   2.40E-17 log weighted error  16.62 significant figures required  16.23 decimal places required  17.14 Series for AI1        on the interval  1.25000D-01 to  3.33333D-01 with weighted error   6.98E-17 log weighted error  16.16 significant figures required  14.53 decimal places required  16.82 Series for AI12       on the interval  0.          to  1.25000D-01 with weighted error   3.55E-17 log weighted error  16.45 significant figures required  14.69 decimal places required  17.12 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::besi1e`
- Original SLATEC routine: `BESI1E`
- Native symbol: `besi1e_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [BESI1E](https://www.netlib.org/slatec/fnlib/besi1e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
