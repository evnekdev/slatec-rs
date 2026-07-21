# Purpose

DBESY0(X) calculates the double precision Bessel function of the second kind of order zero for double precision argument X. Series for BY0 on the interval 0. to 1.60000E+01 with weighted error 8.14E-32 log weighted error 31.09 significant figures required 30.31 decimal places required 31.73

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESY0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESY0](https://www.netlib.org/slatec/fnlib/dbesy0.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision Bessel function of the second kind of order zero for double precision argument X. Series for BY0        on the interval  0.          to  1.60000E+01 with weighted error   8.14E-32 log weighted error  31.09 significant figures required  30.31 decimal places required  31.73 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesy0`
- Original SLATEC routine: `DBESY0`
- Native symbol: `dbesy0_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESY0](https://www.netlib.org/slatec/fnlib/dbesy0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
