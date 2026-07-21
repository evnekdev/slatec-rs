# Purpose

Evaluate a double precision generalization of Pochhammer's symbol

# Description

This canonical unsafe binding exposes original SLATEC routine `DPOCH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPOCH](https://www.netlib.org/slatec/fnlib/dpoch.f).

# Arguments

## 1. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. sub-X = GAMMA(A+X)/GAMMA(A) for double precision A and X. negative integer, POCH(A,X) is just Pochhammer's symbol. This is a preliminary version that does not handle wrong arguments properly and may not properly handle the case when the result is computed to less than half of double precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative integer, POCH(A,X) is just Pochhammer's symbol. This is a preliminary version that does not handle wrong arguments properly and may not properly handle the case when the result is computed to less than half of double precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `X`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::dpoch`
- Original SLATEC routine: `DPOCH`
- Native symbol: `dpoch_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64)`
- Exact Netlib source file: [DPOCH](https://www.netlib.org/slatec/fnlib/dpoch.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
