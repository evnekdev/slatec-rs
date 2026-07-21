# Purpose

This subroutine is a translation of the ALGOL procedure COMLR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine finds the eigenvalues and eigenvectors of a COMPLEX UPPER Hessenberg matrix by the modified LR method. The eigenvectors of a COMPLEX GENERAL matrix can also be found if COMHES has been used to reduce this general matrix to Hessenberg form.

# Description

This canonical unsafe binding exposes original SLATEC routine `COMLR2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COMLR2](https://www.netlib.org/slatec/lin/comlr2.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, HR, HI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, HR, HI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (HR,HI).  N is an INTEGER variable.  N must be less than or equal to NM. dimensional REAL arrays, dimensioned WR(N) and WI(N). (HR,HI).  N is an INTEGER variable.  N must be less than or equal to NM. dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 3. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, 1 and IGH equal to the order of the matrix, N. are used.  If you want the eigenvectors of a complex general matrix, leave INT as it came from  COMHES.  If the eigenvectors of the Hessenberg not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, are used.  If you want the eigenvectors of a complex general matrix, leave INT as it came from  COMHES.  If the eigenvectors of the Hessenberg not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). contains information on the rows and columns interchanged in the reduction by  COMHES, if performed. J for these elements.  INT is a one-dimensional INTEGER array, dimensioned INT(IGH). contains information on the rows and columns interchanged in the reduction by  COMHES, if performed. J for these elements.  INT is a one-dimensional INTEGER array, dimensioned INT(IGH). not applicable or not stated by selected source not a workspace argument

## 6. `HR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by  COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by  COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and not applicable or not stated by selected source not a workspace argument

## 7. `HI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by  COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by  COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and not applicable or not stated by selected source not a workspace argument

## 8. `WR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 9. `WI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 10. `ZR`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). not applicable or not stated by selected source not a workspace argument

## 11. `ZI`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional REAL arrays, dimensioned WR(N) and WI(N). dimensional REAL arrays, dimensioned WR(N) and WI(N). is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+2, ..., N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays, dimensioned WR(N) and WI(N). dimensional REAL arrays, dimensioned WR(N) and WI(N). is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+2, ..., N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `INT`: not a workspace argument
- `HR`: not a workspace argument
- `HI`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::comlr2`
- Original SLATEC routine: `COMLR2`
- Native symbol: `comlr2_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [COMLR2](https://www.netlib.org/slatec/lin/comlr2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
