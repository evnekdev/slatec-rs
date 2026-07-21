# Purpose

CASIN(ZINP) calculates the complex trigonometric arc sine of ZINP. The result is in units of radians, and the real part is in the first or fourth quadrant.

# Description

This canonical unsafe binding exposes original SLATEC routine `CASIN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CASIN](https://www.netlib.org/slatec/fnlib/casin.f).

# Arguments

## `ZINP`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the complex arc sine

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::casin`
- Original SLATEC routine: `CASIN`
- Native symbol: `casin_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CASIN](https://www.netlib.org/slatec/fnlib/casin.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
