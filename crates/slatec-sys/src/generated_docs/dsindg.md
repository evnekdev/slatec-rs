# Purpose

DSINDG(X) calculates the double precision sine for double precision argument X where X is in degrees.

# Description

This canonical unsafe binding exposes original SLATEC routine `DSINDG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSINDG](https://www.netlib.org/slatec/fnlib/dsindg.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

is in degrees.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::dsindg`
- Original SLATEC routine: `DSINDG`
- Native symbol: `dsindg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSINDG](https://www.netlib.org/slatec/fnlib/dsindg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
