# Purpose

AVINT integrates a function tabulated at arbitrarily spaced abscissas. The limits of integration need not coincide with the tabulated abscissas. A method of overlapping parabolas fitted to the data is used provided that there are at least 3 abscissas between the limits of integration. AVINT also handles two special cases. If the limits of integration are equal, AVINT returns a result of zero regardless of the number of tabulated values. If there are only two function values, AVINT uses the trapezoid rule. Description of Parameters The user must dimension all arrays appearing in the call list

# Description

This canonical unsafe binding exposes original SLATEC routine `AVINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [AVINT](https://www.netlib.org/slatec/src/avint.f).

# Arguments

## 1. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Y(N). real array of abscissas, which must be in increasing order. Y(N). real array of abscissas, which must be in increasing order. not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). real array of functional values. i.e., Y(I)=FUNC(X(I)). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Y(N). the integer number of function values supplied. XUP. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `XLO`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. XUP. real lower limit of integration. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `XUP`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. real upper limit of integration. Must have XLO .LE. XUP. real upper limit of integration. Must have XLO .LE. XUP. not applicable or not stated by selected source not a workspace argument

## 6. `ANS`

output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computed approximate value of integral 2,3,4,or 5. AVINT is documented completely in SC-M-69-335 Original program from "Numerical Integration" by Davis & Rabinowitz. Adaptation and modifications for Sandia Mathematical Program not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `IERR`

output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a status code --normal code =1 means the requested integration was performed. --abnormal codes =2 means XUP was less than XLO. =3 means the number of X(I) between XLO and XUP (inclusive) was less than 3 and neither of the two special cases described in the Abstract occurred. No integration was performed. =4 means the restriction X(I+1) .GT. X(I) was violated. =5 means the number N of function values was .LT. 2. 2,3,4,or 5. AVINT is documented completely in SC-M-69-335 Original program from "Numerical Integration" by Davis & Rabinowitz. Adaptation and modifications for Sandia Mathematical Program not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `N`: not a workspace argument
- `XLO`: not a workspace argument
- `XUP`: not a workspace argument
- `ANS`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::avint`
- Original SLATEC routine: `AVINT`
- Native symbol: `avint_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [AVINT](https://www.netlib.org/slatec/src/avint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
