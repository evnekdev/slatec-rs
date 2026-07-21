# Purpose

DBESY1(X) calculates the double precision Bessel function of the second kind of order for double precision argument X. Series for BY1 on the interval 0. to 1.60000E+01 with weighted error 8.65E-33 log weighted error 32.06 significant figures required 32.17 decimal places required 32.71

# Description

This canonical unsafe binding exposes original SLATEC routine `DBESY1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBESY1](https://www.netlib.org/slatec/fnlib/dbesy1.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Compute the Bessel function of the second kind of order one

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::bessel::dbesy1`
- Original SLATEC routine: `DBESY1`
- Native symbol: `dbesy1_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBESY1](https://www.netlib.org/slatec/fnlib/dbesy1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
