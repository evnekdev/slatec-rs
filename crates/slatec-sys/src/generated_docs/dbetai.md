# Purpose

DBETAI calculates the DOUBLE PRECISION incomplete beta function. The incomplete beta function ratio is the probability that a random variable from a beta distribution having parameters PIN and

# Description

This canonical unsafe binding exposes original SLATEC routine `DBETAI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBETAI](https://www.netlib.org/slatec/fnlib/dbetai.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. must be in (0,1) inclusive. must be in (0,1) inclusive. must be in (0,1) inclusive. must be in (0,1) inclusive. not applicable or not stated by selected source not a workspace argument

## 2. `PIN`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. must be .GT. 0.0. must be .GT. 0.0. must be .GT. 0.0. must be .GT. 0.0. not applicable or not stated by selected source not a workspace argument

## 3. `QIN`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. will be less than or equal to X. -- Input Arguments -- All arguments are DOUBLE PRECISION. must be .GT. 0.0. must be .GT. 0.0. will be less than or equal to X. -- Input Arguments -- All arguments are DOUBLE PRECISION. must be .GT. 0.0. must be .GT. 0.0. not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `PIN`: not a workspace argument
- `QIN`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::beta::dbetai`
- Original SLATEC routine: `DBETAI`
- Native symbol: `dbetai_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DBETAI](https://www.netlib.org/slatec/fnlib/dbetai.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
