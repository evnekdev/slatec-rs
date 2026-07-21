# Purpose

This subroutine is a translation of the bisection technique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval, using bisection.

# Description

This canonical unsafe binding exposes original SLATEC routine `BISECT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [BISECT](https://www.netlib.org/slatec/lin/bisect.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. 1 positions.  E(1) is arbitrary. contains the number of eigenvalues determined to lie in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `EPS1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is an absolute error tolerance for the computed positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. is a REAL variable. is unaltered unless it has been reset to its (last) default value. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `D`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the input matrix. dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. contains the diagonal elements of the input matrix. dimensional REAL array, dimensioned D(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the input matrix dimensional REAL array, dimensioned E(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. contains the subdiagonal elements of the input matrix dimensional REAL array, dimensioned E(N). are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. not applicable or not stated by selected source not a workspace argument

## 5. `E2`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). is also set to zero. contains the squares of the corresponding elements of E. dimensional REAL array, dimensional REAL array, dimensioned E2(N). dimensioned E2(N). is also set to zero. not applicable or not stated by selected source not a workspace argument

## 6. `LB`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. and UB define the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `UB`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. are REAL variables. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `MM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. should be set to an upper bound for the number of eigenvalues in the interval.  WARNING - If more than eigenvalues are determined to lie in the interval, an error return is made with no eigenvalues found. is an INTEGER variable. contains the number of eigenvalues determined to lie in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `M`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable. contains the contains the number of eigenvalues determined to lie in number of eigenvalues determined to lie in not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the M eigenvalues in ascending order. dimensional REAL array, dimensioned W(MM). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. contains the M eigenvalues in ascending order. dimensional REAL array, dimensioned W(MM). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. not applicable or not stated by selected source

## 11. `IND`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(MM). contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(MM). not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `RV4`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 14. `RV5`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY dimensional REAL arrays used for temporary storage, dimensioned RV4(N) and RV5(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in BISECT in-line. Note that subroutine TQL1 or IMTQL1 is generally faster than BISECT, if more than N/4 eigenvalues are to be found. Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `MM`: not a workspace argument
- `M`: not a workspace argument
- `W`: contains the M eigenvalues in ascending order. dimensional REAL array, dimensioned W(MM). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc.
- `IND`: not a workspace argument
- `IERR`: not a workspace argument
- `RV4`: not a workspace argument
- `RV5`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bisect`
- Original SLATEC routine: `BISECT`
- Native symbol: `bisect_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [BISECT](https://www.netlib.org/slatec/lin/bisect.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
