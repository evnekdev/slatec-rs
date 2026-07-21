# Purpose

CGBDI computes the determinant of a band matrix using the factors computed by CGBCO or CGBFA. If the inverse is needed, use CGBSL N times. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CGBDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CGBDI](https://www.netlib.org/slatec/lin/cgbdi.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). COMPLEX(LDA, N) the output from CGBCO or CGBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the original matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ML`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals below the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MU`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER number of diagonals above the main diagonal. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from CGBCO or CGBFA. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `DET`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (2). COMPLEX(2) determinant of original matrix. Determinant = DET(1) * 10.0**DET(2) with  1.0 .LE. CABS1(DET(1)) .LT. 10.0 0.0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `ML`: not a workspace argument
- `MU`: not a workspace argument
- `IPVT`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cgbdi`
- Original SLATEC routine: `CGBDI`
- Native symbol: `cgbdi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1)`
- Exact Netlib source file: [CGBDI](https://www.netlib.org/slatec/lin/cgbdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
