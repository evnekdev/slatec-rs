# Purpose

This subroutine is a translation of the ALGOL procedure CXINVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP. VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of A COMPLEX UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `CINVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CINVIT](https://www.netlib.org/slatec/lin/cinvit.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. if more than MM eigenvectors have been requested (the MM eigenvectors calculated to this point are in ZR and ZI), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the if both error situations occur. (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. if more than MM eigenvectors have been requested (the MM eigenvectors calculated to this point are in ZR and ZI), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the if both error situations occur. not applicable or not stated by selected source not a workspace argument

## 3. `AR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 4. `AI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 5. `WR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. not applicable or not stated by selected source not a workspace argument

## 6. `WI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and dimensional REAL arrays, dimensioned WR(N) and are unaltered. contain the real and imaginary parts, respectively, of the eigenvalues of the matrix.  The eigenvalues must be stored in a manner identical to that of subroutine  COMLR, which recognizes possible splitting of the matrix.  WR and dimensional REAL arrays, dimensioned WR(N) and are unaltered. not applicable or not stated by selected source not a workspace argument

## 7. `SELECT`

input `array` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and rank 1; dimensions (N). specifies the eigenvectors to be found.  The eigenvector corresponding to the J-th eigenvalue is is a is a one-dimensional LOGICAL array, dimensioned SELECT(N). one-dimensional LOGICAL array, dimensioned SELECT(N). are unaltered. specifies the eigenvectors to be found.  The eigenvector corresponding to the J-th eigenvalue is is a is a one-dimensional LOGICAL array, dimensioned SELECT(N). one-dimensional LOGICAL array, dimensioned SELECT(N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 8. `MM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. should be set to an upper bound for the number of eigenvectors to be found.  MM is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `M`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvectors actually found.  M is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `ZR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, not applicable or not stated by selected source not a workspace argument

## 11. `ZI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, contain the real and imaginary parts, respectively, of the eigenvectors corresponding to the flagged eigenvalues. The eigenvectors are normalized so that the component of largest magnitude is 1.  Any vector which fails the acceptance test is set to zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,MM) and are set to zero vectors, not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `RM1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (N, *). dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 14. `RM2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (N, *). dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned RM1(N,N) and RM2(N,N). These arrays hold the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. The ALGOL procedure GUESSVEC appears in CINVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 15. `RV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. not applicable or not stated by selected source not a workspace argument

## 16. `RV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. dimensional REAL arrays used for temporary storage, dimensioned RV1(N) and RV2(N). They hold the approximate eigenvectors during the inverse iteration process. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `SELECT`: not a workspace argument
- `MM`: not a workspace argument
- `M`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `IERR`: not a workspace argument
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
