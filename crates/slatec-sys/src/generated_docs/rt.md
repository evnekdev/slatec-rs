# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a special REAL TRIDIAGONAL matrix. The property of the matrix required for use of this subroutine is that the products of pairs of corresponding off-diagonal elements be all non-negative. If eigenvectors are desired, no product can be zero unless both factors are zero.

# Description

This canonical unsafe binding exposes original SLATEC routine `RT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RT](https://www.netlib.org/slatec/lin/rt.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, the diagonal elements in the second column, and the superdiagonal elements in the 1 positions of the third column.  Elements A(1,1) and dimensional REAL array, dimensioned A(NM,3). is greater than NM, is greater than NM, 1,3) is negative, zero, is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. 1 positions of the first column, the diagonal elements in the second column, and the superdiagonal elements in the 1 positions of the third column.  Elements A(1,1) and dimensional REAL array, dimensioned A(NM,3). is greater than NM, is greater than NM, 1,3) is negative, zero, not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, 3). contains the special real tridiagonal matrix in its first three columns.  The subdiagonal elements are stored in the dimensional REAL array, dimensional REAL array, dimensional REAL array, dimensioned A(NM,3). dimensioned A(NM,3). dimensioned A(NM,3). dimensional REAL array, dimensioned Z(NM,N). 1,3) is negative, 1,3) is negative, dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the special real tridiagonal matrix in its first three columns.  The subdiagonal elements are stored in the dimensional REAL array, dimensional REAL array, dimensional REAL array, dimensioned A(NM,3). dimensioned A(NM,3). dimensioned A(NM,3). dimensional REAL array, dimensioned Z(NM,N). 1,3) is negative, 1,3) is negative, dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is a is a one-dimensional REAL array, dimensioned W(N). one-dimensional REAL array, dimensioned W(N). is a is a one-dimensional REAL array, dimensioned W(N). one-dimensional REAL array, dimensioned W(N). not applicable or not stated by selected source

## 5. `MATZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable set equal to zero if only eigenvalues are desired.  Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. zero; J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional REAL array, dimensioned Z(NM,N). dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 7. `FV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `W`: is a is a one-dimensional REAL array, dimensioned W(N). one-dimensional REAL array, dimensioned W(N).
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument
- `FV1`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rt`
- Original SLATEC routine: `RT`
- Native symbol: `rt_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [RT](https://www.netlib.org/slatec/lin/rt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
