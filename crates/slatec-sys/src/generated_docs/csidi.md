# Purpose

CSIDI computes the determinant and inverse of a complex symmetric matrix using the factors from CSIFA. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSIDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSIDI](https://www.netlib.org/slatec/lin/csidi.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). COMPLEX(LDA,N) the output from CSIFA. contains the upper triangle of the inverse of the original matrix.  The strict lower triangle is never referenced. division by zero may occur if the inverse is requested and  CSICO  has set RCOND .EQ. 0.0 or  CSIFA  has set  INFO .NE. 0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array A . INTEGER the leading dimension of the array A . INTEGER the leading dimension of the array A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from CSIFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `DET`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (2). COMPLEX(2) determinant of original matrix. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 0.0. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `WORK`

workspace `workspace` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) vector.  Contents destroyed. not stated by selected source not applicable or not stated by selected source

## 7. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER has the decimal expansion  AB  where If  B .NE. 0, the inverse is computed, If  A .NE. 0, the determinant is computed, 11  gives both. On Return Variables not requested by JOB are not used. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `KPVT`: not a workspace argument
- `DET`: not a workspace argument
- `WORK`: COMPLEX(N) vector.  Contents destroyed.
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::csidi`
- Original SLATEC routine: `CSIDI`
- Native symbol: `csidi_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CSIDI](https://www.netlib.org/slatec/lin/csidi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
