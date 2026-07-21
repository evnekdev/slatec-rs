# Purpose

CHU computes the logarithmic confluent hypergeometric function, U(A,B,X).

# Description

This canonical unsafe binding exposes original SLATEC routine `CHU`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CHU](https://www.netlib.org/slatec/fnlib/chu.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

real.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

real.

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

real and positive This routine is not valid when 1+A-B is close to zero if X is small.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::chu`
- Original SLATEC routine: `CHU`
- Native symbol: `chu_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [CHU](https://www.netlib.org/slatec/fnlib/chu.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
