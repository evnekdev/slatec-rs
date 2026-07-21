# Purpose

DPPDI computes the determinant and inverse of a double precision symmetric positive definite matrix using the factors computed by DPPCO or DPPFA . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DPPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPPDI](https://www.netlib.org/slatec/lin/dppdi.f).

# Arguments

## 1. `AP`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION (N*(N+1)/2) the output from DPPCO or DPPFA. the upper triangular half of the inverse . The strict lower triangle is unaltered. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `DET`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (2). DOUBLE PRECISION(2) determinant of original matrix if requested. Otherwise not referenced. DETERMINANT = DET(1) * 10.0**DET(2) with  1.0 .LE. DET(1) .LT. 10.0 or  DET(1) .EQ. 0.0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 11   both determinant and inverse. = 01   inverse only. = 10   determinant only. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the input factor contains a zero on the diagonal and the inverse is requested. It will not occur if the subroutines are called correctly and if DPOCO or DPOFA has set INFO .EQ. 0 .

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `DET`: not a workspace argument
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::dppdi`
- Original SLATEC routine: `DPPDI`
- Native symbol: `dppdi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DPPDI](https://www.netlib.org/slatec/lin/dppdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
