# Purpose

This subroutine is a translation of the ALGOL procedure BISECT, NUM. MATH. 9, 386-393(1967) by Barth, Martin, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 249-256(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix between specified boundary indices, using bisection.

# Description

This canonical unsafe binding exposes original SLATEC routine `TRIDIB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TRIDIB](https://www.netlib.org/slatec/lin/tridib.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is if multiple eigenvalues at index M11 make unique selection of LB impossible, if multiple eigenvalues at index M22 make unique selection of UB impossible. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `EPS1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix.  EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. not applicable or not stated by selected source not a workspace argument

## 5. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). is also set to zero. contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). is also set to zero. not applicable or not stated by selected source not a workspace argument

## 6. `LB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. and UB define an interval containing exactly the desired are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `UB`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `M11`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. specifies the lower boundary index for the set of desired eigenvalues.  M11 is an INTEGER variable. 1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. specifies the number of eigenvalues desired.  The upper 1. is an INTEGER variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. not applicable or not stated by selected source

## 11. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(M). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(M). not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `RV4`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 14. `RV5`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage of the lower and upper bounds for the eigenvalues in are dimensioned RV4(N) and RV5(N). Note that subroutine TQL1, IMTQL1, or TQLRAT is generally faster than TRIDIB, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `LB`: not a workspace argument
- `UB`: not a workspace argument
- `M11`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains, in its first M positions, the eigenvalues between indices M11 and M22 in ascending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc.
- `IND`: not a workspace argument
- `IERR`: not a workspace argument
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tridib`
- Original SLATEC routine: `TRIDIB`
- Native symbol: `tridib_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [TRIDIB](https://www.netlib.org/slatec/lin/tridib.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
