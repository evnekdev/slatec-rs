# Purpose

DBESK1(X) calculates the double precision modified (hyperbolic) Bessel function of the third kind of order one for double precision argument X. The argument must be large enough that the result does not overflow and small enough that the result does not underflow. Series for BK1 on the interval 0. to 4.00000E+00 with weighted error 9.16E-32 log weighted error 31.04 significant figures required 30.61 decimal places required 31.64

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESK1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESK1](https://www.netlib.org/slatec/fnlib/dbesk1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the modified (hyperbolic) Bessel function of the third kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesk1`
- Original SLATEC routine: `DBESK1`
- Native symbol: `dbesk1_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESK1](https://www.netlib.org/slatec/fnlib/dbesk1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
