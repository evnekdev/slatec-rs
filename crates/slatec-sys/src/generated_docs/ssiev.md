# Purpose

SSIEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by SSIEV.)

# Description

This canonical unsafe binding exposes original SLATEC routine `SSIEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSIEV](https://www.netlib.org/slatec/src/ssiev.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL (LDA,N) real symmetric input matrix. Only the diagonal and upper triangle of A must be input, as SSIEV copies the upper triangle to the lower. 1,..N, and J=I,. are stored in its first N columns.  See also INFO below. REAL (LDA,N) real symmetric input matrix. Only the diagonal and upper triangle of A must be input, as SSIEV copies the upper triangle to the lower. 1,..N, and J=I,. are stored in its first N columns.  See also INFO below. not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by the user to the leading dimension of the array A. INTEGER set by the user to the leading dimension of the array A. INTEGER set by the user to the leading dimension of the array A. not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. On return from SSIEV, if the user has set JOB = 0        the lower triangle of A has been altered. are stored in its first N columns.  See also INFO below. INTEGER set by the user to the order of the matrix A and the number of elements in E. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `E`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL (N) on return from SSIEV, E contains the N eigenvalues of A.  See also INFO below. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL (2*N) temporary storage vector.  Contents changed by SSIEV. not stated by selected source not applicable or not stated by selected source

## 6. `JOB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER set by user on input = 0         only calculate eigenvalues of A. = nonzero   calculate eigenvalues and eigenvectors of A. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER on return from SSIEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. eigenvalues and vectors 1 through K-1 are correct. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

No. 1   recoverable  N is greater than LDA No. 2   recoverable  N is less than one

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `E`: not a workspace argument
- `WORK`: REAL (2*N) temporary storage vector.  Contents changed by SSIEV.
- `JOB`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ssiev`
- Original SLATEC routine: `SSIEV`
- Native symbol: `ssiev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SSIEV](https://www.netlib.org/slatec/src/ssiev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
