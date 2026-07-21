# Purpose

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a COMPLEX HERMITIAN matrix.

# Description

This canonical unsafe binding exposes original SLATEC routine `CH`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CH](https://www.netlib.org/slatec/lin/ch.f).

# Arguments

## 1. `NM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after a total of 30 iterations. determined after a total of 30 iterations. The eigenvalues should be correct for indices The eigenvalues should be correct for indices (AR,AI).  N is an INTEGER variable.  N must be less than or equal to NM. is greater than NM, is greater than NM, J          if the J-th eigenvalue has not been J          if the J-th eigenvalue has not been determined after a total of 30 iterations. determined after a total of 30 iterations. The eigenvalues should be correct for indices The eigenvalues should be correct for indices not applicable or not stated by selected source not a workspace argument

## 3. `AR`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex Hermitian matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). contain the real and imaginary parts, respectively, of the complex Hermitian matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). not applicable or not stated by selected source not a workspace argument

## 4. `AI`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the complex Hermitian matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). contain the real and imaginary parts, respectively, of the complex Hermitian matrix.  AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). not applicable or not stated by selected source not a workspace argument

## 5. `W`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the eigenvalues in ascending order. dimensional REAL array, dimensioned W(N). contains the eigenvalues in ascending order. dimensional REAL array, dimensioned W(N). not applicable or not stated by selected source

## 6. `MATZ`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER variable set equal to zero if only eigenvalues are desired.  Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `ZR`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and contain the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and not applicable or not stated by selected source not a workspace argument

## 8. `ZI`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (NM, *). contain the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and contain the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero.  ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and not applicable or not stated by selected source not a workspace argument

## 9. `FV1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). not applicable or not stated by selected source not a workspace argument

## 10. `FV2`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). dimensional REAL arrays used for temporary storage, dimensioned FV1(N) and FV2(N). not applicable or not stated by selected source not a workspace argument

## 11. `FM1`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (2, *). dimensional REAL array used for temporary storage, dimensioned FM1(2,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY dimensional REAL array used for temporary storage, dimensioned FM1(2,N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY not applicable or not stated by selected source not a workspace argument

## 12. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is an INTEGER flag set to Zero       for normal return, 1, but no eigenvectors are computed. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `NM`: not a workspace argument
- `N`: not a workspace argument
- `AR`: not a workspace argument
- `AI`: not a workspace argument
- `W`: contains the eigenvalues in ascending order. dimensional REAL array, dimensioned W(N).
- `MATZ`: not a workspace argument
- `ZR`: not a workspace argument
- `ZI`: not a workspace argument
- `FV1`: not a workspace argument
- `FV2`: not a workspace argument
- `FM1`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ch`
- Original SLATEC routine: `CH`
- Native symbol: `ch_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`
- Exact Netlib source file: [CH](https://www.netlib.org/slatec/lin/ch.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
