# Purpose

This subroutine is the second step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART, as modified in technical note NASA TN D-7305(1973) by WARD. This subroutine accepts a pair of REAL matrices, one of them in upper Hessenberg form and the other in upper triangular form. It reduces the Hessenberg matrix to quasi-triangular form using orthogonal transformations while maintaining the triangular form of the other matrix. It is usually preceded by QZHES and followed by QZVAL and, possibly, QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZIT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZIT](https://www.netlib.org/slatec/lin/qzit.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. is used to store is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. is used to store not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). triangular form.  The elements below the first subdiagonal are still zero, and no two consecutive subdiagonal elements are nonzero. 1) nor A(J-1,J-2) has become zero after a total of 30*N iterations. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). triangular form.  The elements below the first subdiagonal are still zero, and no two consecutive subdiagonal elements are nonzero. 1) nor A(J-1,J-2) has become zero after a total of 30*N iterations. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements is used to store system Routines - EISPACK Guide, Springer-Verlag, 1976. dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements is used to store system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 5. `EPS1`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a tolerance used to determine negligible elements. 0.0 (or negative) may be input, in which case an element will be neglected only if it is less than roundoff times the norm of B for later use by  QZVAL  and  QZVEC. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `MATZ`

input `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise.  MATZ is a LOGICAL variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reduction by  QZHES, if performed, or else the identity matrix.  If MATZ has been set to .FALSE., dimensional REAL array, dimensional REAL array, dimensioned Z(NM,N). dimensioned Z(NM,N). contains the product of the right hand transformations (for both steps) if MATZ has been set to .TRUE. contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reduction by  QZHES, if performed, or else the identity matrix.  If MATZ has been set to .FALSE., dimensional REAL array, dimensional REAL array, dimensioned Z(NM,N). dimensioned Z(NM,N). contains the product of the right hand transformations (for both steps) if MATZ has been set to .TRUE. not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

positive, then an element will be considered negligible if it is less than EPS1 times the norm of its matrix.  A positive value of EPS1 may result in faster execution, but less accurate results.  EPS1 is a REAL variable.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `EPS1`: not a workspace argument
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzit`
- Original SLATEC routine: `QZIT`
- Native symbol: `qzit_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_fortran_logical_i32,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [QZIT](https://www.netlib.org/slatec/lin/qzit.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
