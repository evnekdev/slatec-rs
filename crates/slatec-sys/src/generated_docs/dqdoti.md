# Purpose

B L A S Subprogram Description of parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `DQDOTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQDOTI](https://www.netlib.org/slatec/src/dqdoti.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1 of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `DB`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. double precision scalar to be added to inner product 1 of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `QC`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (30). extended precision scalar to be added extended precision result D.P. dot product with extended precision accumulation (and result) 1 of is an extended precision result which can be used as input to DQDOTA, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DX`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision vector with N elements is an extended precision result which can be used as input to DQDOTA, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of DX is an extended precision result which can be used as input to DQDOTA, INCX)*N, and LY is defined in a similar way using INCY.  The MP package by Richard P. Brent is used for the extended precision arithmetic. Fred T. Krogh,  JPL,  1977,  June 1 The common block for the MP package is named MPCOM.  If local variable I1 is zero, DQDOTI calls MPBLAS to initialize the MP package and reset I1 to 1. The argument QC(*), and the local variables QX and QY are INTEGER arrays of size 30.  See the comments in the routine MPBLAS for the reason for this choice. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `DY`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). double precision vector with N elements is an extended precision result which can be used as input to DQDOTA, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of DY is an extended precision result which can be used as input to DQDOTA, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `unavailable`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `DB`: not a workspace argument
- `QC`: not a workspace argument
- `DX`: not a workspace argument
- `INCX`: not a workspace argument
- `DY`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dqdoti`
- Original SLATEC routine: `DQDOTI`
- Native symbol: `dqdoti_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQDOTI](https://www.netlib.org/slatec/src/dqdoti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
