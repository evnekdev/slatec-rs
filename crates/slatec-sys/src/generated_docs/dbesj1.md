# Purpose

DBESJ1(X) calculates the double precision Bessel function of the first kind of order one for double precision argument X. Series for BJ1 on the interval 0. to 1.60000E+01 with weighted error 1.16E-33 log weighted error 32.93 significant figures required 32.36 decimal places required 33.57

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESJ1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESJ1](https://www.netlib.org/slatec/fnlib/dbesj1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Bessel function of the first kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesj1`
- Original SLATEC routine: `DBESJ1`
- Native symbol: `dbesj1_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESJ1](https://www.netlib.org/slatec/fnlib/dbesj1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
