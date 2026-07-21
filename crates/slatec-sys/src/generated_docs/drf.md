# Purpose

1. DRF Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the first kind Standard FORTRAN function routine Double precision version The routine calculates an approximation result to

# Description

This canonical unsafe binding exposes original SLATEC routine `DRF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DRF](https://www.netlib.org/slatec/src/drf.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 (t+Y)    (t+Z)    dt. is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 (t+Y)    (t+Z)    dt, are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind X DRF(1,1+KC X ,1+X ) Lemniscate constant A not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `Z`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. negative and at most one of Integral from zero to infinity of -1/2     -1/2     -1/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -1/2 are nonnegative and at most one of them is zero.  If one of them  is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling sequence DRF( X, Y, Z, IER ) Parameters On entry Values assigned by the calling routine Double precision, nonnegative variable On Return    (values assigned by the DRF routine) DRF     - Double precision approximation to the integral are unaltered. 3.    Error Messages Value of IER assigned by the DRF routine Value assigned         Error Message Printed are positive and X * Y = Z * W. are positive and X * Y = Z * W. are the variables in the integral DRF(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. Special double precision functions via DRF Legendre form of ELLIPTIC INTEGRAL of 1st kind not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. >  0 Abnormal termination of the routine 1                MIN(X,Y,Z) .LT. 0.0D0 = 2                MIN(X+Y,X+Z,Y+Z) .LT. LOLIM = 3                MAX(X,Y,Z) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y and Z LOLIM  - Lower limit of valid arguments Not less than 5 * (machine minimum). UPLIM  - Upper limit of valid arguments Not greater than (machine maximum) / 5. Acceptable values for:   LOLIM      UPLIM IBM 360/370 SERIES   :   3.0D-78     1.0D+75 CDC 6000/7000 SERIES :   1.0D-292    1.0D+321 UNIVAC 1100 SERIES   :   1.0D-307    1.0D+307 CRAY                 :   2.3D-2466   1.09D+2465 VAX 11 SERIES        :   1.5D-38     3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than ERRTOL ** 6 / (4 * (1-ERRTOL)  . The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the truncation not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative Truncation 1.0D-3    3.0D-19 3.0D-3    2.0D-16 1.0D-2    3.0D-13 3.0D-2    2.0D-10 1.0D-1    3.0D-7 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: DRF Special Comments Check by addition theorem: DRF(X,X+Z,X+W) + DRF(Y,Y+Z,Y+W)

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `Z`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::drf`
- Original SLATEC routine: `DRF`
- Native symbol: `drf_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DRF](https://www.netlib.org/slatec/src/drf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
