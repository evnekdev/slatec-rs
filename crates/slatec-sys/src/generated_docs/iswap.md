# Purpose

Extended B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `ISWAP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ISWAP](https://www.netlib.org/slatec/lin/iswap.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, interchange  IX(LX+I*INCX) and IY(LY+I*INCY), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `IX`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). integer vector with N elements input vector IY (unchanged if N .LE. 0) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of IX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IY`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). integer vector with N elements input vector IX (unchanged if N .LE. 0) Interchange integer IX and integer IY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of IY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `IX`: not a workspace argument
- `INCX`: not a workspace argument
- `IY`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::iswap`
- Original SLATEC routine: `ISWAP`
- Native symbol: `iswap_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [ISWAP](https://www.netlib.org/slatec/lin/iswap.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
