# Purpose

Description of Parameters The * Flags Output Variables

# Description

This canonical unsafe binding exposes original SLATEC routine `SCOPYM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCOPYM](https://www.netlib.org/slatec/lin/scopym.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of elements in vector(s) 1, copy  -SX(LX+I*INCX) to SY(LY+I*INCY), where LX=1 if not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SX`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real vector with N elements 1, copy  -SX(LX+I*INCX) to SY(LY+I*INCY), where LX=1 if not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Storage spacing between elements of SX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SY`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real negative copy of SX SX  *** 1, copy  -SX(LX+I*INCX) to SY(LY+I*INCY), where LX=1 if not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Storage spacing between elements of SY not stated by selected source not applicable or not stated by selected source not a workspace argument

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

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::scopym`
- Original SLATEC routine: `SCOPYM`
- Native symbol: `scopym_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SCOPYM](https://www.netlib.org/slatec/lin/scopym.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
