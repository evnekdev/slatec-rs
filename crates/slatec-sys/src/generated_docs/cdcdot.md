# Purpose

B L A S Subprogram Description of Parameters

# Description

This canonical unsafe binding exposes original SLATEC routine `CDCDOT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CDCDOT](https://www.netlib.org/slatec/lin/cdcdot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. number of elements in input vector(s) 1 of CX(LX+I*INCY)*CY(LY+I*INCY) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `CB`

input `scalar` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and scalar. complex scalar to be added to inner product 1 of CX(LX+I*INCY)*CY(LY+I*INCY) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `CX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CX N)*INCX, and LY is defined in a similar way using INCY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `CY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). complex vector with N elements not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. storage spacing between elements of CY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:complex32(mut_i32,mut_complex32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `CB`: not a workspace argument
- `CX`: not a workspace argument
- `INCX`: not a workspace argument
- `CY`: not a workspace argument
- `INCY`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::cdcdot`
- Original SLATEC routine: `CDCDOT`
- Native symbol: `cdcdot_`
- ABI fingerprint: `function:complex32(mut_i32,mut_complex32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CDCDOT](https://www.netlib.org/slatec/lin/cdcdot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
