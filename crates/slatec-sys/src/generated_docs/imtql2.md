# Purpose

This subroutine is a translation of the ALGOL procedure IMTQL2, NUM. MATH. 12, 377-383(1968) by Martin and Wilkinson, as modified in NUM. MATH. 15, 450(1970) by Dubrulle. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 241-248(1971). This subroutine finds the eigenvalues and eigenvectors of a SYMMETRIC TRIDIAGONAL matrix by the implicit QL method. The eigenvectors of a FULL SYMMETRIC matrix can also be found if TRED2 has been used to reduce this full matrix to tridiagonal form.

# Description

This canonical unsafe binding exposes original SLATEC routine `IMTQL2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [IMTQL2](https://www.netlib.org/slatec/lin/imtql2.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is not applicable or not stated by selected source not a workspace argument

## 3. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). contains the eigenvalues in ascending order.  If an not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned has been destroyed. contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned has been destroyed. not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the transformation matrix produced in the reduction by  TRED2,  if performed.  This transformation matrix is necessary if you want to obtain the eigenvectors of the full symmetric matrix.  If the eigenvectors of the symmetric tridiagonal matrix are desired, Z must contain the identity dimensional REAL array, dimensioned contains orthonormal eigenvectors of the full symmetric or symmetric tridiagonal matrix, depending on what it contained on input.  If an error exit is made,  Z contains the eigenvectors associated with the stored eigenvalues. contains the transformation matrix produced in the reduction by  TRED2,  if performed.  This transformation matrix is necessary if you want to obtain the eigenvectors of the full symmetric matrix.  If the eigenvectors of the symmetric tridiagonal matrix are desired, Z must contain the identity dimensional REAL array, dimensioned contains orthonormal eigenvectors of the full symmetric or symmetric tridiagonal matrix, depending on what it contained on input.  If an error exit is made,  Z contains the eigenvectors associated with the stored eigenvalues. not applicable or not stated by selected source not a workspace argument

## 6. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1. is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors should be correct 1, but the eigenvalues are not ordered. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::imtql2`
- Original SLATEC routine: `IMTQL2`
- Native symbol: `imtql2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [IMTQL2](https://www.netlib.org/slatec/lin/imtql2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
