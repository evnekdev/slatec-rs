# Purpose

This subroutine is the third step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form and the other in upper triangular form. It reduces the quasi-triangular matrix further, so that any remaining 2-by-2 blocks correspond to pairs of complex eigenvalues, and returns quantities whose ratios give the generalized eigenvalues. It is usually preceded by QZHES and QZIT and may be followed by QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZVAL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZVAL](https://www.netlib.org/slatec/lin/qzval.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. contains the tolerance quantity (EPSB) is unaltered. and ALFI(N). is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. contains the tolerance quantity (EPSB) is unaltered. and ALFI(N). not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). triangular matrix.  A is a two- triangular matrix.  A is a two- dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). triangular matrix in triangular matrix in which all nonzero subdiagonal elements correspond to pairs which all nonzero subdiagonal elements correspond to pairs of complex eigenvalues. of complex eigenvalues. dimensional REAL array, dimensioned BETA(N). triangular matrix.  A is a two- triangular matrix.  A is a two- dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). triangular matrix in triangular matrix in which all nonzero subdiagonal elements correspond to pairs which all nonzero subdiagonal elements correspond to pairs of complex eigenvalues. of complex eigenvalues. dimensional REAL array, dimensioned BETA(N). not applicable or not stated by selected source not a workspace argument

## 4. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains a real upper triangular matrix.  In addition, contains the tolerance quantity (EPSB) dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements is unaltered. system Routines - EISPACK Guide, Springer-Verlag, 1976. contains a real upper triangular matrix.  In addition, contains the tolerance quantity (EPSB) dimensional REAL array, dimensioned B(NM,N). is still in upper triangular form, although its elements is unaltered. system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 5. `ALFR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations.  Non-zero values of ALFI occur in pairs, the first member positive and the second negative. dimensional REAL arrays, dimensioned and ALFI(N). contain the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations.  Non-zero values of ALFI occur in pairs, the first member positive and the second negative. dimensional REAL arrays, dimensioned and ALFI(N). not applicable or not stated by selected source not a workspace argument

## 6. `ALFI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations.  Non-zero values of ALFI occur in pairs, the first member positive and the second negative. dimensional REAL arrays, dimensioned contain the real and imaginary parts of the diagonal elements of the triangular matrix that would be obtained if A were reduced completely to triangular form by unitary transformations.  Non-zero values of ALFI occur in pairs, the first member positive and the second negative. dimensional REAL arrays, dimensioned not applicable or not stated by selected source not a workspace argument

## 7. `BETA`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the diagonal elements of the corresponding B, normalized to be real and non-negative.  The generalized eigenvalues are then the ratios ((ALFR+I*ALFI)/BETA). dimensional REAL array, dimensioned BETA(N). contains the diagonal elements of the corresponding B, normalized to be real and non-negative.  The generalized eigenvalues are then the ratios ((ALFR+I*ALFI)/BETA). dimensional REAL array, dimensioned BETA(N). not applicable or not stated by selected source not a workspace argument

## 8. `MATZ`

input `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise.  MATZ is a LOGICAL variable. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reductions by  QZHES  and  QZIT,  if performed, or else the identity matrix.  If MATZ has been set dimensional REAL dimensional REAL array, dimensioned Z(NM,N). array, dimensioned Z(NM,N). contains the product of the right hand transformations (for all three steps) if MATZ has been set to .TRUE. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reductions by  QZHES  and  QZIT,  if performed, or else the identity matrix.  If MATZ has been set dimensional REAL dimensional REAL array, dimensioned Z(NM,N). array, dimensioned Z(NM,N). contains the product of the right hand transformations (for all three steps) if MATZ has been set to .TRUE. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `ALFR`: not a workspace argument
- `ALFI`: not a workspace argument
- `BETA`: not a workspace argument
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzval`
- Original SLATEC routine: `QZVAL`
- Native symbol: `qzval_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZVAL](https://www.netlib.org/slatec/lin/qzval.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
