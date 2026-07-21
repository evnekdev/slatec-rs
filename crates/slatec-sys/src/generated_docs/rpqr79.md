# Purpose

This routine computes all zeros of a polynomial of degree NDEG with real coefficients by computing the eigenvalues of the companion matrix. Description of Parameters The user must dimension all arrays appearing in the call list

# Description

This canonical unsafe binding exposes original SLATEC routine `RPQR79`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RPQR79](https://www.netlib.org/slatec/src/rpqr79.f).

# Arguments

## 1. `NDEG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. ROOT(NDEG), WORK(NDEG*(NDEG+2)) degree of polynomial not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `COEFF`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). ROOT(NDEG), WORK(NDEG*(NDEG+2)) REAL coefficients in descending order.  i.e., P(Z)= COEFF(1)*(Z**NDEG) + COEFF(NDEG)*Z + COEFF(NDEG+1) 0.0 3  NDEG is invalid (less than or equal to 0) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `ROOT`

output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX vector of roots not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Output Error Code - Normal Code 0  means the roots were computed. - Abnormal Codes 1  more than 30 QR iterations on some eigenvalue of the companion matrix not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL work array of dimension at least NDEG*(NDEG+2) REAL work array of dimension at least NDEG*(NDEG+2) not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NDEG`: not a workspace argument
- `COEFF`: not a workspace argument
- `ROOT`: not a workspace argument
- `IERR`: not a workspace argument
- `WORK`: REAL work array of dimension at least NDEG*(NDEG+2)

# ABI notes

- Canonical Rust path: `slatec_sys::roots::complex::rpqr79`
- Original SLATEC routine: `RPQR79`
- Native symbol: `rpqr79_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [RPQR79](https://www.netlib.org/slatec/src/rpqr79.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
