# Purpose

This subroutine is a translation of the ALGOL procedure TRBAK1, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a REAL SYMMETRIC matrix by back transforming those of the corresponding symmetric tridiagonal matrix determined by TRED1.

# Description

This canonical unsafe binding exposes original SLATEC routine `TRBAK1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [TRBAK1](https://www.netlib.org/slatec/lin/trbak1.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is arbitrary.  These elements provide the remaining information about the is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is arbitrary.  These elements provide the remaining information about the not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains information about the orthogonal transformations used in the reduction by  TRED1  in its strict lower dimensional REAL array, dimensioned dimensional REAL array, dimensioned dimensional REAL array, dimensioned E(N). dimensional REAL array, dimensioned Z(NM,M). contains information about the orthogonal transformations used in the reduction by  TRED1  in its strict lower dimensional REAL array, dimensioned dimensional REAL array, dimensioned dimensional REAL array, dimensioned E(N). dimensional REAL array, dimensioned Z(NM,M). not applicable or not stated by selected source not a workspace argument

## 4. `E`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the subdiagonal elements of the tridiagonal matrix dimensional REAL array, dimensioned E(N). contains the subdiagonal elements of the tridiagonal matrix dimensional REAL array, dimensioned E(N). not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of columns of Z to be back transformed. is an INTEGER variable. dimensional REAL array, dimensioned Z(NM,M). is the number of columns of Z to be back transformed. is an INTEGER variable. dimensional REAL array, dimensioned Z(NM,M). not applicable or not stated by selected source not a workspace argument

## 6. `Z`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the eigenvectors to be back transformed in its dimensional REAL array, dimensioned Z(NM,M). contains the transformed eigenvectors in its first M columns. Note that TRBAK1 preserves vector Euclidean norms. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the eigenvectors to be back transformed in its dimensional REAL array, dimensioned Z(NM,M). contains the transformed eigenvectors in its first M columns. Note that TRBAK1 preserves vector Euclidean norms. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `E`: not a workspace argument
- `M`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::trbak1`
- Original SLATEC routine: `TRBAK1`
- Native symbol: `trbak1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`
- Exact Netlib source file: [TRBAK1](https://www.netlib.org/slatec/lin/trbak1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
