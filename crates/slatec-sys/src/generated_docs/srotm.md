# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `SROTM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SROTM](https://www.netlib.org/slatec/lin/srotm.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SX`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements rotated vector (unchanged if N .LE. 0) 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SX 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for SY using LY and INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SY`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements rotated vector (unchanged if N .LE. 0) Apply the modified Givens transformation, H, to the 2 by N matrix , where **T indicates transpose.  The elements of SX are not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `SPARAM`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (5). element vector. SPARAM(1) is SFLAG described below. Locations 2-5 of SPARAM contain elements of the transformation matrix H described below. SFLAG=-1.E0     SFLAG=0.E0        SFLAG=1.E0     SFLAG=-2.E0 (SH11  SH12)    (1.E0  SH12)    (SH11  1.E0)    (1.E0  0.E0) H=(          )    (          )    (          )    (          ) (SH21  SH22),   (SH21  1.E0),   (-1.E0 SH22),   (0.E0  1.E0). See SROTMG for a description of data storage in SPARAM. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `SPARAM`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::srotm`
- Original SLATEC routine: `SROTM`
- Native symbol: `srotm_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SROTM](https://www.netlib.org/slatec/lin/srotm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
