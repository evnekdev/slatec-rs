# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CSWAP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSWAP](https://www.netlib.org/slatec/lin/cswap.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, interchange  CX(LX+I*INCX) and CY(LY+I*INCY), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements input vector CY (unchanged if N .LE. 0) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `CY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements input vector CX (unchanged if N .LE. 0) Interchange complex CX and complex CY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CY not stated by selected source not applicable or not stated by selected source not a workspace argument

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

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::cswap`
- Original SLATEC routine: `CSWAP`
- Native symbol: `cswap_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CSWAP](https://www.netlib.org/slatec/lin/cswap.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
