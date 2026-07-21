# Purpose

DBESI1(X) calculates the double precision modified (hyperbolic) Bessel function of the first kind of order one and double precision argument X. Series for BI1 on the interval 0. to 9.00000E+00 with weighted error 1.44E-32 log weighted error 31.84 significant figures required 31.45 decimal places required 32.46

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESI1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESI1](https://www.netlib.org/slatec/fnlib/dbesi1.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. calculates the double precision modified (hyperbolic) Bessel function of the first kind of order one and double precision argument X. Series for BI1        on the interval  0.          to  9.00000E+00 with weighted error   1.44E-32 log weighted error  31.84 significant figures required  31.45 decimal places required  32.46 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesi1`
- Original SLATEC routine: `DBESI1`
- Native symbol: `dbesi1_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESI1](https://www.netlib.org/slatec/fnlib/dbesi1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
