# Purpose

STRDI computes the determinant and inverse of a real triangular matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `STRDI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [STRDI](https://www.netlib.org/slatec/lin/strdi.f).

# Arguments

## 1. `T`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDT, *). REAL(LDT,N) contains the triangular matrix.  The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. inverse of original matrix if requested. Otherwise unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. INTEGER is the leading dimension of the array T. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER is the order of the system. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `DET`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (2). REAL(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with  1.0 .LE. ABS(DET(1)) .LT. 10.0 or  DET(1) .EQ. 0.0 . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 010       no det, inverse of lower triangular. = 011       no det, inverse of upper triangular. = 100       det, no inverse. = 110       det, inverse of lower triangular. = 111       det, inverse of upper triangular. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER contains zero if the system is nonsingular and the inverse is requested. Otherwise INFO contains the index of a zero diagonal element of T. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `T`: not a workspace argument
- `LDT`: not a workspace argument
- `N`: not a workspace argument
- `DET`: not a workspace argument
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::strdi`
- Original SLATEC routine: `STRDI`
- Native symbol: `strdi_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [STRDI](https://www.netlib.org/slatec/lin/strdi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
