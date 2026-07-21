# Purpose

Abstract *** a DOUBLE PRECISION routine *** DGAUS8 integrates real functions of one variable over finite intervals using an adaptive 8-point Legendre-Gauss algorithm. DGAUS8 is intended primarily for high accuracy integration or integration of smooth functions. The maximum number of significant digits obtainable in ANS is the smaller of 18 and the number of digits carried in double precision arithmetic.

# Description

This canonical unsafe binding exposes original SLATEC routine `DGAUS8`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DGAUS8](https://www.netlib.org/slatec/src/dgaus8.f).

# Arguments

## 1. `FUN`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are DOUBLE PRECISION * name of external function to be integrated.  This name must be in an EXTERNAL statement in the calling program. must be a DOUBLE PRECISION function of one DOUBLE PRECISION argument.  The value of the argument to FUN is the variable of integration which ranges from A to B. are DOUBLE PRECISION * name of external function to be integrated.  This name must be in an EXTERNAL statement in the calling program. must be a DOUBLE PRECISION function of one DOUBLE PRECISION argument.  The value of the argument to FUN is the variable of integration which ranges from A to B. not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are DOUBLE PRECISION * lower limit of integration negative value for ERR causes an estimate of the absolute error in ANS to be returned in ERR.  Note that B. are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are DOUBLE PRECISION * upper limit of integration (may be less than A) are too nearly equal to allow normal integration.  ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ERR`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are DOUBLE PRECISION * is a requested pseudorelative error tolerance.  Normally pick a value of ABS(ERR) so that DTOL .LT. ABS(ERR) .LE. 1.0D-3 where DTOL is the larger of 1.0D-18 and the double precision unit roundoff D1MACH(4).  ANS will normally have no more error than ABS(ERR) times the integral of the absolute value of FUN(X).  Usually, smaller values of ERR yield more accuracy and require more function evaluations. must be a variable (not a constant) in this case. Note also that the user must reset the value of ERR before making any more calls that use the variable ERR. are double precision * will be an estimate of the absolute error in ANS if the is unchanged if is unchanged if negative.)  The estimated are DOUBLE PRECISION * is a requested pseudorelative error tolerance.  Normally pick a value of ABS(ERR) so that DTOL .LT. ABS(ERR) .LE. 1.0D-3 where DTOL is the larger of 1.0D-18 and the double precision unit roundoff D1MACH(4).  ANS will normally have no more error than ABS(ERR) times the integral of the absolute value of FUN(X).  Usually, smaller values of ERR yield more accuracy and require more function evaluations. must be a variable (not a constant) in this case. Note also that the user must reset the value of ERR before making any more calls that use the variable ERR. are double precision * will be an estimate of the absolute error in ANS if the is unchanged if is unchanged if negative.)  The estimated not applicable or not stated by selected source not a workspace argument

## 5. `ANS`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. are double precision * computed value of integral not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::quadrature::dgaus8`
- Original SLATEC routine: `DGAUS8`
- Native symbol: `dgaus8_`
- ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DGAUS8](https://www.netlib.org/slatec/src/dgaus8.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
