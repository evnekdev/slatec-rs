# Purpose

ERFC(X) calculates the single precision complementary error function for single precision argument X. Series for ERF on the interval 0. to 1.00000D+00 with weighted error 7.10E-18 log weighted error 17.15 significant figures required 16.31 decimal places required 17.71 Series for ERFC on the interval 0. to 2.50000D-01 with weighted error 4.81E-17 log weighted error 16.32 approx significant figures required 15.0 Series for ERC2 on the interval 2.50000D-01 to 1.00000D+00 with weighted error 5.22E-17 log weighted error 16.28 decimal places required 16.96

# Description

This canonical unsafe binding exposes original SLATEC routine `ERFC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ERFC](https://www.netlib.org/slatec/fnlib/erfc.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complementary error function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::error::erfc`
- Original SLATEC routine: `ERFC`
- Native symbol: `erfc_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [ERFC](https://www.netlib.org/slatec/fnlib/erfc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
