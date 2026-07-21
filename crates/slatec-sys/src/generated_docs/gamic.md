# Purpose

Evaluate the complementary incomplete gamma function GAMIC = integral from X to infinity of EXP(-T) * T**(A-1.) . GAMIC is evaluated for arbitrary real values of A and for non- negative values of X (even though GAMIC is defined for X .LT. 0.0), except that for X = 0 and A .LE. 0.0, GAMIC is undefined. GAMIC, A, and X are REAL. A slight deterioration of 2 or 3 digits accuracy will occur when GAMIC is very large or very small in absolute value, because log- arithmic variables are used. Also, if the parameter A is very close to a negative integer (but not a negative integer), there is a loss of accuracy, which is reported if the result is less than half machine precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMIC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMIC](https://www.netlib.org/slatec/fnlib/gamic.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::gamic`
- Original SLATEC routine: `GAMIC`
- Native symbol: `gamic_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [GAMIC](https://www.netlib.org/slatec/fnlib/gamic.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
