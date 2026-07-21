# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) To find the eigenvalues and eigenvectors (if desired) of a REAL GENERAL matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `RG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RG](https://www.netlib.org/slatec/lin/rg.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after a total of 30 iterations. determined after a total of 30 iterations. The eigenvalues should be correct for indices The eigenvalues should be correct for indices is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after a total of 30 iterations. determined after a total of 30 iterations. The eigenvalues should be correct for indices The eigenvalues should be correct for indices is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). has been destroyed. dimensional REAL array, dimensioned Z(NM,N). dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). has been destroyed. dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 4. `WR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first.  If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N.  WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first.  If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N.  WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 5. `WI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first.  If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N.  WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first.  If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N.  WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 6. `MATZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable set equal to zero if only eigenvalues are desired.  Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. th eigenvalue is real, the J-th column of Z contains its eigenvector.  If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector.  The conjugate of this vector is the eigenvector for the conjugate eigenvalue. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the real and imaginary parts of the eigenvectors dimensional REAL array, dimensioned Z(NM,N). contains the real and imaginary parts of the eigenvectors dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 8. `IV1`

output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). dimensional temporary storage arrays of is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional temporary storage arrays of is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 9. `FV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional temporary storage arrays of dimensional temporary storage arrays of not applicable or not stated by selected source not a workspace argument

## 10. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, IERR+2, ..., N, but no eigenvectors are computed. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument
- `IV1`: not a workspace argument
- `FV1`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rg`
- Original SLATEC routine: `RG`
- Native symbol: `rg_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [RG](https://www.netlib.org/slatec/lin/rg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
