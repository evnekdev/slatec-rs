# Purpose

DATANH(X) calculates the double precision arc hyperbolic tangent for double precision argument X. Series for ATNH on the interval 0. to 2.50000E-01 with weighted error 6.86E-32 log weighted error 31.16 significant figures required 30.00 decimal places required 31.88

# Description

This canonical unsafe binding exposes original SLATEC routine `DATANH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DATANH](https://www.netlib.org/slatec/fnlib/datanh.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision arc hyperbolic tangent for double precision argument X. Series for ATNH       on the interval  0.          to  2.50000E-01 with weighted error   6.86E-32 log weighted error  31.16 significant figures required  30.00 decimal places required  31.88 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::datanh`
- Original SLATEC routine: `DATANH`
- Native symbol: `datanh_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DATANH](https://www.netlib.org/slatec/fnlib/datanh.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
