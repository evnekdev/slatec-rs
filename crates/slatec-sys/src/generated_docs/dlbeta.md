# Purpose

DLBETA(A,B) calculates the double precision natural logarithm of the complete beta function for double precision arguments A and B.

# Description

This canonical unsafe binding exposes original SLATEC routine `DLBETA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DLBETA](https://www.netlib.org/slatec/fnlib/dlbeta.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the natural logarithm of the complete Beta function

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the natural logarithm of the complete Beta function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::beta::dlbeta`
- Original SLATEC routine: `DLBETA`
- Native symbol: `dlbeta_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DLBETA](https://www.netlib.org/slatec/fnlib/dlbeta.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
