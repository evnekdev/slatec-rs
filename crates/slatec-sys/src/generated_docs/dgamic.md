# Purpose

Evaluate the complementary incomplete Gamma function

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMIC](https://www.netlib.org/slatec/fnlib/dgamic.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative values of X (even though DGAMIC is defined for X .LT. are DOUBLE PRECISION. slight deterioration of 2 or 3 digits accuracy will occur when DGAMIC is very large or very small in absolute value, because log- arithmic variables are used.  Also, if the parameter A is very close is a loss is a loss of accuracy, which is reported if the result is less than half of accuracy, which is reported if the result is less than half machine precision. machine precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. T) * T**(A-1.)  . 0 and A .LE. 0.0, DGAMIC is undefined. are DOUBLE PRECISION. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgamic`
- Original SLATEC routine: `DGAMIC`
- Native symbol: `dgamic_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMIC](https://www.netlib.org/slatec/fnlib/dgamic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
