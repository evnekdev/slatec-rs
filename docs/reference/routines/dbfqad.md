# DBFQAD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the integral of a product of a function and a derivative of a K-th order B-spline.

## Description

Abstract **** a double precision routine **** DBFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a K-th order B-spline, using the B-representation (T,BCOEF,N,K). (X1,X2) must be a subinterval of T(K) .LE. X .LE. T(N+1). An integration routine, DBSGQ8 (a modification of GAUS8), integrates the product on subintervals of (X1,X2) formed by included (distinct) knots The maximum number of significant digits obtainable in DBSQAD is the smaller of 18 and the number of digits carried in double precision arithmetic. Description of Arguments Input F,T,BCOEF,X1,X2,TOL are double precision F - external function of one argument for the integrand BF(X)=F(X)*DBVALU(T,BCOEF,N,K,ID,X,INBV, WORK) T - knot array of length N+K BCOEF - coefficient array of length N N - length of coefficient array K - order of B-spline, K .GE. 1 ID - order of the spline derivative, 0 .LE. ID .LE. K-1 ID=0 gives the spline function X1,X2 - end points of quadrature interval in T(K) .LE. X .LE. T(N+1) TOL - desired accuracy for the quadrature, suggest 10.*DTOL .LT. TOL .LE. .1 where DTOL is the maximum of 1.0D-18 and double precision unit roundoff for the machine = D1MACH(4) Output QUAD,WORK are double precision QUAD - integral of BF(X) on (X1,X2) IERR - a status code IERR=1 normal return 2 some quadrature on (X1,X2) does not meet the requested tolerance. WORK - work vector of length 3*K Error Conditions Improper input is a fatal error Some quadrature fails to meet the requested tolerance

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbfqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbfqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbfqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbfqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
