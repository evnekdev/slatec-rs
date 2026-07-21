# Purpose

Evaluate the complementary incomplete gamma function

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMIC](https://www.netlib.org/slatec/fnlib/gamic.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. negative values of X (even though GAMIC is defined for X .LT. are REAL. slight deterioration of 2 or 3 digits accuracy will occur when GAMIC is very large or very small in absolute value, because log- arithmic variables are used.  Also, if the parameter A is very close is a loss is a loss of accuracy, which is reported if the result is less than half of accuracy, which is reported if the result is less than half machine precision. machine precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. T) * T**(A-1.)  . 0 and A .LE. 0.0, GAMIC is undefined. are REAL. not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::special::gamma::gamic`
- Original SLATEC routine: `GAMIC`
- Native symbol: `gamic_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [GAMIC](https://www.netlib.org/slatec/fnlib/gamic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
