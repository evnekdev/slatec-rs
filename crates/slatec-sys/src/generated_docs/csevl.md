# Purpose

Evaluate the N-term Chebyshev series CS at X. Adapted from a method presented in the paper by Broucke referenced below. Input Arguments -X value at which the series is to be evaluated. CS array of N terms of a Chebyshev series. In evaluating CS, only half the first coefficient is summed. N number of terms in array CS.

# Description

This canonical unsafe binding exposes original SLATEC routine `CSEVL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSEVL](https://www.netlib.org/slatec/fnlib/csevl.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. value at which the series is to be evaluated. 1,+1).  (WRB) 920501  Reformatted the REFERENCES section.  (WRB) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). array of N terms of a Chebyshev series.  In evaluating only half the first coefficient is summed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. term Chebyshev series CS at X.  Adapted from a method presented in the paper by Broucke referenced below. Input Arguments -- number of terms in array CS. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32_ptr_rank1,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `CS`: not a workspace argument
- `N`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::csevl`
- Original SLATEC routine: `CSEVL`
- Native symbol: `csevl_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [CSEVL](https://www.netlib.org/slatec/fnlib/csevl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
