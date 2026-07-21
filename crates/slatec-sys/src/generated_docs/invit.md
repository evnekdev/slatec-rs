# Purpose

This subroutine is a translation of the ALGOL procedure INVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a REAL UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `INVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INVIT](https://www.netlib.org/slatec/lin/invit.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the upper Hessenberg matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. SELECT may have been altered. If the elements corresponding to a pair of conjugate complex eigenvalues were each initially set to.

## `WR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix. The eigenvalues must be stored in a manner identical to that output by subroutine HQR, which recognizes possible splitting of the matrix. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N).

## `WI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix. The eigenvalues must be stored in a manner identical to that output by subroutine HQR, which recognizes possible splitting of the matrix. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. SELECT may have been altered.

## `SELECT`

**Direction:** `input`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** rank 1; dimensions (N).

specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is specified by setting SELECT(J) to. TRUE. SELECT is a one-dimensional LOGICAL array, dimensioned SELECT(N).

## `MM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

should be set to an upper bound for the number of columns required to store the eigenvectors to be found. NOTE that two columns are required to store the eigenvector corresponding to a complex eigenvalue. One column is required to store the eigenvector corresponding to a real eigenvalue. MM is an INTEGER variable.

## `M`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of columns actually used to store the eigenvectors. M is an INTEGER variable.

## `Z`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

contains the real and imaginary parts of the eigenvectors. The eigenvectors are packed into the columns of Z starting at the first column. If the next selected eigenvalue is real, the next column of Z contains its eigenvector. If the eigenvalue is complex, the next two columns of Z contain the real and imaginary parts of its eigenvector, with the real part first. The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails the acceptance test is set to zero.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, -(2*N+1) if more than MM columns of Z are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, -(N+K) if both error situations occur.

## `RM1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (N, *).

is a two-dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. is dimensioned RM1(N,N).

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. They hold the approximate eigenvectors during the inverse iteration process. RV1 and RV2 are dimensioned RV1(N) and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division.

## `RV2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage. They hold the approximate eigenvectors during the inverse iteration process. RV1 and RV2 are dimensioned RV1(N) and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `SELECT`: not a workspace argument
- `Z`: not a workspace argument
- `RM1`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::invit`
- Original SLATEC routine: `INVIT`
- Native symbol: `invit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [INVIT](https://www.netlib.org/slatec/lin/invit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
