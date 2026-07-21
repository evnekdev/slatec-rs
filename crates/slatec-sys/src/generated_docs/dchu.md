# Purpose

DCHU(A,B,X) calculates the double precision logarithmic confluent hypergeometric function U(A,B,X) for double precision arguments A, B, and X. This routine is not valid when 1+A-B is close to zero if X is small.

# Description

This canonical unsafe binding exposes original SLATEC routine `DCHU`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCHU](https://www.netlib.org/slatec/fnlib/dchu.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

real.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

real.

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

real and positive This routine is not valid when 1+A-B is close to zero if X is small.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dchu`
- Original SLATEC routine: `DCHU`
- Native symbol: `dchu_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64)`
- Exact Netlib source file: [DCHU](https://www.netlib.org/slatec/fnlib/dchu.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
