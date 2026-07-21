# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DROT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DROT](https://www.netlib.org/slatec/lin/drot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DX`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision vector with N elements rotated vector DX (unchanged if N .LE. 0) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of DX 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DY`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision vector with N elements rotated vector DY (unchanged if N .LE. 0) Multiply the 2 x 2 matrix  ( DC DS) times the 2 x N matrix (DX**T) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of DY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `DC`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. D.P. element of rotation matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `DS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. D.P. element of rotation matrix DC)                        (DY**T) where **T indicates transpose.  The elements of DX are in not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `DX`: not a workspace argument
- `INCX`: not a workspace argument
- `DY`: not a workspace argument
- `INCY`: not a workspace argument
- `DC`: not a workspace argument
- `DS`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::drot`
- Original SLATEC routine: `DROT`
- Native symbol: `drot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DROT](https://www.netlib.org/slatec/lin/drot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
