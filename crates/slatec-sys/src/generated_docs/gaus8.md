# Purpose

GAUS8 integrates real functions of one variable over finite intervals using an adaptive 8-point Legendre-Gauss algorithm. GAUS8 is intended primarily for high accuracy integration or integration of smooth functions.

# Description

This canonical unsafe binding exposes original SLATEC routine `GAUS8`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAUS8](https://www.netlib.org/slatec/src/gaus8.f).

# Arguments

## 1. `FUN`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. name of external function to be integrated.  This name must be in an EXTERNAL statement in the calling program. must be a REAL function of one REAL argument.  The value of the argument to FUN is the variable of integration which ranges from A to B. Usually, smaller values for ERR yield more accuracy and require more function evaluations. name of external function to be integrated.  This name must be in an EXTERNAL statement in the calling program. must be a REAL function of one REAL argument.  The value of the argument to FUN is the variable of integration which ranges from A to B. Usually, smaller values for ERR yield more accuracy and require more function evaluations. not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. lower limit of integration negative value for ERR causes an estimate of the absolute error in ANS to be returned in ERR.  Note that B. are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. upper limit of integration (may be less than A) are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ERR`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is a requested pseudorelative error tolerance.  Normally pick a value of ABS(ERR) so that STOL .LT. ABS(ERR) .LE. 1.0E-3 where STOL is the single precision unit roundoff R1MACH(4).  ANS will normally have no more error than times the integral of the absolute value of must be a variable (not a constant) in this case. Note also that the user must reset the value of ERR before making any more calls that use the variable ERR. will be an estimate of the absolute error in ANS if the is unchanged if is unchanged if negative.)  The estimated is a requested pseudorelative error tolerance.  Normally pick a value of ABS(ERR) so that STOL .LT. ABS(ERR) .LE. 1.0E-3 where STOL is the single precision unit roundoff R1MACH(4).  ANS will normally have no more error than times the integral of the absolute value of must be a variable (not a constant) in this case. Note also that the user must reset the value of ERR before making any more calls that use the variable ERR. will be an estimate of the absolute error in ANS if the is unchanged if is unchanged if negative.)  The estimated not applicable or not stated by selected source not a workspace argument

## 5. `ANS`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. computed value of integral not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. a status code --Normal codes 1 ANS most likely meets requested error tolerance, not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

not be used as a correction to the computed integral.

# Workspace and array requirements

- `FUN`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `ERR`: not a workspace argument
- `ANS`: not a workspace argument
- `IERR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::gaus8`
- Original SLATEC routine: `GAUS8`
- Native symbol: `gaus8_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [GAUS8](https://www.netlib.org/slatec/src/gaus8.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
