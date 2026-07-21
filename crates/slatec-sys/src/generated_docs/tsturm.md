# Purpose

This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval and their associated eigenvectors, using bisection and inverse iteration.

# Description

This canonical unsafe binding exposes original SLATEC routine `TSTURM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TSTURM](https://www.netlib.org/slatec/lin/tsturm.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is if M exceeds MM no eigenvalues or eigenvectors are computed, th eigenvalue fails to converge in 5 iterations, then the eigenvalues and eigenvectors in W and Z should be correct for indices 1, 2, ..., J-1. The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is if M exceeds MM no eigenvalues or eigenvectors are computed, th eigenvalue fails to converge in 5 iterations, then the eigenvalues and eigenvectors in W and Z should be correct for indices 1, 2, ..., J-1. The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 3. `EPS1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. values.  It should be chosen so that the accuracy of these eigenvalues is commensurate with relative perturbations of the order of the relative machine precision in the matrix positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix.  EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. not applicable or not stated by selected source not a workspace argument

## 5. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices.  E2(1) is also set to zero. not applicable or not stated by selected source not a workspace argument

## 6. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). not applicable or not stated by selected source not a workspace argument

## 7. `LB`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. and UB define the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `UB`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `MM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. should be set to an upper bound for the number of eigenvalues in the interval.  MM is an INTEGER variable. WARNING -  If more than MM eigenvalues are determined to lie in the interval, an error return is made with no values or vectors found. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `M`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the M eigenvalues in ascending order if the matrix does not split.  If the matrix splits, the eigenvalues are in ascending order for each submatrix.  If a vector error exit is made, W contains those values already found.  W is a one-dimensional REAL array, dimensioned W(MM). contains the M eigenvalues in ascending order if the matrix does not split.  If the matrix splits, the eigenvalues are in ascending order for each submatrix.  If a vector error exit is made, W contains those values already found.  W is a one-dimensional REAL array, dimensioned W(MM). not applicable or not stated by selected source

## 12. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the associated set of orthonormal eigenvectors. If an error exit is made, Z contains those vectors already dimensional REAL array, dimensioned contains the associated set of orthonormal eigenvectors. If an error exit is made, Z contains those vectors already dimensional REAL array, dimensioned not applicable or not stated by selected source not a workspace argument

## 13. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `RV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and not applicable or not stated by selected source not a workspace argument

## 15. `RV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and not applicable or not stated by selected source not a workspace argument

## 16. `RV3`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and not applicable or not stated by selected source not a workspace argument

## 17. `RV4`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and not applicable or not stated by selected source not a workspace argument

## 18. `RV5`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and not applicable or not stated by selected source not a workspace argument

## 19. `RV6`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `EPS1`: not a workspace argument
- `D`: not a workspace argument
- `E`: not a workspace argument
- `E2`: not a workspace argument
- `LB`: not a workspace argument
- `UB`: not a workspace argument
- `MM`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains the M eigenvalues in ascending order if the matrix does not split.  If the matrix splits, the eigenvalues are in ascending order for each submatrix.  If a vector error exit is made, W contains those values already found.  W is a one-dimensional REAL array, dimensioned W(MM).
- `Z`: not a workspace argument
- `IERR`: not a workspace argument
- `RV1`: not a workspace argument
- `RV2`: not a workspace argument
- `RV3`: not a workspace argument
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument
- `RV6`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tsturm`
- Original SLATEC routine: `TSTURM`
- Native symbol: `tsturm_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TSTURM](https://www.netlib.org/slatec/lin/tsturm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
