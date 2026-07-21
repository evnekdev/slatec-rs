# Purpose

This subroutine finds those eigenvectors of a REAL SYMMETRIC BAND matrix corresponding to specified eigenvalues, using inverse iteration. The subroutine may also be used to solve systems

# Description

This canonical unsafe binding exposes original SLATEC routine `BANDV`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BANDV](https://www.netlib.org/slatec/lin/bandv.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. MB positions of the first column, MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of column MB. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, A is 1 instead with lower triangle as above and with 1 positions of 2 positions of column MB+2, further superdiagonals similarly, MB positions of the last column.  Contents of storage locations 1). is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. MB positions of the first column, MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of column MB. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, A is 1 instead with lower triangle as above and with 1 positions of 2 positions of column MB+2, further superdiagonals similarly, MB positions of the last column.  Contents of storage locations 1). not applicable or not stated by selected source not a workspace argument

## 3. `MBW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of columns of the array A used to store the band matrix.  If the matrix is symmetric, MBW is its (half) band width, denoted MB and defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix.  If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, it must however have the same number of adjacent diagonals above the main diagonal as below, and in this 1.  MBW is an INTEGER variable.  MB must not be greater than N. is the number of columns of the array A used to store the band matrix.  If the matrix is symmetric, MBW is its (half) band width, denoted MB and defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix.  If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, it must however have the same number of adjacent diagonals above the main diagonal as below, and in this 1.  MBW is an INTEGER variable.  MB must not be greater than N. not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). symmetric band coefficient matrix. contains the lower triangle of the symmetric band input matrix stored as an N by MB array.  Its lowest subdiagonal dimensional dimensional REAL array, dimensioned A(NM,MBW). REAL array, dimensioned A(NM,MBW). W(J)*I)*X(J)=B(J), where I is the identity dimensional REAL array, dimensioned W(M). dimensional REAL array, dimensioned Z(NM,M). are unaltered. W(M)*I is available, upon return, as the product of the first N elements of RV. symmetric band coefficient matrix. contains the lower triangle of the symmetric band input matrix stored as an N by MB array.  Its lowest subdiagonal dimensional dimensional REAL array, dimensioned A(NM,MBW). REAL array, dimensioned A(NM,MBW). W(J)*I)*X(J)=B(J), where I is the identity dimensional REAL array, dimensioned W(M). dimensional REAL array, dimensioned Z(NM,M). are unaltered. W(M)*I is available, upon return, as the product of the first N elements of RV. not applicable or not stated by selected source not a workspace argument

## 5. `E21`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. specifies the ordering of the eigenvalues and contains 0.0E0 if the eigenvalues are in ascending order, or 2.0E0 if the eigenvalues are in descending order. If the subroutine is being used to solve systems of linear equations, E21 should be set to 1.0E0 if the coefficient matrix is symmetric and to -1.0E0 if not.  E21 is a REAL variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of specified eigenvalues or the number of systems of linear equations.  M is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear 1,2,...,M. dimensional REAL array, dimensioned W(M). are unaltered. contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear 1,2,...,M. dimensional REAL array, dimensioned W(M). are unaltered. not applicable or not stated by selected source

## 8. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). 1,2,...,M), if the subroutine is used to solve systems of linear equations. dimensional REAL array, dimensioned Z(NM,M). contains the associated set of orthogonal eigenvectors. Any vector which fails to converge is set to zero.  If the subroutine is used to solve systems of linear equations, 1,2,...,M). 1,2,...,M), if the subroutine is used to solve systems of linear equations. dimensional REAL array, dimensioned Z(NM,M). contains the associated set of orthogonal eigenvectors. Any vector which fails to converge is set to zero.  If the subroutine is used to solve systems of linear equations, 1,2,...,M). not applicable or not stated by selected source not a workspace argument

## 9. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, -J         if the eigenvector corresponding to the J-th eigenvalue fails to converge, or if the J-th system of linear equations is nearly singular. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NV`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable. 1). 1). must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable. 1). 1). not applicable or not stated by selected source not a workspace argument

## 11. `RV`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV 1). are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV 1). not applicable or not stated by selected source not a workspace argument

## 12. `RV6`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV is dimensioned RV6(N). Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV is dimensioned RV6(N). Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `MBW`: not a workspace argument
- `A`: not a workspace argument
- `E21`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear 1,2,...,M. dimensional REAL array, dimensioned W(M). are unaltered.
- `Z`: not a workspace argument
- `IERR`: not a workspace argument
- `NV`: not a workspace argument
- `RV`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandv`
- Original SLATEC routine: `BANDV`
- Native symbol: `bandv_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BANDV](https://www.netlib.org/slatec/lin/bandv.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
