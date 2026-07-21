# Purpose

1. RJ Standard FORTRAN function routine Single precision version The routine calculates an approximation result to

# Description

This canonical unsafe binding exposes original SLATEC routine `RJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RJ](https://www.netlib.org/slatec/src/rj.f).

# Arguments

## 1. `X`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is zero) negative, at most one of them zero, and P positive, Integral from zero to infinity of -1/2     -1/2     -1/2     -1 (t+Y)    (t+Z)    (t+P)  dt. Integral from zero to infinity of -1/2     -1/2     -1/2     -1 (t+Y)    (t+Z)    (t+P)  dt, are nonnegative, at most one of them is zero, and P is positive.  If X or Y or Z is zero, the integral is COMPLETE.  The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence RJ( X, Y, Z, P, IER ) Parameters On Entry Values assigned by the calling routine Single precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the RJ routine Value Assigned        Error Message Printed are positive and X * Y = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W).  The sum of the third and fourth terms on the left side is 3 * RC(A,B). are the variables in the integral RJ(X,Y,Z,P). are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. X RF(1,1+KC X ,1+X ) + 3          2 2    2     2 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `Y`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is zero) negative, at most one of them zero, and P positive, Integral from zero to infinity of -1/2     -1/2     -1/2     -1 Integral from zero to infinity of -1/2     -1/2     -1/2     -1 are nonnegative, at most one of them is zero, and P is positive.  If X or Y or Z is zero, the integral is COMPLETE.  The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence RJ( X, Y, Z, P, IER ) Parameters On Entry Values assigned by the calling routine Single precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the RJ routine Value Assigned        Error Message Printed B) * RJ(A,B,B,A) + 3 / SQRT(A) B) * RJ(A,B,B,A) + 3 / SQRT(A) B) * RJ(A,B,B,A) + 3 / SQRT(A) B) * RJ(A,B,B,A) + 3 / SQRT(A) are positive and X * Y = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W).  The sum of the third and fourth terms on the left side is 3 * RC(A,B). are the variables in the integral RJ(X,Y,Z,P). are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `Z`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. is zero) negative, at most one of them zero, and P positive, Integral from zero to infinity of -1/2     -1/2     -1/2     -1 Integral from zero to infinity of -1/2     -1/2     -1/2     -1 are nonnegative, at most one of them is zero, and P is positive.  If X or Y or Z is zero, the integral is COMPLETE.  The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. 2.     Calling Sequence RJ( X, Y, Z, P, IER ) Parameters On Entry Values assigned by the calling routine Single precision, nonnegative variable are unaltered. 3.    Error Messages Value of IER assigned by the RJ routine Value Assigned        Error Message Printed B) * RJ(A,B,B,A) + 3 / SQRT(A) are positive and X * Y are positive and X * Y = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W).  The sum of the third and and B - A = P * (P-Z) * (P-W).  The sum of the third and fourth terms on the left side is 3 * RC(A,B). fourth terms on the left side is 3 * RC(A,B). are the variables in the integral RJ(X,Y,Z,P). are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. K SIN (B)) 2      2   2                2 RJ(0,1-K ,1,1-K SIN (B)) / RF (0,1-K ,1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `P`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Integral from zero to infinity of -1/2     -1/2     -1/2     -1 Integral from zero to infinity of -1/2     -1/2     -1/2     -1 Single precision, positive variable On  Return     (values assigned by the RJ routine) RJ     - Single precision approximation to the integral are unaltered. 3.    Error Messages Value of IER assigned by the RJ routine Value Assigned        Error Message Printed B) * RJ(A,B,B,A) + 3 / SQRT(A) are positive and X * Y are positive and X * Y = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), = Z * W,  A = P * P * (X+Y+Z+W),  B = P * (P+X) * (P+Y), and B - A = P * (P-Z) * (P-W).  The sum of the third and and B - A = P * (P-Z) * (P-W).  The sum of the third and fourth terms on the left side is 3 * RC(A,B). fourth terms on the left side is 3 * RC(A,B). are the variables in the integral RJ(X,Y,Z,P). are unaltered. Warning: Changes in the program may improve speed at the expense of robustness. INT (1+N SIN (THETA) )   * 0 2    2         -1/2 (1-K  SIN (THETA) )     D THETA 2          2   2 = SIN (PHI) RF(COS (PHI), 1-K SIN (PHI),1) 3            2         2   2 -(N/3) SIN (PHI) RJ(COS (PHI),1-K SIN (PHI), 2 1,1+N SIN (PHI)) Bulirsch form of ELLIPTIC INTEGRAL of 3rd kind X RF(1,1+KC X ,1+X ) + 3          2 2    2     2 X  RJ(1,1+KC X ,1+X ,1+PX ) 2 A RF(0,KC ,1) + 2 +(1/3)(B-PA) RJ(0,KC ,1,P) Heuman's LAMBDA function COS (A)SIN (B))   ) 2         2       2 SIN (A) SIN (P),1) SIN (A) SIN (P),1) 2       3            2       2 2       3            2       2 COS (A) SIN (B)))) 2         2       2 SIN (A) SIN (P),1,1- 2       2          2       2 -SIN (A) SIN (P)/(1-COS (A) SIN (B)))) (PI/2) LAMBDA0(A,B) =L(A,B,PI/2) = 2                         2       2    -1/2 = COS (A)  SIN(B) COS(B) (1-COS (A) SIN (B)) 2                  2       2 RF(0,COS (A),1) + (1/3) SIN (A) COS (A) 2       2    -3/2 SIN(B) COS(B) (1-COS (A) SIN (B)) 2         2       2          2       2 RJ(0,COS (A),1,COS (A) COS (B)/(1-COS (A) SIN (B))) Jacobi ZETA function not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer 0 Normal and reliable termination of the routine.  It is assumed that the requested accuracy has been achieved. >  0 Abnormal termination of the routine 1               MIN(X,Y,Z) .LT. 0.0E0 = 2               MIN(X+Y,X+Z,Y+Z,P) .LT. LOLIM = 3               MAX(X,Y,Z,P) .GT. UPLIM 4.     Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X Y, Z, and P LOLIM is not less than the cube root of the value of LOLIM used in the routine for RC. UPLIM is not greater than 0.3 times the cube root of the value of UPLIM used in the routine for RC. Acceptable Values For:   LOLIM      UPLIM IBM 360/370 SERIES   :   2.0E-26     3.0E+24 CDC 6000/7000 SERIES :   5.0E-98     3.0E+106 UNIVAC 1100 SERIES   :   5.0E-13     6.0E+11 CRAY                 :   1.32E-822   1.4E+821 VAX 11 SERIES        :   2.5E-13     9.0E+11 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". Relative error due to truncation of the series for RJ is less than 3 * ERRTOL ** 6 / (1 - ERRTOL) ** 3/2. The accuracy of the computed approximation to the inte- gral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order Introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column.  In addition to the trunca- tion error there will be round-off error, but in prac- tice the total error from both sources is usually less than the amount given in the table. Sample choices:  ERRTOL   Relative Truncation not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This Fortran function returns its scalar result using the compiler-validated ABI fingerprint `function:f32(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`. It has no separate Rust `Result` status channel.

# Callback contract

This interface has no callback argument.

# Status and error values

1.0E-3    4.0E-18 3.0E-3    3.0E-15 1.0E-2    4.0E-12 3.0E-2    3.0E-9 1.0E-1    4.0E-6 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. Long Description: RJ Special Comments Check by addition theorem: RJ(X,X+Z,X+W,X+P)

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `Z`: not a workspace argument
- `P`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::rj`
- Original SLATEC routine: `RJ`
- Native symbol: `rj_`
- ABI fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`
- Exact Netlib source file: [RJ](https://www.netlib.org/slatec/src/rj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
