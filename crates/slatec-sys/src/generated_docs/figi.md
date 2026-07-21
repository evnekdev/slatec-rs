# Purpose

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, this subroutine reduces it to a symmetric tridiagonal matrix with the same eigenvalues. If, further, a zero product only occurs when both factors are zero, the reduced matrix is similar to the original matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `FIGI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [FIGI](https://www.netlib.org/slatec/lin/figi.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, T, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, T, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix T.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, its diagonal in the N positions of the second column, 1 positions of are arbitrary. 1 positions.  E(1) is not set. 1,3) is negative and a symmetric matrix cannot be produced with FIGI, is zero with one factor non-zero.  In this case, the eigenvectors of the symmetric matrix are not simply related to those of  T  and should not be sought. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix T.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, its diagonal in the N positions of the second column, 1 positions of are arbitrary. 1 positions.  E(1) is not set. 1,3) is negative and a symmetric matrix cannot be produced with FIGI, is zero with one factor non-zero.  In this case, the eigenvectors of the symmetric matrix are not simply related to those of  T  and should not be sought. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `T`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, 3). contains the nonsymmetric matrix.  Its subdiagonal is are arbitrary. are arbitrary. dimensional REAL array, dimensioned T(NM,3). is unaltered. 1,3) is negative and a symmetric 1,3) is negative and a symmetric matrix cannot be produced with FIGI, matrix cannot be produced with FIGI, is zero with one factor is zero with one factor non-zero.  In this case, the eigenvectors of non-zero.  In this case, the eigenvectors of the symmetric matrix are not simply related the symmetric matrix are not simply related to those of  T  and should not be sought. to those of  T  and should not be sought. Questions and comments should be directed to B. S. Garbow, Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the nonsymmetric matrix.  Its subdiagonal is are arbitrary. are arbitrary. dimensional REAL array, dimensioned T(NM,3). is unaltered. 1,3) is negative and a symmetric 1,3) is negative and a symmetric matrix cannot be produced with FIGI, matrix cannot be produced with FIGI, is zero with one factor is zero with one factor non-zero.  In this case, the eigenvectors of non-zero.  In this case, the eigenvectors of the symmetric matrix are not simply related the symmetric matrix are not simply related to those of  T  and should not be sought. to those of  T  and should not be sought. Questions and comments should be directed to B. S. Garbow, Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `D`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the tridiagonal symmetric dimensional REAL array, dimensioned D(N). contains the diagonal elements of the tridiagonal symmetric dimensional REAL array, dimensioned D(N). not applicable or not stated by selected source not a workspace argument

## 5. `E`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the tridiagonal dimensional REAL array, dimensioned E(N). are not needed. contains the subdiagonal elements of the tridiagonal dimensional REAL array, dimensioned E(N). are not needed. not applicable or not stated by selected source not a workspace argument

## 6. `E2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E. are not needed. dimensional REAL array, dimensioned E2(N). contains the squares of the corresponding elements of E. are not needed. dimensional REAL array, dimensioned E2(N). not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `T`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi`
- Original SLATEC routine: `FIGI`
- Native symbol: `figi_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [FIGI](https://www.netlib.org/slatec/lin/figi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
