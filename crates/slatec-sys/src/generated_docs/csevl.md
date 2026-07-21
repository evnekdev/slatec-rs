# Purpose

Evaluate the N-term Chebyshev series CS at X. Adapted from a method presented in the paper by Broucke referenced below. Input Arguments --

# Description

This canonical unsafe binding exposes original SLATEC routine `CSEVL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSEVL](https://www.netlib.org/slatec/fnlib/csevl.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

value at which the series is to be evaluated.

## `CS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

array of N terms of a Chebyshev series. In evaluating CS, only half the first coefficient is summed.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of terms in array CS.

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32_ptr_rank1,mut_i32)`. It has no separate Rust `Result` status channel.

# Workspace and array requirements

- `CS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::csevl`
- Original SLATEC routine: `CSEVL`
- Native symbol: `csevl_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [CSEVL](https://www.netlib.org/slatec/fnlib/csevl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
