# Purpose

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Subroutine POLYVL calculates the value of the polynomial and its first NDER derivatives where the polynomial was produced by a previous call to POLINT.

# Description

This canonical unsafe binding exposes original SLATEC routine `POLYVL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POLYVL](https://www.netlib.org/slatec/src/polyvl.f).

# Arguments

## 1. `NDER`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is .GT. 0. Note *** 0, neither YP nor WORK need to be dimensioned variables. 1, YP does not need to be a dimensioned variable. the number of derivatives to be evaluated 0, WORK does not need to be a dimensioned variable. is .GT. 0. Note *** 0, neither YP nor WORK need to be dimensioned variables. 1, YP does not need to be a dimensioned variable. the number of derivatives to be evaluated 0, WORK does not need to be a dimensioned variable. not applicable or not stated by selected source not a workspace argument

## 2. `XX`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. the argument at which the polynomial and its derivatives are to be evaluated. 1,...,NDER. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `YFIT`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. the value of the polynomial at XX not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `YP`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). must be dimensioned by at least NDER the derivatives of the polynomial at XX.  The derivative of 1,...,NDER. must be dimensioned by at least NDER the derivatives of the polynomial at XX.  The derivative of 1,...,NDER. not applicable or not stated by selected source not a workspace argument

## 5. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* is .GT. 0. Note *** ***** must not be altered between the call must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* is .GT. 0. Note *** ***** must not be altered between the call not applicable or not stated by selected source not a workspace argument

## 6. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call *       to POLINT and the call to POLYVL. must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call *       to POLINT and the call to POLYVL. not applicable or not stated by selected source not a workspace argument

## 7. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call ***** must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call ***** not applicable or not stated by selected source not a workspace argument

## 8. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is .GT. 0. Note *** this is an array to provide internal working storage for POLYVL.  It must be dimensioned by at least 2*N if NDER is is .GT. 0. Note *** this is an array to provide internal working storage for POLYVL.  It must be dimensioned by at least 2*N if NDER is not applicable or not stated by selected source

## 9. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Output error flag with the following possible values. = 1  indicates normal execution Storage Parameters not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NDER`: not a workspace argument
- `XX`: not a workspace argument
- `YFIT`: not a workspace argument
- `YP`: not a workspace argument
- `N`: not a workspace argument
- `X`: not a workspace argument
- `C`: not a workspace argument
- `WORK`: is .GT. 0. Note *** this is an array to provide internal working storage for POLYVL.  It must be dimensioned by at least 2*N if NDER is
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::polyvl`
- Original SLATEC routine: `POLYVL`
- Native symbol: `polyvl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [POLYVL](https://www.netlib.org/slatec/src/polyvl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
