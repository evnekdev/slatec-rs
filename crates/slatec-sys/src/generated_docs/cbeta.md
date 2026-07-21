# Purpose

CBETA computes the complete beta function of complex parameters A and B.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBETA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBETA](https://www.netlib.org/slatec/fnlib/cbeta.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. complex and the real part of A positive not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. complex and the real part of B positive not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32,mut_complex32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::cbeta`
- Original SLATEC routine: `CBETA`
- Native symbol: `cbeta_`
- ABI fingerprint: `function:complex32(mut_complex32,mut_complex32)`
- Exact Netlib source file: [CBETA](https://www.netlib.org/slatec/fnlib/cbeta.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
