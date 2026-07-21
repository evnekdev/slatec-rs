# Purpose

ALI(X) computes the logarithmic integral; i.e., the integral from 0.0 to X of (1.0/ln(t))dt.

# Description

This canonical unsafe binding exposes original SLATEC routine `ALI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ALI](https://www.netlib.org/slatec/fnlib/ali.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computes the logarithmic integral; i.e., the integral from 0.0 to X of (1.0/ln(t))dt. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::ali`
- Original SLATEC routine: `ALI`
- Native symbol: `ali_`
- ABI fingerprint: `function:f32(mut_f32)`
- Exact Netlib source file: [ALI](https://www.netlib.org/slatec/fnlib/ali.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
