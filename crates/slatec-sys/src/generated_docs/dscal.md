# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DSCAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSCAL](https://www.netlib.org/slatec/lin/dscal.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1, replace DX(IX+I*INCX) with  DA * DX(IX+I*INCX), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scale factor not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DX`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision vector with N elements double precision result (unchanged if N.LE.0) Replace double precision DX by double precision DA*DX. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of DX N)*INCX. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `DA`: not a workspace argument
- `DX`: not a workspace argument
- `INCX`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dscal`
- Original SLATEC routine: `DSCAL`
- Native symbol: `dscal_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DSCAL](https://www.netlib.org/slatec/lin/dscal.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
