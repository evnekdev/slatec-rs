# Purpose

CLBETA computes the natural log of the complex valued complete beta function of complex parameters A and B. This is a preliminary version which is not accurate.

# Description

This canonical unsafe binding exposes original SLATEC routine `CLBETA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CLBETA](https://www.netlib.org/slatec/fnlib/clbeta.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

complex and the real part of A positive.

## `B`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

complex and the real part of B positive.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32,mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::clbeta`
- Original SLATEC routine: `CLBETA`
- Native symbol: `clbeta_`
- ABI fingerprint: `function:complex32(mut_complex32,mut_complex32)`
- Exact Netlib source file: [CLBETA](https://www.netlib.org/slatec/fnlib/clbeta.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
