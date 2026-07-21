# Purpose

CCBRT(Z) calculates the complex cube root of Z. The principal root for which -PI .LT. arg(Z) .LE. +PI is returned.

# Description

This canonical unsafe binding exposes original SLATEC routine `CCBRT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CCBRT](https://www.netlib.org/slatec/fnlib/ccbrt.f).

# Arguments

## `Z`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the cube root

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_complex32)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::complex::ccbrt`
- Original SLATEC routine: `CCBRT`
- Native symbol: `ccbrt_`
- ABI fingerprint: `function:complex32(mut_complex32)`
- Exact Netlib source file: [CCBRT](https://www.netlib.org/slatec/fnlib/ccbrt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
