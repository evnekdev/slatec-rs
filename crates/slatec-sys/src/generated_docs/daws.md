# Purpose

DAWS(X) calculates Dawson's integral for real argument X. Series for DAW on the interval 0. to 1.00000D+00 with weighted error 3.83E-17 log weighted error 16.42 significant figures required 15.78 decimal places required 16.97 Series for DAW2 on the interval 0. to 1.60000D+01 with weighted error 5.17E-17 log weighted error 16.29 significant figures required 15.90 decimal places required 17.02 Series for DAWA on the interval 0. to 6.25000D-02 with weighted error 2.24E-17 log weighted error 16.65 significant figures required 14.73 decimal places required 17.36

# Description

This canonical unsafe binding exposes original SLATEC routine `DAWS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DAWS](https://www.netlib.org/slatec/fnlib/daws.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute Dawson's function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::elementary::daws`
- Original SLATEC routine: `DAWS`
- Native symbol: `daws_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DAWS](https://www.netlib.org/slatec/fnlib/daws.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
