# Purpose

This subroutine is a translation of the ALGOL procedure CXINVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP. VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of A COMPLEX UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `CINVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CINVIT](https://www.netlib.org/slatec/lin/cinvit.f).

# Arguments

## `NM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM.

## `AR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors.

## `AI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors.

## `WR`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the matrix. The eigenvalues must be stored in a manner identical to that of subroutine COMLR, which recognizes possible splitting of the matrix. WR and.

## `WI`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

the real and imaginary parts, respectively, of the eigenvalues of the matrix. The eigenvalues must be stored in a manner identical to that of subroutine COMLR, which recognizes possible splitting of the matrix. WR and are one-dimensional REAL arrays, dimensioned WR(N) and unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors.

## `SELECT`

**Direction:** `input-output`. **Fortran type:** `LOGICAL`. **Rust ABI type:** `*mut crate::FortranLogical`. **Shape:** rank 1; dimensions (N).

specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is specified by setting SELECT(J) to. TRUE. SELECT is a one-dimensional LOGICAL array, dimensioned SELECT(N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors.

## `MM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

should be set to an upper bound for the number of eigenvectors to be found. MM is an INTEGER variable.

## `M`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is the number of eigenvectors actually found. M is an INTEGER variable.

## `ZR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails the acceptance test is set to zero. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and.

## `ZI`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (NM, *).

the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails the acceptance test is set to zero. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

is an INTEGER flag set to Zero for normal return, -(2*N+1) if more than MM eigenvectors have been requested (the MM eigenvectors calculated to this point are in ZR and ZI), -K if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of ZR and ZI are set to zero vectors, -(N+K) if both error situations occur.

## `RM1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (N, *).

two-dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B.

## `RM2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (N, *).

two-dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B.

## `RV1`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process.

## `RV2`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

one-dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `SELECT`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `RM1`: not a workspace argument
- `RM2`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cinvit`
- Original SLATEC routine: `CINVIT`
- Native symbol: `cinvit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [CINVIT](https://www.netlib.org/slatec/lin/cinvit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
