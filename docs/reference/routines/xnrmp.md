# XNRMP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute normalized Legendre polynomials.

## Description

SUBROUTINE TO CALCULATE NORMALIZED LEGENDRE POLYNOMIALS (DXNRMP is double-precision version) XNRMP calculates normalized Legendre polynomials of varying order and fixed argument and degree. The order MU and degree NU are non-negative integers and the argument is real. Because the algorithm requires the use of numbers outside the normal machine range, this subroutine employs a special arithmetic called extended-range arithmetic. See J.M. Smith, F.W.J. Olver, and D.W. Lozier, Extended-Range Arithmetic and Normalized Legendre Polynomials, ACM Transactions on Mathematical Software, 93-105, March 1981, for a complete description of the algorithm and special arithmetic. Also see program comments in XSET. The normalized Legendre polynomials are multiples of the associated Legendre polynomials of the first kind where the normalizing coefficients are chosen so as to make the integral from -1 to 1 of the square of each function equal to 1. See E. Jahnke, F. Emde and F. Losch, Tables of Higher Functions, McGraw-Hill, New York, 1960, p. 121. The input values to XNRMP are NU, MU1, MU2, SARG, and MODE. These must satisfy 1. NU .GE. 0 specifies the degree of the normalized Legendre polynomial that is wanted. 2. MU1 .GE. 0 specifies the lowest-order normalized Legendre polynomial that is wanted. 3. MU2 .GE. MU1 specifies the highest-order normalized Legendre polynomial that is wanted. 4a. MODE = 1 and -1.0 .LE. SARG .LE. 1.0 specifies that Normalized Legendre(NU, MU, SARG) is wanted for MU = MU1, MU1 + 1, ..., MU2. 4b. MODE = 2 and -3.14159... .LT. SARG .LT. 3.14159... specifies that Normalized Legendre(NU, MU, COS(SARG)) is wanted for MU = MU1, MU1 + 1, ..., MU2. The output of XNRMP consists of the two vectors SPN and IPN and the error estimate ISIG. The computed values are stored as extended-range numbers such that (SPN(1),IPN(1))=NORMALIZED LEGENDRE(NU,MU1,X) (SPN(2),IPN(2))=NORMALIZED LEGENDRE(NU,MU1+1,X) . . (SPN(K),IPN(K))=NORMALIZED LEGENDRE(NU,MU2,X) where K = MU2 - MU1 + 1 and X = SARG or COS(SARG) according to whether MODE = 1 or 2. Finally, ISIG is an estimate of the number of decimal digits lost through rounding errors in the computation. For example if SARG is accurate to 12 significant decimals, then the computed function values are accurate to 12 - ISIG significant decimals (except in neighborhoods of zeros). The interpretation of (SPN(I),IPN(I)) is SPN(I)*(IR**IPN(I)) where IR is the internal radix of the computer arithmetic. When IPN(I) = 0 the value of the normalized Legendre polynomial is contained entirely in SPN(I) and subsequent single-precision computations can be performed without further consideration of extended-range arithmetic. However, if IPN(I) .NE. 0 the corresponding value of the normalized Legendre polynomial cannot be represented in single-precision because of overflow or underflow. THE USER MUST TEST IPN(I) IN HIS/HER PROGRAM. In the case that IPN(I) is nonzero, the user should try using double precision if it has a wider exponent range. If double precision fails, the user could rewrite his/her program to use extendedrange arithmetic. The interpretation of (SPN(I),IPN(I)) can be changed to SPN(I)*(10**IPN(I)) by calling the extended-range subroutine XCON. This should be done before printing the computed values. As an example of usage, the Fortran coding J = K DO 20 I = 1, K CALL XCON(SPN(I), IPN(I),IERROR) IF (IERROR.NE.0) RETURN PRINT 10, SPN(I), IPN(I) 10 FORMAT(1X, E30.8 , I15) IF ((IPN(I) .EQ. 0) .OR. (J .LT. K)) GO TO 20 J = I - 1 20 CONTINUE will print all computed values and determine the largest J such that IPN(1) = IPN(2) = ... = IPN(J) = 0. Because of the change of representation caused by calling XCON, (SPN(I), IPN(I)) for I = J+1, J+2, ... cannot be used in subsequent extended-range computations. IERROR is an error indicator. If no errors are detected, IERROR=0 when control returns to the calling routine. If an error is detected, IERROR is returned as nonzero. The calling routine must check the value of IERROR. If IERROR=112 or 113, invalid input was provided to XNRMP. If IERROR=101,102,103, or 104, invalid input was provided to XSET. If IERROR=105 or 106, an internal consistency error occurred in XSET (probably due to a software malfunction in the

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/xnrmp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xnrmp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xnrmp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xnrmp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
