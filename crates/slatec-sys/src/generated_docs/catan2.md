# Purpose

CATAN2(CSN,CCS) calculates the complex trigonometric arc tangent of the ratio CSN/CCS and returns a result whose real part is in the correct quadrant (within a multiple of 2*PI). The result is in units of radians and the real part is between -PI and +PI.

# Description

This canonical unsafe binding exposes original SLATEC routine `CATAN2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CATAN2](https://www.netlib.org/slatec/fnlib/catan2.f).

# Arguments

## `CSN`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complex arc tangent in the proper quadrant

## `CCS`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complex arc tangent in the proper quadrant

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32,mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::catan2`
- Original SLATEC routine: `CATAN2`
- Native symbol: `catan2_`
- ABI fingerprint: `function:complex32(mut_complex32,mut_complex32)`
- Exact Netlib source file: [CATAN2](https://www.netlib.org/slatec/fnlib/catan2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
