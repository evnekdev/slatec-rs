# Purpose

This subroutine is a translation of the ALGOL procedure ORTHES, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). Given a REAL GENERAL matrix, this subroutine reduces a submatrix situated in rows and columns

# Description

This canonical unsafe binding exposes original SLATEC routine `ORTHES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [ORTHES](https://www.netlib.org/slatec/lin/orthes.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, A, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, A, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. not applicable or not stated by selected source not a workspace argument

## 3. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. through IGH to upper Hessenberg form by orthogonal similarity transformations. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been 1 and IGH equal to the order of the matrix, N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 5. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the general matrix to be reduced to upper dimensional REAL array, dimensional REAL array, dimensioned A(NM,N). dimensioned A(NM,N). contains the upper Hessenberg matrix.  Some information about the orthogonal transformations used in the reduction is stored in the remaining triangle under the Hessenberg matrix. dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contains the general matrix to be reduced to upper dimensional REAL array, dimensional REAL array, dimensioned A(NM,N). dimensioned A(NM,N). contains the upper Hessenberg matrix.  Some information about the orthogonal transformations used in the reduction is stored in the remaining triangle under the Hessenberg matrix. dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 6. `ORT`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). formations used in the reduction.  Only elements LOW+1 dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY formations used in the reduction.  Only elements LOW+1 dimensional REAL array, dimensioned ORT(IGH). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `LOW`: not a workspace argument
- `IGH`: not a workspace argument
- `A`: not a workspace argument
- `ORT`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::orthes`
- Original SLATEC routine: `ORTHES`
- Native symbol: `orthes_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1)`
- Exact Netlib source file: [ORTHES](https://www.netlib.org/slatec/lin/orthes.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
