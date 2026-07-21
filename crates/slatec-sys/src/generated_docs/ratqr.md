# Purpose

This subroutine is a translation of the ALGOL procedure RATQR, NUM. MATH. 11, 264-272(1968) by REINSCH and BAUER. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 257-265(1971). This subroutine finds the algebraically smallest or largest eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QR method with Newton corrections.

# Description

This canonical unsafe binding exposes original SLATEC routine `RATQR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RATQR](https://www.netlib.org/slatec/lin/ratqr.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is 1 positions.  E2(1) is arbitrary.  E2 is a one- dimensional REAL array, dimensioned E2(N). is set to 1 and  TYPE  to .TRUE. when the matrix is NOT positive definite, or th eigenvalue are NOT monotone increasing, where K refers to the last such occurrence. Note that subroutine TRIDIB is generally faster and more accurate than RATQR if the eigenvalues are clustered. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is 1 positions.  E2(1) is arbitrary.  E2 is a one- dimensional REAL array, dimensioned E2(N). is set to 1 and  TYPE  to .TRUE. when the matrix is NOT positive definite, or th eigenvalue are NOT monotone increasing, where K refers to the last such occurrence. Note that subroutine TRIDIB is generally faster and more accurate than RATQR if the eigenvalues are clustered. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 2. `EPS1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a theoretical absolute error tolerance for the positive, or indeed smaller than its default value, it is reset at each iteration to the respective default value, namely, the product of the relative machine precision and the magnitude of the current eigenvalue iterate.  The theoretical absolute is a REAL variable. is a REAL variable. is unaltered unless it has been reset to its (last) default value. dimensional REAL array, dimensioned BD(N).  BD need not be distinct from E2. is a theoretical absolute error tolerance for the positive, or indeed smaller than its default value, it is reset at each iteration to the respective default value, namely, the product of the relative machine precision and the magnitude of the current eigenvalue iterate.  The theoretical absolute is a REAL variable. is a REAL variable. is unaltered unless it has been reset to its (last) default value. dimensional REAL array, dimensioned BD(N).  BD need not be distinct from E2. not applicable or not stated by selected source not a workspace argument

## 3. `D`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is set to 0.0e0 if the smallest eigenvalues have been found, and to 2.0e0 if the largest eigenvalues have been found.  E2 is otherwise unaltered (unless overwritten by BD). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is set to 0.0e0 if the smallest eigenvalues have been found, and to 2.0e0 if the largest eigenvalues have been found.  E2 is otherwise unaltered (unless overwritten by BD). not applicable or not stated by selected source not a workspace argument

## 4. `E`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is set to 0.0e0 if the smallest eigenvalues have been found, and to 2.0e0 if the largest eigenvalues have been found.  E2 is otherwise unaltered (unless overwritten by BD). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered (unless W overwrites D). Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is set to 0.0e0 if the smallest eigenvalues have been found, and to 2.0e0 if the largest eigenvalues have been found.  E2 is otherwise unaltered (unless overwritten by BD). not applicable or not stated by selected source not a workspace argument

## 5. `E2`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `M`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvalues to be found.  M is an INTEGER variable. is greater than N, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the M algebraically smallest eigenvalues in ascending order, or the M largest eigenvalues in descending order.  If an error exit is made because of an incorrect specification of IDEF, no eigenvalues are found.  If the Newton iterates for a particular eigenvalue are not monotone, the best estimate obtained is returned and IERR is set. dimensional REAL array, dimensioned W(N).  W need not be distinct from D. 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. contains the M algebraically smallest eigenvalues in ascending order, or the M largest eigenvalues in descending order.  If an error exit is made because of an incorrect specification of IDEF, no eigenvalues are found.  If the Newton iterates for a particular eigenvalue are not monotone, the best estimate obtained is returned and IERR is set. dimensional REAL array, dimensioned W(N).  W need not be distinct from D. 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. not applicable or not stated by selected source

## 8. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(N). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(N). not applicable or not stated by selected source not a workspace argument

## 9. `BD`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains refined bounds for the theoretical errors of the corresponding eigenvalues in W.  These bounds are usually dimensional REAL array, dimensioned BD(N).  BD need not be distinct from E2. contains refined bounds for the theoretical errors of the corresponding eigenvalues in W.  These bounds are usually dimensional REAL array, dimensioned BD(N).  BD need not be distinct from E2. not applicable or not stated by selected source not a workspace argument

## 10. `TYPE`

input `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. Input logical selector. Set true to compute algebraically smallest eigenvalues and false to compute algebraically largest eigenvalues; it also determines the ordering of `W` and the sentinel stored in `E2(1)`. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IDEF`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. should be set to 1 if the input matrix is known to be positive definite, to -1 if the input matrix is known to be negative definite, and to 0 otherwise.  IDEF is an INTEGER variable. is set to 1 and  TYPE  to .TRUE. when the matrix is NOT positive definite, or 1 and  TYPE  to .FALSE. when the matrix is NOT negative definite, no eigenvalues are computed, or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `EPS1`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains the M algebraically smallest eigenvalues in ascending order, or the M largest eigenvalues in descending order.  If an error exit is made because of an incorrect specification of IDEF, no eigenvalues are found.  If the Newton iterates for a particular eigenvalue are not monotone, the best estimate obtained is returned and IERR is set. dimensional REAL array, dimensioned W(N).  W need not be distinct from D. 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc.
- `IND`: not a workspace argument
- `BD`: not a workspace argument
- `TYPE`: not a workspace argument
- `IDEF`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ratqr`
- Original SLATEC routine: `RATQR`
- Native symbol: `ratqr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [RATQR](https://www.netlib.org/slatec/lin/ratqr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
