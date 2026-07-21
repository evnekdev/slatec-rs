# Purpose

CLNGAM computes the natural log of the complex valued gamma function at ZIN, where ZIN is a complex number. This is a preliminary version, which is not accurate.

# Description

This canonical unsafe binding exposes original SLATEC routine `CLNGAM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CLNGAM](https://www.netlib.org/slatec/fnlib/clngam.f).

# Arguments

## `ZIN`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the logarithm of the absolute value of the Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::clngam`
- Original SLATEC routine: `CLNGAM`
- Native symbol: `clngam_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CLNGAM](https://www.netlib.org/slatec/fnlib/clngam.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
