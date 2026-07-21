# Purpose

1. DRD Evaluate an INCOMPLETE (or COMPLETE) ELLIPTIC INTEGRAL of the second kind Standard FORTRAN function routine Double precision version The routine calculates an approximation result to

# Description

This canonical unsafe binding exposes original SLATEC routine `DRD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DRD](https://www.netlib.org/slatec/src/drd.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Integral from zero to infinity of -1/2     -1/2     -3/2 (t+Y)    (t+Z)    dt. is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -3/2 (t+Y)    (t+Z)    dt, is positive, and Z is is positive, and Z is is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence DRD( X, Y, Z, IER ) Parameters On Entry Values assigned by the calling routine Double precision, nonnegative variable is positive are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. AX DRF(1,1+KC X ,1+X ) + 3          2 2    2 +(1/3)(B-A) X DRD(1,1+KC X ,1+X ) Legendre form of alternative ELLIPTIC INTEGRAL of 2nd kind not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Integral from zero to infinity of -1/2     -1/2     -3/2 is zero, the integral is complete. Integral from zero to infinity of -1/2     -1/2     -3/2 is positive, and Z is is positive, and Z is is zero, the integral is COMPLETE. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence DRD( X, Y, Z, IER ) Parameters On Entry Values assigned by the calling routine Double precision, nonnegative variable is positive are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `Z`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Integral from zero to infinity of -1/2     -1/2     -3/2 Integral from zero to infinity of -1/2     -1/2     -3/2 Double precision, positive variable On Return    (values assigned by the DRD routine) DRD     - Double precision approximation to the integral are unaltered. 3.    Error Messages Value of IER assigned by the DRD routine Value assigned         Error message printed are positive. are positive. are the variables in the integral DRD(X,Y,Z). are unaltered. WARNING: Changes in the program may improve speed at the expense of robustness. K SIN (B),1) 2             2 DRD(0,1-K ,1)/DRF(0,1-K ,1) 2       3           2       2   2 -(K /3) SIN (B) DRD(COS (B),1-K SIN (B),1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. >  0 Abnormal termination of the routine 1                MIN(X,Y) .LT. 0.0D0 = 2                MIN(X + Y, Z ) .LT. LOLIM = 3                MAX(X,Y,Z) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X, Y, and Z LOLIM  - Lower limit of valid arguments Not less  than 2 / (machine maximum) ** (2/3). UPLIM  - Upper limit of valid arguments Not greater than (0.1D0 * ERRTOL / machine minimum) ** (2/3), where ERRTOL is described below. In the following table it is assumed that ERRTOL will never be chosen smaller than 1.0D-5. Acceptable values for:   LOLIM      UPLIM IBM 360/370 SERIES   :   6.0D-51     1.0D+48 CDC 6000/7000 SERIES :   5.0D-215    2.0D+191 UNIVAC 1100 SERIES   :   1.0D-205    2.0D+201 CRAY                 :   3.0D-1644   1.69D+1640 VAX 11 SERIES        :   1.0D-25     4.5D+21 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL    Relative error due to truncation is less than 3 * ERRTOL ** 6 / (1-ERRTOL) ** 3/2. The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the truncation not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative truncation 1.0D-3    4.0D-18 3.0D-3    3.0D-15 1.0D-2    4.0D-12 3.0D-2    3.0D-9 1.0D-1    4.0D-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: DRD Special Comments Check: DRD(X,Y,Z) + DRD(Y,Z,X) + DRD(Z,X,Y)

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `Z`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::drd`
- Original SLATEC routine: `DRD`
- Native symbol: `drd_`
- ABI fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`
- Exact Netlib source file: [DRD](https://www.netlib.org/slatec/src/drd.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
