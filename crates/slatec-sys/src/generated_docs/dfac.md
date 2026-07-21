# Purpose

DFAC(N) calculates the double precision factorial for integer argument N.

# Description

This canonical unsafe binding exposes original SLATEC routine `DFAC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DFAC](https://www.netlib.org/slatec/fnlib/dfac.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the factorial function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dfac`
- Original SLATEC routine: `DFAC`
- Native symbol: `dfac_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DFAC](https://www.netlib.org/slatec/fnlib/dfac.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
