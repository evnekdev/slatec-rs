# Purpose

DCOT(X) calculates the double precision trigonometric cotangent for double precision argument X. X is in units of radians. Series for COT on the interval 0. to 6.25000E-02 with weighted error 5.52E-34 log weighted error 33.26 significant figures required 32.34 decimal places required 33.85

# Description

This canonical unsafe binding exposes original SLATEC routine `DCOT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCOT](https://www.netlib.org/slatec/fnlib/dcot.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision trigonometric cotangent is in units of radians. is in units of radians. Series for COT        on the interval  0.          to  6.25000E-02 Series for COT        on the interval  0.          to  6.25000E-02 with weighted error   5.52E-34 with weighted error   5.52E-34 log weighted error  33.26 log weighted error  33.26 significant figures required  32.34 significant figures required  32.34 decimal places required  33.85 decimal places required  33.85 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dcot`
- Original SLATEC routine: `DCOT`
- Native symbol: `dcot_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DCOT](https://www.netlib.org/slatec/fnlib/dcot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
