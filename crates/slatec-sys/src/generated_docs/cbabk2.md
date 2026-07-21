# Purpose

This subroutine is a translation of the ALGOL procedure CBABK2, which is a complex version of BALBAK, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 315-326(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding balanced matrix determined by CBAL.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBABK2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBABK2](https://www.netlib.org/slatec/lin/cbabk2.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (ZR,ZI).  N is an INTEGER variable.  N must be less than or equal to NM. (ZR,ZI).  N is an INTEGER variable.  N must be less than or equal to NM. not applicable or not stated by selected source not a workspace argument

## 3. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are INTEGER variables determined by  CBAL. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are INTEGER variables determined by  CBAL. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `SCALE`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains information determining the permutations and dimensional REAL array, dimensioned SCALE(N). contains information determining the permutations and dimensional REAL array, dimensioned SCALE(N). not applicable or not stated by selected source not a workspace argument

## 6. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). is the number of eigenvectors to be back transformed. is an INTEGER variable. dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). not applicable or not stated by selected source not a workspace argument

## 7. `ZR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 8. `ZI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `SCALE`: not a workspace argument
- `M`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cbabk2`
- Original SLATEC routine: `CBABK2`
- Native symbol: `cbabk2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [CBABK2](https://www.netlib.org/slatec/lin/cbabk2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
