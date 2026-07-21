# Purpose

DBESJ0(X) calculates the double precision Bessel function of the first kind of order zero for double precision argument X. Series for BJ0 on the interval 0. to 1.60000E+01 with weighted error 4.39E-32 log weighted error 31.36 significant figures required 31.21 decimal places required 32.00

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESJ0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESJ0](https://www.netlib.org/slatec/fnlib/dbesj0.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Bessel function of the first kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesj0`
- Original SLATEC routine: `DBESJ0`
- Native symbol: `dbesj0_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESJ0](https://www.netlib.org/slatec/fnlib/dbesj0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
