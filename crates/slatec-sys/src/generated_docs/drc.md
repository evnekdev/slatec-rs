# Purpose

1. DRC Standard FORTRAN function routine Double precision version The routine calculates an approximation result to

# Description

This canonical unsafe binding exposes original SLATEC routine `DRC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DRC](https://www.netlib.org/slatec/src/drc.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Integral from zero to infinity of -1/2     -1 (t+Y)  dt, where X is nonnegative and Y is positive. integral from zero to infinity of -1/2     -1 (t+Y)  dt, where X is nonnegative and Y is positive.  The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order.  Logarithmic, inverse circular, and inverse hyper- bolic functions can be expressed in terms of DRC. 2.     Calling Sequence DRC( X, Y, IER ) Parameters On Entry Values assigned by the calling routine Double precision, nonnegative variable are unaltered. 3.    Error messages Value of IER assigned by the DRC routine Value assigned         Error message printed Z * Z Z * Z are the variables in the integral DRC(X,Y). are unaltered. DRC(0,1/4)=DRC(1/16,1/8)=PI=3.14159... DRC(9/4,2)=LN(2) WARNING: Changes in the program may improve speed at the expense of robustness. 1) DRC(((1+X)/2)  , X ) 1) DRC(((1+X)/2)  , X ) 1 .LE. X .LE. 1 2 X  ,1 ) X  ,1 ) X ) DRC(X  ,1 ) INF .LT. X .LT. +INF 2 X DRC(1,1+X  ) DRC(X  ,X +1 ) INF .LT. X .LT. +INF 2 X DRC(1+X  ,1 ) 1) DRC(X  ,1 ) 1) DRC(X  ,1 ) 1 .LT. X .LT. 1 2 X  ) X  ) 1 ) 1 ) 1 ) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Integral from zero to infinity of -1/2     -1 integral from zero to infinity of -1/2     -1 Double precision, positive variable On Return  (values assigned by the DRC routine) DRC    - Double precision approximation to the integral are unaltered. 3.    Error messages Value of IER assigned by the DRC routine Value assigned         Error message printed Z * Z Z * Z are the variables in the integral DRC(X,Y). are unaltered. DRC(0,1/4)=DRC(1/16,1/8)=PI=3.14159... DRC(9/4,2)=LN(2) WARNING: Changes in the program may improve speed at the expense of robustness. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer to indicate normal or abnormal termination. 0 Normal and reliable termination of the routine.  It is assumed that the requested accuracy has been achieved. > 0 Abnormal termination of the routine 1                X.LT.0.0D0.OR.Y.LE.0.0D0 = 2                X+Y.LT.LOLIM = 3                MAX(X,Y) .GT. UPLIM 4.     Control parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X and Y LOLIM  - Lower limit of valid arguments Not less  than 5 * (machine minimum)  . UPLIM  - Upper limit of valid arguments Not greater than (machine maximum) / 5 . Acceptable values for:   LOLIM       UPLIM IBM 360/370 SERIES   :   3.0D-78     1.0D+75 CDC 6000/7000 SERIES :   1.0D-292    1.0D+321 UNIVAC 1100 SERIES   :   1.0D-307    1.0D+307 CRAY                 :   2.3D-2466   1.0D+2465 VAX 11 SERIES        :   1.5D-38     3.0D+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL  - relative error due to truncation is less than 16 * ERRTOL ** 6 / (1 - 2 * ERRTOL). The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative truncation not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

1.0D-3    2.0D-17 3.0D-3    2.0D-14 1.0D-2    2.0D-11 3.0D-2    2.0D-8 1.0D-1    2.0D-5 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: DRC special comments Check: DRC(X,X+Z) + DRC(Y,Y+Z) = DRC(0,Z)

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::drc`
- Original SLATEC routine: `DRC`
- Native symbol: `drc_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DRC](https://www.netlib.org/slatec/src/drc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
