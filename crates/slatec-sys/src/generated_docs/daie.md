# Purpose

DAIE(X) calculates the Airy function or the exponentially scaled Airy function depending on the value of the argument. The function and argument are both double precision. Evaluate AI(X) for X .LE. 0.0 and AI(X)*EXP(ZETA) where ZETA = 2/3 * X**(3/2) for X .GE. 0.0 Series for AIF on the interval -1.00000E+00 to 1.00000E+00 with weighted error 8.37E-33 log weighted error 32.08 significant figures required 30.87 decimal places required 32.63 Series for AIG on the interval -1.00000E+00 to 1.00000E+00 with weighted error 7.47E-34 log weighted error 33.13 significant figures required 31.50 decimal places required 33.68 Series for AIP1 on the interval 1.25000E-01 to 1.00000E+00 with weighted error 3.69E-32 log weighted error 31.43 significant figures required 29.55 decimal places required 32.31 Series for AIP2 on the interval 0. to 1.25000E-01 with weighted error 3.48E-32 log weighted error 31.46 significant figures required 28.74 decimal places required 32.24

# Description

This canonical unsafe binding exposes original SLATEC routine `DAIE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DAIE](https://www.netlib.org/slatec/fnlib/daie.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::airy::daie`
- Original SLATEC routine: `DAIE`
- Native symbol: `daie_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DAIE](https://www.netlib.org/slatec/fnlib/daie.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
