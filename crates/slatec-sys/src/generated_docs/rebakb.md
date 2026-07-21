# Purpose

This subroutine is a translation of the ALGOL procedure REBAKB, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine forms the eigenvectors of a generalized SYMMETRIC eigensystem by back transforming those of the derived symmetric matrix determined by REDUC2.

# Description

This canonical unsafe binding exposes original SLATEC routine `REBAKB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [REBAKB](https://www.netlib.org/slatec/lin/rebakb.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, B and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, B and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix system.  N is an INTEGER variable.  N must be less than or equal to NM. is the order of the matrix system.  N is an INTEGER variable.  N must be less than or equal to NM. not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains information about the similarity transformation (Cholesky decomposition) used in the reduction by  REDUC2 dimensional REAL array, dimensioned B(NM,N). system Routines - EISPACK Guide, Springer-Verlag, 1976. contains information about the similarity transformation (Cholesky decomposition) used in the reduction by  REDUC2 dimensional REAL array, dimensioned B(NM,N). system Routines - EISPACK Guide, Springer-Verlag, 1976. not applicable or not stated by selected source not a workspace argument

## 4. `DL`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains further information about the transformation. dimensional REAL array, dimensioned DL(N). contains further information about the transformation. dimensional REAL array, dimensioned DL(N). not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL array dimensioned Z(NM,M). columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL array dimensioned Z(NM,M). columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 6. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the eigenvectors to be back transformed in its dimensional REAL array dimensioned Z(NM,M). contains the transformed eigenvectors in its first contains the eigenvectors to be back transformed in its dimensional REAL array dimensioned Z(NM,M). contains the transformed eigenvectors in its first not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `B`: not a workspace argument
- `DL`: not a workspace argument
- `M`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rebakb`
- Original SLATEC routine: `REBAKB`
- Native symbol: `rebakb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [REBAKB](https://www.netlib.org/slatec/lin/rebakb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
