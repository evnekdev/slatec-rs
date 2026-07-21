# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROT](https://www.netlib.org/slatec/lin/srot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SX`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements rotated vector SX (unchanged if N .LE. 0) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SX 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SY`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements rotated vector SY (unchanged if N .LE. 0) Multiply the 2 x 2 matrix  ( SC SS) times the 2 x N matrix (SX**T) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SC`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. element of rotation matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `SS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. element of rotation matrix SC)                        (SY**T) where **T indicates transpose.  The elements of SX are in not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `SX`: not a workspace argument
- `INCX`: not a workspace argument
- `SY`: not a workspace argument
- `INCY`: not a workspace argument
- `SC`: not a workspace argument
- `SS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srot`
- Original SLATEC routine: `SROT`
- Native symbol: `srot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROT](https://www.netlib.org/slatec/lin/srot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
