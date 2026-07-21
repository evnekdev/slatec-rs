# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a REAL SYMMETRIC BAND matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `RSB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RSB](https://www.netlib.org/slatec/lin/rsb.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. is an INTEGER variable. MB  positions of the first column, its next subdiagonal MB  positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the  N  positions of the last column.  Contents of storage locations not part of the matrix are arbitrary.  A is a two-dimensional REAL array, dimensioned A(NM,MB). is greater than NM, is greater than NM, positive or greater than N, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors, if requested, is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. is an INTEGER variable. MB  positions of the first column, its next subdiagonal MB  positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the  N  positions of the last column.  Contents of storage locations not part of the matrix are arbitrary.  A is a two-dimensional REAL array, dimensioned A(NM,MB). is greater than NM, is greater than NM, positive or greater than N, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors, if requested, not applicable or not stated by selected source not a workspace argument

## 3. `MB`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the half band width of the matrix, defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix.  MB must be less than or is an INTEGER variable. positive or greater than N, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors, if requested, is the half band width of the matrix, defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix.  MB must be less than or is an INTEGER variable. positive or greater than N, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors, if requested, not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the lower triangle of the real symmetric band matrix.  Its lowest subdiagonal is stored in the last has been destroyed. dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned Z(NM,N). contains the lower triangle of the real symmetric band matrix.  Its lowest subdiagonal is stored in the last has been destroyed. dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 5. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned W(N). not applicable or not stated by selected source

## 6. `MATZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable set equal to zero if only eigenvalues are desired.  Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. is not zero.  The not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). is not zero.  The dimensional REAL array, dimensioned Z(NM,N). is not zero.  The dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 8. `FV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 9. `FV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 10. `IERR`

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
- `MB`: not a workspace argument
- `A`: not a workspace argument
- `W`: dimensional REAL array, dimensioned W(N). dimensional REAL array, dimensioned W(N).
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument
- `FV1`: not a workspace argument
- `FV2`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rsb`
- Original SLATEC routine: `RSB`
- Native symbol: `rsb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [RSB](https://www.netlib.org/slatec/lin/rsb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
