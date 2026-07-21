# Purpose

This subroutine is the first step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART. This subroutine accepts a pair of REAL GENERAL matrices and reduces one of them to upper Hessenberg form and the other to upper triangular form using orthogonal transformations. It is usually followed by QZIT, QZVAL and, possibly, QZVEC.

# Description

This canonical unsafe binding exposes original SLATEC routine `QZHES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QZHES](https://www.netlib.org/slatec/lin/qzhes.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. is the order of the matrices A and B.  N is an INTEGER variable.  N must be less than or equal to NM. not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). dimensional dimensional REAL array, dimensioned B(NM,N). REAL array, dimensioned B(NM,N). has been reduced to upper Hessenberg form.  The elements below the first subdiagonal have been set to zero. dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional dimensional dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). dimensional dimensional REAL array, dimensioned B(NM,N). REAL array, dimensioned B(NM,N). has been reduced to upper Hessenberg form.  The elements below the first subdiagonal have been set to zero. dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 4. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). dimensional dimensional REAL array, dimensioned B(NM,N). REAL array, dimensioned B(NM,N). has been reduced to upper triangular form.  The elements below the main diagonal have been set to zero. system Routines - EISPACK Guide, Springer-Verlag, 1976. dimensional dimensional REAL array, dimensioned B(NM,N). REAL array, dimensioned B(NM,N). has been reduced to upper triangular form.  The elements below the main diagonal have been set to zero. system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 5. `MATZ`

input `scalar` argument; Fortran declaration `LOGICAL`, Rust ABI type `*mut crate::FortranLogical`, and scalar. should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise.  MATZ is a LOGICAL variable. is not referenced. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Z`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the product of the right hand transformations if is not referenced. dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the product of the right hand transformations if is not referenced. dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `MATZ`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzhes`
- Original SLATEC routine: `QZHES`
- Native symbol: `qzhes_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_fortran_logical_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [QZHES](https://www.netlib.org/slatec/lin/qzhes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
