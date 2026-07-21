# Purpose

DBETA(A,B) calculates the double precision complete beta function for double precision arguments A and B.

# Description

This canonical unsafe binding exposes original SLATEC routine `DBETA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBETA](https://www.netlib.org/slatec/fnlib/dbeta.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

real and positive.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

real and positive.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::beta::dbeta`
- Original SLATEC routine: `DBETA`
- Native symbol: `dbeta_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DBETA](https://www.netlib.org/slatec/fnlib/dbeta.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
