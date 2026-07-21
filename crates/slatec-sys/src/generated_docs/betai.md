# Purpose

BETAI calculates the REAL incomplete beta function. The incomplete beta function ratio is the probability that a random variable from a beta distribution having parameters PIN and QIN will be less than or equal to X. -- Input Arguments -- All arguments are REAL.

# Description

This canonical unsafe binding exposes original SLATEC routine `BETAI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BETAI](https://www.netlib.org/slatec/fnlib/betai.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

upper limit of integration. X must be in (0,1) inclusive.

## `PIN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

first beta distribution parameter. PIN must be. GT. 0.

## `QIN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

second beta distribution parameter. QIN must be. GT. 0.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::beta::betai`
- Original SLATEC routine: `BETAI`
- Native symbol: `betai_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [BETAI](https://www.netlib.org/slatec/fnlib/betai.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
