# DXNRMP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials.

## Description

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (XNRMP is single-precision version) DXNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree NU are non-negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Software, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in DXSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to DXNRMP are NU, MU1, MU2, DARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. 2. MU1 .GE. 0 specifies the lowest-order normalized Legendre polynomial that is wanted. 3. MU2 .GE. MU1 specifies the highest-order normalized Legendre polynomial that is wanted. 4a. MODE = 1 and -1.0D0 .LE. DARG .LE. 1.0D0 specifies that Normalized Legendre(NU, MU, DARG) is wanted for MU = MU1, MU1 + 1, ..., MU2. 4b. MODE = 2 and -3.14159... .LT. DARG .LT. 3.14159... specifies that Normalized Legendre(NU, MU, COS(DARG)) is wanted for MU = MU1, MU1 + 1, ..., MU2. The output of DXNRMP consists of the two vectors DPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that (DPN(1),IPN(1))=NORMALIZED LEGENDRE(NU,MU1,DX) (DPN(2),IPN(2))=NORMALIZED LEGENDRE(NU,MU1+1,DX) . . (DPN(K),IPN(K))=NORMALIZED LEGENDRE(NU,MU2,DX) where K = MU2 - MU1 + 1 and DX = DARG or COS(DARG) according to whether MODE = 1 or 2. Finally, ISIG is an estimate of the number of decimal digits lost through rounding errors in the computation. For example if DARG is accurate to 12 significant decimals, then the computed function values are accurate to 12 - ISIG significant decimals (except in neighborhoods of zeros). The interpretation of (DPN(I),IPN(I)) is DPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When IPN(I) = 0 the value of the normalized Legendre polynomial is contained entirely in DPN(I) and subsequent double-precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corresponding value of the normalized Legendre polynomial cannot be represented in double-precision because of overflow or underflow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user could rewrite his/her program to use extended range arithmetic. The interpretation of (DPN(I),IPN(I)) can be changed to DPN(I)*(10**IPN(I)) by calling the extended-range subroutine DXCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL DXCON(DPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, DPN(I), IPN(I) 10 FORMAT(1X, D30.18 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J such that IPN(1) = IPN(2) = ... = IPN(J) = 0. Because of the change of representation caused by calling DXCON, (DPN(I), IPN(I)) for I = J+1, J+2, ... cannot be used in subsequent extended-range computations. IERROR is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=212 or 213, invalid input was provided to DXNRMP. If IERROR=201,202,203, or 204, invalid input was provided to DXSET. If IERROR=205 or 206, an internal consistency error occurred in DXSET (probably due to a software malfunction in the

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C3A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dxnrmp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxnrmp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxnrmp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dxnrmp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
