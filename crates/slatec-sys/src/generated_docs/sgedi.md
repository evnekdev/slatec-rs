# Purpose

SGEDI computes the determinant and inverse of a matrix using the factors computed by SGECO or SGEFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SGEDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGEDI](https://www.netlib.org/slatec/lin/sgedi.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the output from SGECO or SGEFA. inverse of original matrix if requested. Otherwise unchanged. division by zero will occur if the input factor contains zero on the diagonal and the inverse is requested. It will not occur if the subroutines are called correctly and if SGECO has set RCOND .GT. 0.0 or SGEFA has set INFO .EQ. 0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from SGECO or SGEFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `DET`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (2). REAL(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with  1.0 .LE. ABS(DET(1)) .LT. 10.0 or  DET(1) .EQ. 0.0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) vector.  Contents destroyed. not stated by selected source not applicable or not stated by selected source

## 7. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 11   both determinant and inverse. = 01   inverse only. = 10   determinant only. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `IPVT`: not a workspace argument
- `DET`: not a workspace argument
- `WORK`: REAL(N) vector.  Contents destroyed.
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgedi`
- Original SLATEC routine: `SGEDI`
- Native symbol: `sgedi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SGEDI](https://www.netlib.org/slatec/lin/sgedi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
