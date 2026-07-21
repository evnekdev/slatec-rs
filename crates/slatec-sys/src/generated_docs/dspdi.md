# Purpose

DSPDI computes the determinant, inertia and inverse of a double precision symmetric matrix using the factors from DSPFA, where the matrix is stored in packed form. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DSPDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DSPDI](https://www.netlib.org/slatec/lin/dspdi.f).

# Arguments

## 1. `AP`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION (N*(N+1)/2) the output from DSPFA. contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array. DOUBLE PRECISION (N*(N+1)/2) the output from DSPFA. contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) the pivot vector from DSPFA. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DET`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (2). DOUBLE PRECISION(2) determinant of original matrix. DETERMINANT = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 0.0. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INERT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (3). INTEGER(3) the inertia of the original matrix. number of positive eigenvalues. number of negative eigenvalues. number of zero eigenvalues. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(N) vector.  Contents ignored. not stated by selected source not applicable or not stated by selected source

## 7. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER has the decimal expansion  ABC  where if  C .NE. 0, the inverse is computed, if  B .NE. 0, the determinant is computed, if  A .NE. 0, the inertia is computed. 111  gives all three. On Return Variables not requested by JOB are not used. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

A division by zero will occur if the inverse is requested and  DSPCO  has set RCOND .EQ. 0.0 or  DSPFA  has set  INFO .NE. 0 .

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `DET`: not a workspace argument
- `INERT`: not a workspace argument
- `WORK`: DOUBLE PRECISION(N) vector.  Contents ignored.
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::dspdi`
- Original SLATEC routine: `DSPDI`
- Native symbol: `dspdi_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DSPDI](https://www.netlib.org/slatec/lin/dspdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
