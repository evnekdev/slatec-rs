# Purpose

DBSI1E(X) calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one for double precision argument X. The result is I1(X) multiplied by EXP(-ABS(X)). Series for BI1 on the interval 0. to 9.00000E+00 with weighted error 1.44E-32 log weighted error 31.84 significant figures required 31.45 decimal places required 32.46 Series for AI1 on the interval 1.25000E-01 to 3.33333E-01 with weighted error 2.81E-32 log weighted error 31.55 significant figures required 29.93 decimal places required 32.38 Series for AI12 on the interval 0. to 1.25000E-01 with weighted error 1.83E-32 log weighted error 31.74 significant figures required 29.97 decimal places required 32.66

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSI1E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSI1E](https://www.netlib.org/slatec/fnlib/dbsi1e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbsi1e`
- Original SLATEC routine: `DBSI1E`
- Native symbol: `dbsi1e_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBSI1E](https://www.netlib.org/slatec/fnlib/dbsi1e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
