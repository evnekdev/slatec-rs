# Purpose

This subroutine is a translation of the ALGOL procedure HQR, NUM. MATH. 14, 219-231(1970) by Martin, Peters, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 359-371(1971). This subroutine finds the eigenvalues of a REAL UPPER Hessenberg matrix by the QR method.

# Description

This canonical unsafe binding exposes original SLATEC routine `HQR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HQR](https://www.netlib.org/slatec/lin/hqr.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameter, H, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameter, H, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is the order of the matrix H.  N is an INTEGER variable. must be less than or equal to NM. dimensional REAL arrays, dimensioned WR(N) and WI(N). is the order of the matrix H.  N is an INTEGER variable. must be less than or equal to NM. dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 3. `LOW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been 1 and IGH equal to the order of the matrix, N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IGH`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are two INTEGER variables determined by the balancing subroutine  BALANC.  If  BALANC  has not been not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `H`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contains the upper Hessenberg matrix.  Information about the transformations used in the reduction to Hessenberg form by  ELMHES  or  ORTHES, if performed, is stored in the remaining triangle under the Hessenberg matrix. dimensional REAL array, dimensioned H(NM,N). has been destroyed.  Therefore, it must be saved before calling  HQR  if subsequent calculation and back transformation of eigenvectors is to be performed. contains the upper Hessenberg matrix.  Information about the transformations used in the reduction to Hessenberg form by  ELMHES  or  ORTHES, if performed, is stored in the remaining triangle under the Hessenberg matrix. dimensional REAL array, dimensioned H(NM,N). has been destroyed.  Therefore, it must be saved before calling  HQR  if subsequent calculation and back transformation of eigenvectors is to be performed. not applicable or not stated by selected source not a workspace argument

## 6. `WR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 7. `WI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct dimensional REAL arrays, dimensioned WR(N) and WI(N). contain the real and imaginary parts, respectively, of the eigenvalues.  The eigenvalues are unordered except that complex conjugate pairs of values appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct dimensional REAL arrays, dimensioned WR(N) and WI(N). not applicable or not stated by selected source not a workspace argument

## 8. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional REAL arrays, dimensioned WR(N) and WI(N). dimensional REAL arrays, dimensioned WR(N) and WI(N). is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+2, ..., N. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL arrays, dimensioned WR(N) and WI(N). dimensional REAL arrays, dimensioned WR(N) and WI(N). is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+2, ..., N. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

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
- `H`: not a workspace argument
- `WR`: not a workspace argument
- `WI`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::hqr`
- Original SLATEC routine: `HQR`
- Native symbol: `hqr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [HQR](https://www.netlib.org/slatec/lin/hqr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
