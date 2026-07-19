# RC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate an approximation to RC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive.

## Description

1. RC Standard FORTRAN function routine Single precision version The routine calculates an approximation to RC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive. The duplication theorem is iterated until the variables are nearly equal, and the function is then expanded in Taylor series to fifth order. Logarithmic, inverse circular, and inverse hyperbolic functions can be expressed in terms of RC. 2. Calling Sequence RC( X, Y, IER ) Parameters on Entry Values assigned by the calling routine X - Single precision, nonnegative variable Y - Single precision, positive variable On Return (values assigned by the RC routine) RC - Single precision approximation to the integral IER - Integer to indicate normal or abnormal termination. IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER > 0 Abnormal termination of the routine X and Y are unaltered. 3. Error Messages Value of IER assigned by the RC routine Value Assigned Error Message Printed IER = 1 X.LT.0.0E0.OR.Y.LE.0.0E0 = 2 X+Y.LT.LOLIM = 3 MAX(X,Y) .GT. UPLIM 4. Control Parameters Values of LOLIM, UPLIM, and ERRTOL are set by the routine. LOLIM and UPLIM determine the valid range of X and Y LOLIM - Lower limit of valid arguments Not less than 5 * (machine minimum) . UPLIM - Upper limit of valid arguments Not greater than (machine maximum) / 5 . Acceptable values for: LOLIM UPLIM IBM 360/370 SERIES : 3.0E-78 1.0E+75 CDC 6000/7000 SERIES : 1.0E-292 1.0E+321 UNIVAC 1100 SERIES : 1.0E-37 1.0E+37 CRAY : 2.3E-2466 1.09E+2465 VAX 11 SERIES : 1.5E-38 3.0E+37 ERRTOL determines the accuracy of the answer The value assigned by the routine will result in solution precision within 1-2 decimals of "machine precision". ERRTOL - Relative error due to truncation is less than 16 * ERRTOL ** 6 / (1 - 2 * ERRTOL). The accuracy of the computed approximation to the integral can be controlled by choosing the value of ERRTOL. Truncation of a Taylor series after terms of fifth order introduces an error less than the amount shown in the second column of the following table for each value of ERRTOL in the first column. In addition to the truncation error there will be round-off error, but in practice the total error from both sources is usually less than the amount given in the table. Sample Choices: ERRTOL Relative Truncation error less than 1.0E-3 2.0E-17 3.0E-3 2.0E-14 1.0E-2 2.0E-11 3.0E-2 2.0E-8 1.0E-1 2.0E-5 Decreasing ERRTOL by a factor of 10 yields six more decimal digits of accuracy at the expense of one or two more iterations of the duplication theorem. *Long Description: RC Special Comments Check: RC(X,X+Z) + RC(Y,Y+Z) = RC(0,Z) where X, Y, and Z are positive and X * Y = Z * Z On Input: X and Y are the variables in the integral RC(X,Y). On Output: X and Y are unaltered. RC(0,1/4)=RC(1/16,1/8)=PI=3.14159... RC(9/4,2)=LN(2) ******************************************************** Warning: Changes in the program may improve speed at the expense of robustness. Special Functions via RC LN X X .GT. 0 2 LN(X) = (X-1) RC(((1+X)/2) , X ) ARCSIN X -1 .LE. X .LE. 1 2 ARCSIN X = X RC (1-X ,1 ) ARCCOS X 0 .LE. X .LE. 1 2 2 ARCCOS X = SQRT(1-X ) RC(X ,1 ) ARCTAN X -INF .LT. X .LT. +INF 2 ARCTAN X = X RC(1,1+X ) ARCCOT X 0 .LE. X .LT. INF 2 2 ARCCOT X = RC(X ,X +1 ) ARCSINH X -INF .LT. X .LT. +INF 2 ARCSINH X = X RC(1+X ,1 ) ARCCOSH X X .GE. 1 2 2 ARCCOSH X = SQRT(X -1) RC(X ,1 ) ARCTANH X -1 .LT. X .LT. 1 2 ARCTANH X = X RC(1,1-X ) ARCCOTH X X .GT. 1 2 2 ARCCOTH X = RC(X ,X -1 )

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C14`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::scalar_expanded::carlson_rc_f32`

## Providers

- Canonical provider: `main-src/src/rc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/rc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/rc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/rc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
