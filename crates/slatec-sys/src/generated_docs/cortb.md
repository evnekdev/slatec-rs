# Purpose

This subroutine is a translation of a complex analogue of the ALGOL procedure ORTBAK, NUM. MATH. 12, 349-368(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 339-358(1971). This subroutine forms the eigenvectors of a COMPLEX GENERAL matrix by back transforming those of the corresponding upper Hessenberg matrix determined by CORTH.

# Description

This canonical unsafe binding exposes original SLATEC routine `CORTB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CORTB](https://www.netlib.org/slatec/lin/cortb.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, AR, AI, ZR, and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, 1 and IGH equal to the order of the matrix. are used.  ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, 1 and IGH equal to the order of the matrix. are used.  ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and not applicable or not stated by selected source not a workspace argument

## 3. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, are used.  ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, are used.  ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and not applicable or not stated by selected source not a workspace argument

## 4. `AR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). formations used in the reduction by  CORTH  in their dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH). formations used in the reduction by  CORTH  in their dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH). not applicable or not stated by selected source not a workspace argument

## 5. `AI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). formations used in the reduction by  CORTH  in their dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH). formations used in the reduction by  CORTH  in their dimensional REAL arrays, dimensioned AR(NM,IGH) and AI(NM,IGH). not applicable or not stated by selected source not a workspace argument

## 6. `ORTR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain further information about the unitary transformations used in the reduction by  CORTH.  Only and ORTI have been altered. Note that CORTB preserves vector Euclidean norms. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ORTI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain further information about the unitary transformations used in the reduction by  CORTH.  Only not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (ZR,ZI) to be back transformed. is an INTEGER variable. dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). (ZR,ZI) to be back transformed. is an INTEGER variable. dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). not applicable or not stated by selected source not a workspace argument

## 9. `ZR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. not applicable or not stated by selected source not a workspace argument

## 10. `ZI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. contain the real and imaginary parts, respectively, of the eigenvectors to be back transformed in their first dimensional REAL arrays, dimensioned ZR(NM,M) and ZI(NM,M). contain the real and imaginary parts, respectively, of the transformed eigenvectors in their first M columns. not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `LOW`: not a workspace argument
- `IGH`: not a workspace argument
- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `ORTR`: not a workspace argument
- `ORTI`: not a workspace argument
- `M`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cortb`
- Original SLATEC routine: `CORTB`
- Native symbol: `cortb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`
- Exact Netlib source file: [CORTB](https://www.netlib.org/slatec/lin/cortb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
