# Purpose

Written BY L. F. Shampine and S. M. Davenport. POLFIT computes the least squares polynomial fit of degree L as

# Description

This canonical unsafe binding exposes original SLATEC routine `PCOEF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [PCOEF](https://www.netlib.org/slatec/src/pcoef.f).

# Arguments

## 1. `L`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Indicates the degree of polynomial to be changed to its Taylor expansion.  To obtain the Taylor coefficients in reverse order, input  L  as the negative of the degree desired.  The absolute value of L  must be less than or equal to NDEG, the highest degree polynomial fitted by  POLFIT . Indicates the degree of polynomial to be changed to its Taylor expansion.  To obtain the Taylor coefficients in reverse order, input  L  as the negative of the degree desired.  The absolute value of L  must be less than or equal to NDEG, the highest degree polynomial fitted by  POLFIT . not applicable or not stated by selected source not a workspace argument

## 2. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. The point about which the Taylor expansion is to be made. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `TC`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Vector containing the first LL+1 Taylor coefficients where LL=ABS(L).  If  L.GT.0 , the coefficients are in the usual Taylor series order, i.e. C) + ... + TC(N+1)*(X-C)**N C) + ... + TC(N+1)*(X-C)**N If L .LT. 0, the coefficients are in reverse order, If L .LT. 0, the coefficients are in reverse order, i.e. i.e. C)**N + ... + TC(N)*(X-C) + TC(N+1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). sum of orthogonal polynomials.  PCOEF  changes this fit to its Taylor expansion about any point  C , i.e. writes the polynomial C).  Taking  C=0.  gives the polynomial zero  C  often leads to polynomials which are better scaled and more accurately evaluated. The parameters for  PCOEF  are Work and output array containing values from last call to  POLFIT . not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `L`: not a workspace argument
- `C`: not a workspace argument
- `TC`: not a workspace argument
- `A`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::approximation::pcoef`
- Original SLATEC routine: `PCOEF`
- Native symbol: `pcoef_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [PCOEF](https://www.netlib.org/slatec/src/pcoef.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
