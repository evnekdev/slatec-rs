# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CAXPY`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CAXPY](https://www.netlib.org/slatec/lin/caxpy.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, replace  CY(LY+I*INCY) with CA*CX(LX+I*INCX) + not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CA`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. complex scalar multiplier not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `CX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `CY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements complex result (unchanged if N .LE. 0) Overwrite complex CY with complex  CA*CX + CY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `CA`: not a workspace argument
- `CX`: not a workspace argument
- `INCX`: not a workspace argument
- `CY`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::caxpy`
- Original SLATEC routine: `CAXPY`
- Native symbol: `caxpy_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CAXPY](https://www.netlib.org/slatec/lin/caxpy.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
