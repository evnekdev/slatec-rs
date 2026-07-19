# DPFQAD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the integral on (X1,X2) of a product of a function F and the ID-th derivative of a B-spline, (PP-representation).

## Description

Abstract **** a double precision routine **** DPFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a B-spline, using the PP-representation (C,XI,LXI,K). (X1,X2) is normally a subinterval of XI(1) .LE. X .LE. XI(LXI+1). An integration routine, DPPGQ8 (a modification of GAUS8), integrates the product on subintervals of (X1,X2) formed by the included break points. Integration outside of (XI(1),XI(LXI+1)) is permitted provided F is defined. The maximum number of significant digits obtainable in DBSQAD is the smaller of 18 and the number of digits carried in double precision arithmetic. Description of arguments Input F,C,XI,X1,X2,TOL are double precision F - external function of one argument for the integrand PF(X)=F(X)*DPPVAL(LDC,C,XI,LXI,K,ID,X, INPPV) LDC - leading dimension of matrix C, LDC .GE. K C(I,J) - right Taylor derivatives at XI(J), I=1,K , J=1,LXI XI(*) - break point array of length LXI+1 LXI - number of polynomial pieces K - order of B-spline, K .GE. 1 ID - order of the spline derivative, 0 .LE. ID .LE. K-1 ID=0 gives the spline function X1,X2 - end points of quadrature interval, normally in XI(1) .LE. X .LE. XI(LXI+1) TOL - desired accuracy for the quadrature, suggest 10.*DTOL .LT. TOL .LE. 0.1 where DTOL is the maximum of 1.0D-18 and double precision unit roundoff for the machine = D1MACH(4) Output QUAD is double precision QUAD - integral of PF(X) on (X1,X2) IERR - a status code IERR=1 normal return 2 some quadrature does not meet the requested tolerance Error Conditions Improper input is a fatal error. Some quadrature does not meet the requested tolerance.

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

- Canonical provider: `main-src/src/dpfqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpfqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpfqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpfqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
