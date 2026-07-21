# Purpose

This subroutine is the optional fourth step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL matrices, one of them in quasi-triangular form (in which each 2-by-2 block corresponds to

# Description

This canonical unsafe binding exposes original SLATEC routine `QZVEC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZVEC](https://www.netlib.org/slatec/lin/qzvec.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. contains the tolerance quantity (EPSB) is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. contains the tolerance quantity (EPSB) not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). pair of complex eigenvalues) and the other in upper triangular form.  It computes the eigenvectors of the triangular problem and transforms the results back to the original coordinate system. It is usually preceded by  QZHES,  QZIT, and  QZVAL. triangular matrix.  A is a two- triangular matrix.  A is a two- dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). is unaltered.  Its subdiagonal elements provide information about the storage of the complex eigenvectors. th and (J+1)-th columns of Z contain its eigenvector. If ALFI(J) .LT. 0.0, the eigenvalue is the second of 1)-th and J-th columns of Z contain the conjugate of its eigenvector. Each eigenvector is normalized so that the modulus of its largest component is 1.0 . Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY pair of complex eigenvalues) and the other in upper triangular form.  It computes the eigenvectors of the triangular problem and transforms the results back to the original coordinate system. It is usually preceded by  QZHES,  QZIT, and  QZVAL. triangular matrix.  A is a two- triangular matrix.  A is a two- dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned B(NM,N). dimensional REAL array, dimensioned Z(NM,N). is unaltered.  Its subdiagonal elements provide information about the storage of the complex eigenvectors. th and (J+1)-th columns of Z contain its eigenvector. If ALFI(J) .LT. 0.0, the eigenvalue is the second of 1)-th and J-th columns of Z contain the conjugate of its eigenvector. Each eigenvector is normalized so that the modulus of its largest component is 1.0 . Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains a real upper triangular matrix.  In addition, contains the tolerance quantity (EPSB) dimensional REAL array, dimensioned B(NM,N). has been destroyed. system Routines - EISPACK Guide, Springer-Verlag, 1976. contains a real upper triangular matrix.  In addition, contains the tolerance quantity (EPSB) dimensional REAL array, dimensioned B(NM,N). has been destroyed. system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 5. `ALFR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 6. `ALFI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. th eigenvalue is real and the J-th column of Z contains its eigenvector. th eigenvalue is complex. If ALFI(J) .GT. 0.0, the eigenvalue is the first of dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. th eigenvalue is real and the J-th column of Z contains its eigenvector. th eigenvalue is complex. If ALFI(J) .GT. 0.0, the eigenvalue is the first of not applicable or not stated by selected source not a workspace argument

## 7. `BETA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. dimensional REAL arrays with are the generalized eigenvalues.  They are usually obtained from QZVAL.  They are dimensioned ALFR(N), ALFI(N), and BETA(N). are unaltered. not applicable or not stated by selected source not a workspace argument

## 8. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the transformation matrix produced in the reductions by  QZHES,  QZIT, and  QZVAL,  if performed.  If the eigenvectors of the triangular problem are desired, Z must dimensional REAL array, dimensioned Z(NM,N). contains the real and imaginary parts of the eigenvectors. contains the transformation matrix produced in the reductions by  QZHES,  QZIT, and  QZVAL,  if performed.  If the eigenvectors of the triangular problem are desired, Z must dimensional REAL array, dimensioned Z(NM,N). contains the real and imaginary parts of the eigenvectors. not applicable or not stated by selected source not a workspace argument

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
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzvec`
- Original SLATEC routine: `QZVEC`
- Native symbol: `qzvec_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZVEC](https://www.netlib.org/slatec/lin/qzvec.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
