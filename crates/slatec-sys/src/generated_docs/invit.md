# Purpose

This subroutine is a translation of the ALGOL procedure INVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a REAL UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `INVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [INVIT](https://www.netlib.org/slatec/lin/invit.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, if both error situations occur. and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, if both error situations occur. and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). is an INTEGER variable. are unaltered. dimensional REAL array, dimensioned Z(NM,MM). dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). is an INTEGER variable. are unaltered. dimensional REAL array, dimensioned Z(NM,MM). dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. not applicable or not stated by selected source not a workspace argument

## 4. `WR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix.  The eigenvalues must be stored in a manner identical to that output by subroutine  HQR,  which recognizes possible splitting of the dimensional REAL arrays, dimensioned WR(N) and WI(N). may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. contain the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix.  The eigenvalues must be stored in a manner identical to that output by subroutine  HQR,  which recognizes possible splitting of the dimensional REAL arrays, dimensioned WR(N) and WI(N). may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. not applicable or not stated by selected source not a workspace argument

## 5. `WI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix.  The eigenvalues must be stored in a manner identical to that output by subroutine  HQR,  which recognizes possible splitting of the dimensional REAL arrays, dimensioned WR(N) and WI(N). are unaltered. contain the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix.  The eigenvalues must be stored in a manner identical to that output by subroutine  HQR,  which recognizes possible splitting of the dimensional REAL arrays, dimensioned WR(N) and WI(N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 6. `SELECT`

input `array` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and rank 1; dimensions (N). specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is is a is a one-dimensional LOGICAL array, dimensioned SELECT(N). one-dimensional LOGICAL array, dimensioned SELECT(N). may have been altered.  If the elements corresponding to a pair of conjugate complex eigenvalues were each initially set to .TRUE., the program resets the second of the two elements to .FALSE. specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is is a is a one-dimensional LOGICAL array, dimensioned SELECT(N). one-dimensional LOGICAL array, dimensioned SELECT(N). may have been altered.  If the elements corresponding to a pair of conjugate complex eigenvalues were each initially set to .TRUE., the program resets the second of the two elements to .FALSE. not applicable or not stated by selected source not a workspace argument

## 7. `MM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. should be set to an upper bound for the number of columns required to store the eigenvectors to be found. NOTE that two columns are required to store the eigenvector corresponding to a complex eigenvalue.  One column is required to store the eigenvector corresponding is an INTEGER variable. are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `M`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of columns actually used to store the eigenvectors.  M is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the real and imaginary parts of the eigenvectors. The eigenvectors are packed into the columns of Z starting at the first column.  If the next selected eigenvalue is real, the next column of Z contains its eigenvector.  If the eigenvalue is complex, the next two columns of Z contain the real and imaginary parts of its eigenvector, with the real part first.  The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails dimensional REAL array, dimensioned Z(NM,MM). are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, contains the real and imaginary parts of the eigenvectors. The eigenvectors are packed into the columns of Z starting at the first column.  If the next selected eigenvalue is real, the next column of Z contains its eigenvector.  If the eigenvalue is complex, the next two columns of Z contain the real and imaginary parts of its eigenvector, with the real part first.  The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails dimensional REAL array, dimensioned Z(NM,MM). are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K         if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, not applicable or not stated by selected source not a workspace argument

## 10. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `RM1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (N, *). dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. is dimensioned RM1(N,N). dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. is dimensioned RM1(N,N). not applicable or not stated by selected source not a workspace argument

## 12. `RV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  They hold the approximate eigenvectors during the are dimensioned and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage.  They hold the approximate eigenvectors during the are dimensioned and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 13. `RV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  They hold the approximate eigenvectors during the are dimensioned dimensional REAL arrays used for temporary storage.  They hold the approximate eigenvectors during the are dimensioned not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `SELECT`: not a workspace argument
- `MM`: not a workspace argument
- `M`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument
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
