# Purpose

ASINH(X) computes the arc hyperbolic sine of X. Series for ASNH on the interval 0. to 1.00000D+00 with weighted error 2.19E-17 log weighted error 16.66 significant figures required 15.60 decimal places required 17.31

# Description

This canonical unsafe binding exposes original SLATEC routine `ASINH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ASINH](https://www.netlib.org/slatec/fnlib/asinh.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computes the arc hyperbolic sine of X. Series for ASNH       on the interval  0.          to  1.00000D+00 with weighted error   2.19E-17 log weighted error  16.66 significant figures required  15.60 decimal places required  17.31 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::asinh`
- Original SLATEC routine: `ASINH`
- Native symbol: `asinh_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [ASINH](https://www.netlib.org/slatec/fnlib/asinh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
