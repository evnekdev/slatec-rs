# Purpose

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Subroutine POLINT is designed to produce the polynomial which

# Description

This canonical unsafe binding exposes original SLATEC routine `POLINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POLINT](https://www.netlib.org/slatec/src/polint.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the number of data points  (N .GE. 1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,...,N.  POLINT sets up information in the array C which can be used by subroutine POLYVL to evaluate the polynomial and its derivatives and by subroutine POLCOF to produce the coefficients. Formal Parameters the array of abscissas (all of which must be distinct) must be dimensioned at least N in the calling program. 1,...,N.  POLINT sets up information in the array C which can be used by subroutine POLYVL to evaluate the polynomial and its derivatives and by subroutine POLCOF to produce the coefficients. Formal Parameters the array of abscissas (all of which must be distinct) must be dimensioned at least N in the calling program. not applicable or not stated by selected source not a workspace argument

## 3. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1,...,N.  POLINT sets up information in the array C which can be used by subroutine POLYVL to evaluate the polynomial and its derivatives and by subroutine POLCOF to produce the coefficients. Formal Parameters the array of ordinates must be dimensioned at least N in the calling program. 1,...,N.  POLINT sets up information in the array C which can be used by subroutine POLYVL to evaluate the polynomial and its derivatives and by subroutine POLCOF to produce the coefficients. Formal Parameters the array of ordinates must be dimensioned at least N in the calling program. not applicable or not stated by selected source not a workspace argument

## 4. `C`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). an array of information used by subroutines Dimensioning Information  ******* must be dimensioned at least N in the calling program. an array of information used by subroutines Dimensioning Information  ******* must be dimensioned at least N in the calling program. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `Y`: not a workspace argument
- `C`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::polint`
- Original SLATEC routine: `POLINT`
- Native symbol: `polint_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [POLINT](https://www.netlib.org/slatec/src/polint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
