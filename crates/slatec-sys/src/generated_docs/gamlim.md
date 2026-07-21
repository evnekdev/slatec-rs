# Purpose

Calculate the minimum and maximum legal bounds for X in GAMMA(X).

# Description

This canonical unsafe binding exposes original SLATEC routine `GAMLIM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAMLIM](https://www.netlib.org/slatec/fnlib/gamlim.f).

# Arguments

## `XMIN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

not the only bounds, but they are the only non- trivial ones to calculate. Output Arguments -- minimum legal value of X in GAMMA(X). Any smaller value of X might result in underflow.

## `XMAX`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

not the only bounds, but they are the only non- trivial ones to calculate. Output Arguments -- maximum legal value of X in GAMMA(X). Any larger value will cause overflow.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::gamlim`
- Original SLATEC routine: `GAMLIM`
- Native symbol: `gamlim_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32)`
- Exact Netlib source file: [GAMLIM](https://www.netlib.org/slatec/fnlib/gamlim.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
