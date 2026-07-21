# Purpose

AIE(X) computes the exponentially scaled Airy function for non-negative X. It evaluates AI(X) for X .LE. 0.0 and EXP(ZETA)*AI(X) for X .GE. 0.0 where ZETA = (2.0/3.0)*(X**1.5). Series for AIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.09E-19 log weighted error 18.96 significant figures required 17.76 decimal places required 19.44 Series for AIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.51E-17 log weighted error 16.82 significant figures required 15.19 decimal places required 17.27 Series for AIP on the interval 0. to 1.00000D+00 with weighted error 5.10E-17 log weighted error 16.29 significant figures required 14.41 decimal places required 17.06

# Description

This canonical unsafe binding exposes original SLATEC routine `AIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [AIE](https://www.netlib.org/slatec/fnlib/aie.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computes the exponentially scaled Airy function for non-negative X.  It evaluates AI(X) for X .LE. 0.0 and (2.0/3.0)*(X**1.5). (2.0/3.0)*(X**1.5). Series for AIF        on the interval -1.00000D+00 to  1.00000D+00 Series for AIF        on the interval -1.00000D+00 to  1.00000D+00 with weighted error   1.09E-19 with weighted error   1.09E-19 log weighted error  18.96 log weighted error  18.96 significant figures required  17.76 significant figures required  17.76 decimal places required  19.44 decimal places required  19.44 Series for AIG        on the interval -1.00000D+00 to  1.00000D+00 Series for AIG        on the interval -1.00000D+00 to  1.00000D+00 with weighted error   1.51E-17 with weighted error   1.51E-17 log weighted error  16.82 log weighted error  16.82 significant figures required  15.19 significant figures required  15.19 decimal places required  17.27 decimal places required  17.27 Series for AIP        on the interval  0.          to  1.00000D+00 Series for AIP        on the interval  0.          to  1.00000D+00 with weighted error   5.10E-17 with weighted error   5.10E-17 log weighted error  16.29 log weighted error  16.29 significant figures required  14.41 significant figures required  14.41 decimal places required  17.06 decimal places required  17.06 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::aie`
- Original SLATEC routine: `AIE`
- Native symbol: `aie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [AIE](https://www.netlib.org/slatec/fnlib/aie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
