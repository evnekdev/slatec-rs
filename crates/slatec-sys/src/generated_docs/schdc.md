# Purpose

SCHDC computes the Cholesky decomposition of a positive definite matrix. A pivoting option allows the user to estimate the condition of a positive definite matrix or determine the rank of a positive semidefinite matrix. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SCHDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SCHDC](https://www.netlib.org/slatec/lin/schdc.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA,P). contains the matrix whose decomposition is to be computed.  Only the upper half of A need be stored. The lower part of the array A is not referenced. is not referenced if contains in its upper half the Cholesky factor contains in its upper half the Cholesky factor of the matrix A as it has been permuted by pivoting. of the matrix A as it has been permuted by pivoting. th position, provided pivoting was requested. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array A. INTEGER. is the leading dimension of the array A. INTEGER. is the leading dimension of the array A. not a workspace argument

## 3. `P`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the order of the matrix. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL. is a work array. not stated by selected source not applicable or not stated by selected source

## 5. `JPVT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(P). contains integers that control the selection of the pivot elements, if pivoting has been requested. Each diagonal element A(K,K) is placed in one of three classes according to the value of JPVT(K). If JPVT(K) .GT. 0, then X(K) is an initial element. If JPVT(K) .EQ. 0, then X(K) is a free element. If JPVT(K) .LT. 0, then X(K) is a final element. Before the decomposition is computed, initial elements are moved by symmetric row and column interchanges to the beginning of the array A and final elements to the end.  Both initial and final elements are frozen in place during the computation and only free elements are moved.  At the K-th stage of the reduction, if A(K,K) is occupied by a free element it is interchanged with the largest free element is not referenced if contains the index of the diagonal element contains the index of the diagonal element not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. .EQ. 0. INTEGER. is an integer that initiates column pivoting. If JOB .EQ. 0, no pivoting is done. If JOB .NE. 0, pivoting is done. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. contains the index of the last positive diagonal element of the Cholesky factor. P is the normal return. For pivoting with positive semidefinite matrices INFO will in general be less than P.  However, INFO may be greater than the rank of A, since rounding error can cause an otherwise zero element to be positive.  Indefinite systems will always cause to be less than P. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `P`: not a workspace argument
- `WORK`: REAL. is a work array.
- `JPVT`: not a workspace argument
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::schdc`
- Original SLATEC routine: `SCHDC`
- Native symbol: `schdc_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SCHDC](https://www.netlib.org/slatec/lin/schdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
