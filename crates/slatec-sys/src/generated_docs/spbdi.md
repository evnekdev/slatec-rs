# Purpose

SPBDI computes the determinant of a real symmetric positive definite band matrix using the factors computed by SPBCO or SPBFA. If the inverse is needed, use SPBSL N times. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPBDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPBDI](https://www.netlib.org/slatec/lin/spbdi.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the output from SPBCO or SPBFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . INTEGER the leading dimension of the array  ABD . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the number of diagonals above the main diagonal. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `DET`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (2). REAL(2) determinant of original matrix in the form Determinant = DET(1) * 10.0**DET(2) with  1.0 .LE. DET(1) .LT. 10.0 or  DET(1) .EQ. 0.0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `M`: not a workspace argument
- `DET`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::spbdi`
- Original SLATEC routine: `SPBDI`
- Native symbol: `spbdi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SPBDI](https://www.netlib.org/slatec/lin/spbdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
