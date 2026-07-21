# Purpose

DQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. Column pivoting based on the 2-norms of the reduced columns may be performed at the user's option. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `DQRDC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQRDC](https://www.netlib.org/slatec/lin/dqrdc.f).

# Arguments

## 1. `X`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 2; dimensions (LDX, *). DOUBLE PRECISION(LDX,P), where LDX .GE. N. contains the matrix whose decomposition is to be computed. is an initial column. is a free column. is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end.  Both initial and final columns are frozen in place during the computation and only free columns are moved.  At the K-th stage of the reduction, if X(K) is occupied by a free column it is interchanged with the free column of largest reduced norm.  JPVT is not referenced if contains in its upper triangle the upper contains in its upper triangle the upper triangular matrix R of the QR factorization. triangular matrix R of the QR factorization. Below its diagonal X contains information from Below its diagonal X contains information from which the orthogonal part of the decomposition which the orthogonal part of the decomposition can be recovered.  Note that if pivoting has can be recovered.  Note that if pivoting has been requested, the decomposition is not that been requested, the decomposition is not that of the original matrix X but that of X of the original matrix X but that of X with its columns permuted as described by JPVT. with its columns permuted as described by JPVT. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. INTEGER. is the leading dimension of the array X. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a users option. INTEGER. is the number of rows of the matrix X. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `P`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is a users option. INTEGER. is the number of columns of the matrix X. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `QRAUX`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(P). contains further information required to recover the orthogonal part of the decomposition. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JPVT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(P). contains integers that control the selection of the pivot columns.  The K-th column X(K) of X is placed in one of three classes according to the value of JPVT(K). is an initial column. is a free column. is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end.  Both initial and final columns are frozen in place during the computation and only free columns are moved.  At the K-th stage of the reduction, if X(K) is occupied by a free column it is interchanged with the free column of largest reduced norm.  JPVT is not referenced if contains the index of the column of the contains the index of the column of the original matrix that has been interchanged into original matrix that has been interchanged into the K-th column, if pivoting was requested. the K-th column, if pivoting was requested. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). DOUBLE PRECISION(P). is a work array.  WORK is not referenced if not stated by selected source not applicable or not stated by selected source

## 8. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. .EQ. 0. .EQ. 0. INTEGER. is an integer that initiates column pivoting. If JOB .EQ. 0, no pivoting is done. If JOB .NE. 0, pivoting is done. On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `N`: not a workspace argument
- `P`: not a workspace argument
- `QRAUX`: not a workspace argument
- `JPVT`: not a workspace argument
- `WORK`: DOUBLE PRECISION(P). is a work array.  WORK is not referenced if
- `JOB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::dqrdc`
- Original SLATEC routine: `DQRDC`
- Native symbol: `dqrdc_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`
- Exact Netlib source file: [DQRDC](https://www.netlib.org/slatec/lin/dqrdc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
