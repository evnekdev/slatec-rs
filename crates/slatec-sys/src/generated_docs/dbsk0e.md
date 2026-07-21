# Purpose

DBSK0E(X) computes the double precision exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero for positive double precision argument X. Series for BK0 on the interval 0. to 4.00000E+00 with weighted error 3.08E-33 log weighted error 32.51 significant figures required 32.05 decimal places required 33.11 Series for AK0 on the interval 1.25000E-01 to 5.00000E-01 with weighted error 2.85E-32 log weighted error 31.54 significant figures required 30.19 decimal places required 32.33 Series for AK02 on the interval 0. to 1.25000E-01 with weighted error 2.30E-32 log weighted error 31.64 significant figures required 29.68 decimal places required 32.40

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSK0E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSK0E](https://www.netlib.org/slatec/fnlib/dbsk0e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbsk0e`
- Original SLATEC routine: `DBSK0E`
- Native symbol: `dbsk0e_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBSK0E](https://www.netlib.org/slatec/fnlib/dbsk0e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
