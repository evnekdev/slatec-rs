# Purpose

Find the zeros of the complex polynomial

# Description

This canonical unsafe binding exposes original SLATEC routine `CPZERO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPZERO](https://www.netlib.org/slatec/src/cpzero.f).

# Arguments

## 1. `IN`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. degree of P(Z) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). 1) +...+ A(N+1) 1) +...+ A(N+1) complex vector containing coefficients of P(Z), i) 0.0 or N=0 on input If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `R`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). N word complex vector containing initial estimates for zeros if these are known. contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated. Ith zero, N word complex vector containing initial estimates for zeros if these are known. contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated. Ith zero, not applicable or not stated by selected source not a workspace argument

## 4. `T`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). 4(N+1) word array used for temporary storage not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IFLG`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. flag to indicate if initial estimates of zeros are input. If IFLG .EQ. 0, no estimates are input. contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated. error diagnostic 0.0 or N=0 on input If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. flag to indicate if initial estimates of zeros are input. If IFLG .EQ. 0, no estimates are input. contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated. error diagnostic 0.0 or N=0 on input If IFLG .EQ. 2 on return, the program failed to converge after 25*N iterations.  Best current estimates of the zeros are in R(I).  Error bounds are not calculated. not applicable or not stated by selected source not a workspace argument

## 6. `S`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). an N word array bound for R(I) . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

If IFLG .EQ. 0 on return, all is well

# Workspace and array requirements

- `IN`: not a workspace argument
- `A`: not a workspace argument
- `R`: not a workspace argument
- `T`: not a workspace argument
- `IFLG`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::roots::complex::cpzero`
- Original SLATEC routine: `CPZERO`
- Native symbol: `cpzero_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [CPZERO](https://www.netlib.org/slatec/src/cpzero.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
