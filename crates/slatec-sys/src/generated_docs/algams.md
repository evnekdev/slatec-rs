# Purpose

Evaluates the logarithm of the absolute value of the gamma function.

# Description

This canonical unsafe binding exposes original SLATEC routine `ALGAMS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ALGAMS](https://www.netlib.org/slatec/fnlib/algams.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

input argument.

## `ALGAM`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

result.

## `SGNGAM`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is set to the sign of GAMMA(X) and will be returned at +1. 0 or -1. 0.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::algams`
- Original SLATEC routine: `ALGAMS`
- Native symbol: `algams_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32)`
- Exact Netlib source file: [ALGAMS](https://www.netlib.org/slatec/fnlib/algams.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
