# Purpose

Evaluate the complementary incomplete Gamma function DGAMIC = integral from X to infinity of EXP(-T) * T**(A-1.) . DGAMIC is evaluated for arbitrary real values of A and for non- negative values of X (even though DGAMIC is defined for X .LT. 0.0), except that for X = 0 and A .LE. 0.0, DGAMIC is undefined. DGAMIC, A, and X are DOUBLE PRECISION. A slight deterioration of 2 or 3 digits accuracy will occur when DGAMIC is very large or very small in absolute value, because log- arithmic variables are used. Also, if the parameter A is very close to a negative INTEGER (but not a negative integer), there is a loss of accuracy, which is reported if the result is less than half machine precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMIC](https://www.netlib.org/slatec/fnlib/dgamic.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgamic`
- Original SLATEC routine: `DGAMIC`
- Native symbol: `dgamic_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMIC](https://www.netlib.org/slatec/fnlib/dgamic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
