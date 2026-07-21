# Purpose

COSDG(X) evaluates the cosine for real X in degrees.

# Description

This canonical unsafe binding exposes original SLATEC routine `COSDG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COSDG](https://www.netlib.org/slatec/fnlib/cosdg.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the cosine of an argument in degrees

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::cosdg`
- Original SLATEC routine: `COSDG`
- Native symbol: `cosdg_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [COSDG](https://www.netlib.org/slatec/fnlib/cosdg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
