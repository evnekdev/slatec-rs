# Purpose

DBESI0(X) calculates the double precision modified (hyperbolic) Bessel function of the first kind of order zero and double precision argument X. Series for BI0 on the interval 0. to 9.00000E+00 with weighted error 9.51E-34 log weighted error 33.02 significant figures required 33.31 decimal places required 33.65

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESI0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESI0](https://www.netlib.org/slatec/fnlib/dbesi0.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision modified (hyperbolic) Bessel function of the first kind of order zero and double precision argument X. Series for BI0        on the interval  0.          to  9.00000E+00 with weighted error   9.51E-34 log weighted error  33.02 significant figures required  33.31 decimal places required  33.65 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesi0`
- Original SLATEC routine: `DBESI0`
- Native symbol: `dbesi0_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESI0](https://www.netlib.org/slatec/fnlib/dbesi0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
