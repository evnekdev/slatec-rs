# Purpose

Find the zeros of the real polynomial

# Description

This canonical unsafe binding exposes original SLATEC routine `RPZERO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RPZERO](https://www.netlib.org/slatec/src/rpzero.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 1) +...+ A(N+1) 1) +...+ A(N+1) degree of P(X) I) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). 1) +...+ A(N+1) 1) +...+ A(N+1) real vector containing coefficients of P(X), I) 0.0 or N=0 on input. If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `R`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). N word complex vector containing initial estimates for zeros if these are known. contains estimates of the zeros Warning ****** If estimates are input, they must be separated; that is, distinct or not repeated. ith zero, N word complex vector containing initial estimates for zeros if these are known. contains estimates of the zeros Warning ****** If estimates are input, they must be separated; that is, distinct or not repeated. ith zero, not applicable or not stated by selected source not a workspace argument

## 4. `T`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). 6(N+1) word array used for temporary storage not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IFLG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. flag to indicate if initial estimates of zeros are input. If IFLG .EQ. 0, no estimates are input. contains estimates of the zeros Warning ****** If estimates are input, they must be separated; that is, distinct or not repeated. error diagnostic 0.0 or N=0 on input. If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. flag to indicate if initial estimates of zeros are input. If IFLG .EQ. 0, no estimates are input. contains estimates of the zeros Warning ****** If estimates are input, they must be separated; that is, distinct or not repeated. error diagnostic 0.0 or N=0 on input. If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. not applicable or not stated by selected source not a workspace argument

## 6. `S`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). an N word array bound for R(I) . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

If IFLG .EQ. 0 on return, all is well.

# Workspace and array requirements

- `N`: not a workspace argument
- `A`: not a workspace argument
- `R`: not a workspace argument
- `T`: not a workspace argument
- `IFLG`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::roots::complex::rpzero`
- Original SLATEC routine: `RPZERO`
- Native symbol: `rpzero_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [RPZERO](https://www.netlib.org/slatec/src/rpzero.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
