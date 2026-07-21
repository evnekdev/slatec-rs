# Purpose

DBESK0(X) calculates the double precision modified (hyperbolic) Bessel function of the third kind of order zero for double precision argument X. The argument must be greater than zero but not so large that the result underflows. Series for BK0 on the interval 0. to 4.00000E+00 with weighted error 3.08E-33 log weighted error 32.51 significant figures required 32.05 decimal places required 33.11

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESK0`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESK0](https://www.netlib.org/slatec/fnlib/dbesk0.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the modified (hyperbolic) Bessel function of the third kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesk0`
- Original SLATEC routine: `DBESK0`
- Native symbol: `dbesk0_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESK0](https://www.netlib.org/slatec/fnlib/dbesk0.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
