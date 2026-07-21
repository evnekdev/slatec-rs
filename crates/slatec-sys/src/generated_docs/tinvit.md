# Purpose

This subroutine is a translation of the inverse iteration tech- nique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a TRIDIAGONAL SYMMETRIC matrix corresponding to specified eigenvalues, using inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `TINVIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TINVIT](https://www.netlib.org/slatec/lin/tinvit.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). 1).  E2(1) must contain 1).  E2(1) must contain 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 if the eigenvalues are in descending order.  If  BISECT, if the eigenvalues are in descending order.  If  BISECT, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, their output E2 array is exactly what is expected here. their output E2 array is exactly what is expected here. contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). 1).  E2(1) must contain 1).  E2(1) must contain 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 if the eigenvalues are in descending order.  If  BISECT, if the eigenvalues are in descending order.  If  BISECT, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, their output E2 array is exactly what is expected here. their output E2 array is exactly what is expected here. not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned is considered negligible if it is not larger than the product of the relative machine precision and the sum contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned is considered negligible if it is not larger than the product of the relative machine precision and the sum not applicable or not stated by selected source not a workspace argument

## 5. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E, with zeros corresponding to negligible elements of E. dimensional REAL array, dimensioned E2(N). contains the squares of the corresponding elements of E, with zeros corresponding to negligible elements of E. dimensional REAL array, dimensioned E2(N). not applicable or not stated by selected source not a workspace argument

## 6. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of specified eigenvalues for which eigenvectors are to be determined.  M is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the M eigenvalues in ascending or descending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If  BISECT  or  TRIDIB  has been used to determine the eigenvalues, their output IND array is suitable for input contains the M eigenvalues in ascending or descending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If  BISECT  or  TRIDIB  has been used to determine the eigenvalues, their output IND array is suitable for input not applicable or not stated by selected source

## 8. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(M). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(M). not applicable or not stated by selected source not a workspace argument

## 9. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the associated set of orthonormal eigenvectors. Any vector which fails to converge is set to zero. dimensional REAL array, dimensioned Z(NM,M). contains the associated set of orthonormal eigenvectors. Any vector which fails to converge is set to zero. dimensional REAL array, dimensioned Z(NM,M). not applicable or not stated by selected source not a workspace argument

## 10. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, -J         if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `RV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and not applicable or not stated by selected source not a workspace argument

## 12. `RV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and not applicable or not stated by selected source not a workspace argument

## 13. `RV3`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and are dimensioned RV1(N), RV2(N) and RV3(N). dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and are dimensioned RV1(N), RV2(N) and RV3(N). not applicable or not stated by selected source not a workspace argument

## 14. `RV4`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and not applicable or not stated by selected source not a workspace argument

## 15. `RV6`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `E2`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains the M eigenvalues in ascending or descending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If  BISECT  or  TRIDIB  has been used to determine the eigenvalues, their output IND array is suitable for input
- `IND`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument
- `RV3`: not a workspace argument
- `RV4`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tinvit`
- Original SLATEC routine: `TINVIT`
- Native symbol: `tinvit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TINVIT](https://www.netlib.org/slatec/lin/tinvit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
