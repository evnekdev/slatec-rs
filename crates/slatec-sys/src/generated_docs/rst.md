# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a REAL SYMMETRIC TRIDIAGONAL matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `RST`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RST](https://www.netlib.org/slatec/lin/rst.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is arbitrary.  E is a one-dimensional REAL array, dimensioned E(N). is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after 30 iterations. determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices arrays should be correct for indices is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is arbitrary.  E is a one-dimensional REAL array, dimensioned E(N). is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after 30 iterations. determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices arrays should be correct for indices not applicable or not stated by selected source not a workspace argument

## 3. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the real symmetric dimensional REAL array, dimensioned W(N). contains the eigenvalues in ascending order. contains the diagonal elements of the real symmetric dimensional REAL array, dimensioned W(N). contains the eigenvalues in ascending order. not applicable or not stated by selected source

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the matrix in its last not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `MATZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable set equal to zero if only eigenvalues are desired.  Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional REAL array, dimensioned Z(NM,N). dimensional REAL array, dimensioned Z(NM,N). not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, 1. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `W`: contains the diagonal elements of the real symmetric dimensional REAL array, dimensioned W(N). contains the eigenvalues in ascending order.
- `E`: not a workspace argument
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rst`
- Original SLATEC routine: `RST`
- Native symbol: `rst_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [RST](https://www.netlib.org/slatec/lin/rst.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
