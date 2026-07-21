# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DSDOT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSDOT](https://www.netlib.org/slatec/lin/dsdot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1 of  SX(LX+I*INCX) * SY(LY+I*INCY), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `SX`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `SY`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). single precision vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of SY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

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

- Canonical Rust path: `slatec_sys::blas::level1::dsdot`
- Original SLATEC routine: `DSDOT`
- Native symbol: `dsdot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSDOT](https://www.netlib.org/slatec/lin/dsdot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
