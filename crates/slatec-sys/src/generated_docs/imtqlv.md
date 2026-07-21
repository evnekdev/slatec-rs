# Purpose

This subroutine is a variant of IMTQL1 which is a translation of ALGOL procedure IMTQL1, NUM. MATH. 12, 377-383(1968) by Martin and Wilkinson, as modified in NUM. MATH. 15, 450(1970) by Dubrulle. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 241-248(1971). This subroutine finds the eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the implicit QL method and associates with them their corresponding submatrix indices.

# Description

This canonical unsafe binding exposes original SLATEC routine `IMTQLV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [IMTQLV](https://www.netlib.org/slatec/lin/imtqlv.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is 1 positions.  E2(1) is arbitrary.  E2 is a one- dimensional REAL array, dimensioned E2(N). is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is 1 positions.  E2(1) is arbitrary.  E2 is a one- dimensional REAL array, dimensioned E2(N). not applicable or not stated by selected source not a workspace argument

## 2. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. not applicable or not stated by selected source not a workspace argument

## 3. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. not applicable or not stated by selected source not a workspace argument

## 4. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the eigenvalues in ascending order.  If an error exit is made, the eigenvalues are correct and ordered for dimensional REAL array, dimensioned 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to contains the eigenvalues in ascending order.  If an error exit is made, the eigenvalues are correct and ordered for dimensional REAL array, dimensioned 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to not applicable or not stated by selected source

## 6. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains the submatrix indices associated with the dimensional REAL array, dimensioned IND(N). contains the submatrix indices associated with the dimensional REAL array, dimensioned IND(N). not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1, but may not be the smallest is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues should be correct for indices 1.  These eigenvalues are ordered, but are not necessarily the smallest. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `W`: contains the eigenvalues in ascending order.  If an error exit is made, the eigenvalues are correct and ordered for dimensional REAL array, dimensioned 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to
- `IND`: not a workspace argument
- `IERR`: not a workspace argument
- `RV1`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::imtqlv`
- Original SLATEC routine: `IMTQLV`
- Native symbol: `imtqlv_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [IMTQLV](https://www.netlib.org/slatec/lin/imtqlv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
