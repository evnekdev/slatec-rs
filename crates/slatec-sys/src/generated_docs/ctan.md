# Purpose

CTAN(Z) calculates the complex trigonometric tangent of complex argument Z. Z is in units of radians.

# Description

This canonical unsafe binding exposes original SLATEC routine `CTAN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CTAN](https://www.netlib.org/slatec/fnlib/ctan.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complex tangent

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::ctan`
- Original SLATEC routine: `CTAN`
- Native symbol: `ctan_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CTAN](https://www.netlib.org/slatec/fnlib/ctan.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
