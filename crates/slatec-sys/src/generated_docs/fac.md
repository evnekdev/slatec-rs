# Purpose

FAC(N) evaluates the factorial function of N. FAC is single precision. N must be an integer between 0 and 25 inclusive.

# Description

This canonical unsafe binding exposes original SLATEC routine `FAC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FAC](https://www.netlib.org/slatec/fnlib/fac.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the factorial function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::fac`
- Original SLATEC routine: `FAC`
- Native symbol: `fac_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [FAC](https://www.netlib.org/slatec/fnlib/fac.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
