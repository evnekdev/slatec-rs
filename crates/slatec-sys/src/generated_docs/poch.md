# Purpose

Evaluate a generalization of Pochhammer's symbol

# Description

This canonical unsafe binding exposes original SLATEC routine `POCH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POCH](https://www.netlib.org/slatec/fnlib/poch.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. sub-X = GAMMA(A+X)/GAMMA(A).  For X a non-negative integer, is just Pochhammer's symbol.  A and X are single precision. This is a preliminary version.  Error handling when POCH(A,X) is less than half precision is probably incorrect.  Grossly incorrect arguments are not handled properly. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is just Pochhammer's symbol.  A and X are single precision. This is a preliminary version.  Error handling when POCH(A,X) is less than half precision is probably incorrect.  Grossly incorrect arguments are not handled properly. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::poch`
- Original SLATEC routine: `POCH`
- Native symbol: `poch_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32)`
- Exact Netlib source file: [POCH](https://www.netlib.org/slatec/fnlib/poch.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
