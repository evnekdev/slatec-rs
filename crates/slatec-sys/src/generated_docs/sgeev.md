# Purpose

SGEEV computes the eigenvalues and, optionally, the eigenvectors of a general real matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by SGEEV.)

# Description

This canonical unsafe binding exposes original SLATEC routine `SGEEV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGEEV](https://www.netlib.org/slatec/src/sgeev.f).

# Arguments

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(LDA,N) real nonsymmetric input matrix.

## `LDA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to the leading dimension of the real array A.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to the order of the matrices A and V, and the number of elements in E.

## `E`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) on return from SGEEV, E contains the eigenvalues of A. See also INFO below.

## `V`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

COMPLEX(LDV,N) on return from SGEEV, if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. (Note that if the input matrix A is nearly degenerate, V may be badly conditioned, i. e. , may have nearly dependent columns.

## `LDV`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case, N must be. LE. LDV. If JOB is set to zero, LDV is not referenced.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

REAL(2N) temporary storage vector. Contents changed by SGEEV.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER set by the user to = 0 eigenvalues only to be calculated by SGEEV. Neither V nor LDV is referenced. = nonzero eigenvalues and vectors to be calculated. In this case, A & V must be distinct arrays. Also, if LDA. GT.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER on return from SGEEV the value of INFO is = 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one. No. 3 recoverable JOB is nonzero and N is greater than LDV No. 4 warning LDA > LDV, elements of A other than the N by N input elements have been changed. No. 5 warning LDA < LDV, elements of V other than the N x N output elements have been changed. |

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `E`: not a workspace argument
- `V`: not a workspace argument
- `LDV`: not a workspace argument
- `WORK`: REAL(2N) temporary storage vector. Contents changed by SGEEV.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::sgeev`
- Original SLATEC routine: `SGEEV`
- Native symbol: `sgeev_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [SGEEV](https://www.netlib.org/slatec/src/sgeev.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
