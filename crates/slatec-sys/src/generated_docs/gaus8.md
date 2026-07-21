# Purpose

GAUS8 integrates real functions of one variable over finite intervals using an adaptive 8-point Legendre-Gauss algorithm. GAUS8 is intended primarily for high accuracy integration or integration of smooth functions.

# Description

This canonical unsafe binding exposes original SLATEC routine `GAUS8`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GAUS8](https://www.netlib.org/slatec/src/gaus8.f).

# Arguments

## `FUN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

name of external function to be integrated. This name must be in an EXTERNAL statement in the calling program. must be a REAL function of one REAL argument. The value of the argument to FUN is the variable of integration which ranges from A to B. Usually, smaller values for ERR yield more accuracy and require more function evaluations. A negative value for ERR causes an estimate of the absolute error in ANS to be returned in ERR.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

lower limit of integration B. -1 A and B are too nearly equal to allow normal integration. ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

upper limit of integration (may be less than A).

## `ERR`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

is a requested pseudorelative error tolerance. Normally pick a value of ABS(ERR) so that STOL. LT. ABS(ERR). LE. 1.

## `ANS`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

computed value of integral.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

a status code --Normal codes 1 ANS most likely meets requested error tolerance,.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::gaus8`
- Original SLATEC routine: `GAUS8`
- Native symbol: `gaus8_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [GAUS8](https://www.netlib.org/slatec/src/gaus8.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
