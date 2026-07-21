# Purpose

DBSK1E(S) computes the double precision exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one for positive double precision argument X. Series for BK1 on the interval 0. to 4.00000E+00 with weighted error 9.16E-32 log weighted error 31.04 significant figures required 30.61 decimal places required 31.64 Series for AK1 on the interval 1.25000E-01 to 5.00000E-01 with weighted error 3.07E-32 log weighted error 31.51 significant figures required 30.71 decimal places required 32.30 Series for AK12 on the interval 0. to 1.25000E-01 with weighted error 2.41E-32 log weighted error 31.62 significant figures required 30.25 decimal places required 32.38

# Description

This canonical unsafe binding exposes original SLATEC routine `DBSK1E`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBSK1E](https://www.netlib.org/slatec/fnlib/dbsk1e.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input positive double-precision argument. The function returns the exponentially scaled order-one modified Bessel K value at `X`; native code does not modify the scalar.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64)`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dbsk1e`
- Original SLATEC routine: `DBSK1E`
- Native symbol: `dbsk1e_`
- ABI fingerprint: `function:f64(mut_f64)`
- Exact Netlib source file: [DBSK1E](https://www.netlib.org/slatec/fnlib/dbsk1e.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
