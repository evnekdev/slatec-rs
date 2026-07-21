# Purpose

Evaluate Tricomi's incomplete Gamma function defined by DGAMIT = X**(-A)/GAMMA(A) * integral from 0 to X of EXP(-T) * T**(A-1.) for A .GT. 0.0 and by analytic continuation for A .LE. 0.0. GAMMA(X) is the complete gamma function of X. DGAMIT is evaluated for arbitrary real values of A and for non- negative values of X (even though DGAMIT is defined for X .LT. 0.0), except that for X = 0 and A .LE. 0.0, DGAMIT is infinite, which is a fatal error. The function and both arguments are DOUBLE PRECISION. A slight deterioration of 2 or 3 digits accuracy will occur when DGAMIT is very large or very small in absolute value, because log- arithmic variables are used. Also, if the parameter A is very close to a negative integer (but not a negative integer), there is a loss of accuracy, which is reported if the result is less than half machine precision.

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMIT](https://www.netlib.org/slatec/fnlib/dgamit.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate Tricomi's form of the incomplete Gamma function

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input value at which the source-defined function is evaluated: Calculate Tricomi's form of the incomplete Gamma function

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamma::dgamit`
- Original SLATEC routine: `DGAMIT`
- Native symbol: `dgamit_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DGAMIT](https://www.netlib.org/slatec/fnlib/dgamit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
