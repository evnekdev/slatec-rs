# Purpose

DBSI0E(X) calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero for double precision argument X. The result is the Bessel function I0(X) multiplied by EXP(-ABS(X)). Series for BI0 on the interval 0. to 9.00000E+00 with weighted error 9.51E-34 log weighted error 33.02 significant figures required 33.31 decimal places required 33.65 Series for AI0 on the interval 1.25000E-01 to 3.33333E-01 with weighted error 2.74E-32 log weighted error 31.56 significant figures required 30.15 decimal places required 32.39 Series for AI02 on the interval 0. to 1.25000E-01 with weighted error 1.97E-32 log weighted error 31.71 decimal places required 32.63

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSI0E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSI0E](https://www.netlib.org/slatec/fnlib/dbsi0e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbsi0e`
- Original SLATEC routine: `DBSI0E`
- Native symbol: `dbsi0e_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBSI0E](https://www.netlib.org/slatec/fnlib/dbsi0e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
