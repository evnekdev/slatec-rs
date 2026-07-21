# Purpose

SSIEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by SSIEV.)

# Description

This canonical unsafe binding exposes original SLATEC routine `SSIEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSIEV](https://www.netlib.org/slatec/src/ssiev.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (LDA, *).

REAL (LDA,N) real symmetric input matrix. Only the diagonal and upper triangle of A must be input, as SSIEV copies the upper triangle to the lower. That is, the user must define A(I,J), I=1,. N, and J=I,. ,N. On return from SSIEV, if the user has set JOB = 0 the lower triangle of A has been altered.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to the leading dimension of the array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to the order of the matrix A and the number of elements in E.

## `E`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL (N) on return from SSIEV, E contains the N eigenvalues of A. See also INFO below.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL (2*N) temporary storage vector. Contents changed by SSIEV.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by user on input = 0 only calculate eigenvalues of A. = nonzero calculate eigenvalues and eigenvectors of A.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER on return from SSIEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. eigenvalues and vectors 1 through K-1 are correct.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 for normal return. = K if the eigenvalue iteration fails to converge. eigenvalues and vectors 1 through K-1 are correct. No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `E`: not a workspace argument
- `WORK`: REAL (2*N) temporary storage vector. Contents changed by SSIEV.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ssiev`
- Original SLATEC routine: `SSIEV`
- Native symbol: `ssiev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SSIEV](https://www.netlib.org/slatec/src/ssiev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
