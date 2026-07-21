# Purpose

MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS

# Description

This canonical unsafe binding exposes original SLATEC routine `QMOMO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QMOMO](https://www.netlib.org/slatec/src/qmomo.f).

# Arguments

## 1. `ALFA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real 1) 1,X), K = 1, ..., 25. 1,X), K = 1, ..., 25. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `BETA`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real 1) K = 1, ..., 25. K = 1, ..., 25. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `RI`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Vector of dimension 25 1,1) of Real Vector of dimension 25 1,1) of not applicable or not stated by selected source not a workspace argument

## 4. `RJ`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Vector of dimension 25 1,1) of Real Vector of dimension 25 1,1) of not applicable or not stated by selected source not a workspace argument

## 5. `RG`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Vector of dimension 25 1,1) of Real Vector of dimension 25 1,1) of not applicable or not stated by selected source not a workspace argument

## 6. `RH`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (25). Real Vector of dimension 25 1,1) of Real Vector of dimension 25 1,1) of not applicable or not stated by selected source not a workspace argument

## 7. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Input parameter indicating the modified Moments to be computed 1 compute RI, RJ = 2 compute RI, RJ, RG = 3 compute RI, RJ, RH = 4 compute RI, RJ, RG, RH not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ALFA`: not a workspace argument
- `BETA`: not a workspace argument
- `RI`: not a workspace argument
- `RJ`: not a workspace argument
- `RG`: not a workspace argument
- `RH`: not a workspace argument
- `INTEGR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qmomo`
- Original SLATEC routine: `QMOMO`
- Native symbol: `qmomo_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [QMOMO](https://www.netlib.org/slatec/src/qmomo.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
