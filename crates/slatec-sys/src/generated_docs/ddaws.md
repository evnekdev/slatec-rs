# Purpose

DDAWS(X) calculates the double precision Dawson's integral for double precision argument X. Series for DAW on the interval 0. to 1.00000E+00 with weighted error 8.95E-32 log weighted error 31.05 significant figures required 30.41 decimal places required 31.71 Series for DAW2 on the interval 0. to 1.60000E+01 with weighted error 1.61E-32 log weighted error 31.79 significant figures required 31.40 decimal places required 32.62 Series for DAWA on the interval 0. to 6.25000E-02 with weighted error 1.97E-32 log weighted error 31.71 significant figures required 29.79 decimal places required 32.64

# Description

This canonical unsafe binding exposes original SLATEC routine `DDAWS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DDAWS](https://www.netlib.org/slatec/fnlib/ddaws.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute Dawson's function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::ddaws`
- Original SLATEC routine: `DDAWS`
- Native symbol: `ddaws_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DDAWS](https://www.netlib.org/slatec/fnlib/ddaws.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
