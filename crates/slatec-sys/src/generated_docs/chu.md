# Purpose

CHU computes the logarithmic confluent hypergeometric function, U(A,B,X).

# Description

This canonical unsafe binding exposes original SLATEC routine `CHU`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHU](https://www.netlib.org/slatec/fnlib/chu.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. real B is close to zero if X is small. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. real not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. real and positive not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::chu`
- Original SLATEC routine: `CHU`
- Native symbol: `chu_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [CHU](https://www.netlib.org/slatec/fnlib/chu.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
