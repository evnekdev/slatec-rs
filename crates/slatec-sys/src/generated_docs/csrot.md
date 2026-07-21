# Purpose

CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y)

# Description

This canonical unsafe binding exposes original SLATEC routine `CSROT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSROT](https://www.netlib.org/slatec/lin/csrot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1 1 X = CX(LX+I*INCX) X = CX(LX+I*INCX) Y = CY(LY+I*INCY), Y = CY(LY+I*INCY), (integer)  number of elements in each vector not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CX`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). (complex array)  beginning of one vector not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. N)*INCX, and LY is defined in a similar way using INCY. Argument Description (integer)  memory spacing of successive elements of vector CX not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `CY`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). (complex array)  beginning of the other vector not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (integer)  memory spacing of successive elements of vector CY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (real)  cosine term of the rotation not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `S`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. (real)  sine term of the rotation. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `CX`: not a workspace argument
- `INCX`: not a workspace argument
- `CY`: not a workspace argument
- `INCY`: not a workspace argument
- `C`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::csrot`
- Original SLATEC routine: `CSROT`
- Native symbol: `csrot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CSROT](https://www.netlib.org/slatec/lin/csrot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
