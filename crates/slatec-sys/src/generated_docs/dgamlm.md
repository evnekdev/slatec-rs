# Purpose

Calculate the minimum and maximum legal bounds for X in gamma(X).

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAMLM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAMLM](https://www.netlib.org/slatec/fnlib/dgamlm.f).

# Arguments

## `XMIN`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

not the only bounds, but they are the only non- trivial ones to calculate. Output Arguments -- double precision minimum legal value of X in gamma(X). Any smaller value of X might result in underflow.

## `XMAX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

not the only bounds, but they are the only non- trivial ones to calculate. Output Arguments -- double precision maximum legal value of X in gamma(X). Any larger value of X might cause overflow.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::special::dgamlm`
- Original SLATEC routine: `DGAMLM`
- Native symbol: `dgamlm_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64)`
- Exact Netlib source file: [DGAMLM](https://www.netlib.org/slatec/fnlib/dgamlm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
