# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure TRBAK3, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine forms the eigenvectors of a COMPLEX HERMITIAN matrix by back transforming those of the corresponding real symmetric tridiagonal matrix determined by HTRID3.

# Description

This canonical unsafe binding exposes original SLATEC routine `HTRIB3`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HTRIB3](https://www.netlib.org/slatec/lin/htrib3.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, A, ZR, and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, A, ZR, and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. not applicable or not stated by selected source not a workspace argument

## 3. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains some information about the unitary transformations dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned TAU(2,N). contains some information about the unitary transformations dimensional dimensional REAL array, dimensioned A(NM,N). REAL array, dimensioned A(NM,N). dimensional REAL array, dimensioned TAU(2,N). not applicable or not stated by selected source not a workspace argument

## 4. `TAU`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (2, *). contains further information about the transformations. dimensional REAL array, dimensioned TAU(2,N). contains further information about the transformations. dimensional REAL array, dimensioned TAU(2,N). not applicable or not stated by selected source not a workspace argument

## 5. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvectors to be back transformed. is an INTEGER variable. are immaterial.  ZR and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `ZR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the eigenvectors to be back transformed in its contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the eigenvectors to be back transformed in its contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 7. `ZI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). are immaterial.  ZR and dimensional REAL arrays, dimensioned ZR(NM,M) and contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY are immaterial.  ZR and dimensional REAL arrays, dimensioned ZR(NM,M) and contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. NOTE that the last component of each returned vector is real and that vector Euclidean norms are preserved. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `TAU`: not a workspace argument
- `M`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::htrib3`
- Original SLATEC routine: `HTRIB3`
- Native symbol: `htrib3_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [HTRIB3](https://www.netlib.org/slatec/lin/htrib3.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
