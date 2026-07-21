# Purpose

Compute the dot product of 2 complex vectors, CX and CY, e.g.

# Description

This canonical unsafe binding exposes original SLATEC routine `DCDOT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCDOT](https://www.netlib.org/slatec/lin/dcdot.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Number of complex components of CX and CY. Complex arrays of length N. Number of complex components of CX and CY. Complex arrays of length N. not applicable or not stated by selected source not a workspace argument

## 2. `FM`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. =+1.0   compute CX DOT CY. =-1.0   compute CXconjugate DOT CY. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `CX`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). DOT CY, or, CXconjugate DOT CY.  The real and imaginary are converted to double precision, the dot product accumulation is done in double precision and the output is given as 2 double precision numbers, corresponding to the real and imaginary part of the result. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INCX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (Integer)   Spacing of elements of CX to use not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `CY`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). are converted to double precision, the dot product accumulation is done in double precision and the output is given as 2 double precision numbers, corresponding to the real and imaginary part of the result. Complex arrays of length N. are converted to double precision, the dot product accumulation is done in double precision and the output is given as 2 double precision numbers, corresponding to the real and imaginary part of the result. Complex arrays of length N. not applicable or not stated by selected source not a workspace argument

## 6. `INCY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (Integer)   Spacing of elements of CY to use. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `DCR`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. (Double Precision) Real part of dot product. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `DCI`

output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. (Double Precision) Imaginary part of dot product. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `FM`: not a workspace argument
- `CX`: not a workspace argument
- `INCX`: not a workspace argument
- `CY`: not a workspace argument
- `INCY`: not a workspace argument
- `DCR`: not a workspace argument
- `DCI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::blas::level1::dcdot`
- Original SLATEC routine: `DCDOT`
- Native symbol: `dcdot_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DCDOT](https://www.netlib.org/slatec/lin/dcdot.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
