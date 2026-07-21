# Purpose

This subroutine is a translation of the ALGOL procedure REDUC2, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine reduces the generalized SYMMETRIC eigenproblems ABx=(LAMBDA)x OR BAy=(LAMBDA)y, where B is POSITIVE DEFINITE, to the standard symmetric eigenproblem using the Cholesky factorization of B.

# Description

This canonical unsafe binding exposes original SLATEC routine `REDUC2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [REDUC2](https://www.netlib.org/slatec/lin/reduc2.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and B, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and B, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrices A and B.  If the Cholesky should be prefixed is an INTEGER variable. is negative, the diagonal elements of L. is not positive definite. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). is an INTEGER variable. contain the real symmetric input matrices.  Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). dimensional REAL array, dimensioned DL(N). contains in its full lower triangle the full lower triangle of the symmetric matrix derived from the reduction to the standard form.  The strict upper triangle of A is unaltered. is an INTEGER variable. contain the real symmetric input matrices.  Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). dimensional REAL array, dimensioned DL(N). contains in its full lower triangle the full lower triangle of the symmetric matrix derived from the reduction to the standard form.  The strict upper triangle of A is unaltered. not applicable or not stated by selected source not a workspace argument

## 4. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). should be prefixed contain the real symmetric input matrices.  Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). contains in its strict lower triangle the strict lower triangle of its Cholesky factor L.  The full upper triangle of B is unaltered. is not positive definite. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY system Routines - EISPACK Guide, Springer-Verlag, 1976. should be prefixed contain the real symmetric input matrices.  Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). contains in its strict lower triangle the strict lower triangle of its Cholesky factor L.  The full upper triangle of B is unaltered. is not positive definite. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 5. `DL`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is negative, the diagonal elements of L. dimensional REAL array, dimensioned DL(N). contains the diagonal elements of L. is negative, the diagonal elements of L. dimensional REAL array, dimensioned DL(N). contains the diagonal elements of L. not applicable or not stated by selected source not a workspace argument

## 6. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `DL`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::reduc2`
- Original SLATEC routine: `REDUC2`
- Native symbol: `reduc2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [REDUC2](https://www.netlib.org/slatec/lin/reduc2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
