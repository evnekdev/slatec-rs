# Purpose

CGTSL given a general tridiagonal matrix and a right hand side will find the solution. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGTSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGTSL](https://www.netlib.org/slatec/lin/cgtsl.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the order of the tridiagonal matrix. contain the subdiagonal. On output C is destroyed. 1) should contain the superdiagonal. On output E is destroyed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `C`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) is the subdiagonal of the tridiagonal matrix. contain the subdiagonal. contain the subdiagonal. On output C is destroyed. On output C is destroyed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `D`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) is the diagonal of the tridiagonal matrix. On output D is destroyed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `E`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) is the superdiagonal of the tridiagonal matrix. 1) should contain the superdiagonal. 1) should contain the superdiagonal. On output E is destroyed. On output E is destroyed. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `B`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) is the right hand side vector. On Return is the solution vector. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0 normal value. = K if the K-th element of the diagonal becomes exactly zero.  The subroutine returns when this is detected. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `C`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `B`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cgtsl`
- Original SLATEC routine: `CGTSL`
- Native symbol: `cgtsl_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CGTSL](https://www.netlib.org/slatec/lin/cgtsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
