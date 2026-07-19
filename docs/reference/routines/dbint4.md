# DBINT4

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a cubic spline which interpolates given data.

## Description

Abstract **** a double precision routine **** DBINT4 computes the B representation (T,BCOEF,N,K) of a cubic spline (K=4) which interpolates data (X(I),Y(I)), I=1,NDATA. Parameters IBCL, IBCR, FBCL, FBCR allow the specification of the spline first or second derivative at both X(1) and X(NDATA). When this data is not specified by the problem, it is common practice to use a natural spline by setting second derivatives at X(1) and X(NDATA) to zero (IBCL=IBCR=2,FBCL=FBCR=0.0). The spline is defined on T(4) .LE. X .LE. T(N+1) with (ordered) interior knots at X(I) values where N=NDATA+2. The knots T(1),T(2),T(3) lie to the left of T(4)=X(1) and the knots T(N+2), T(N+3), T(N+4) lie to the right of T(N+1)=X(NDATA) in increasing order. If no extrapolation outside (X(1),X(NDATA)) is anticipated, the knots T(1)=T(2)=T(3)=T(4)=X(1) and T(N+2)=T(N+3)=T(N+4)= T(N+1)=X(NDATA) can be specified by KNTOPT=1. KNTOPT=2 selects a knot placement for T(1), T(2), T(3) to make the first 7 knots symmetric about T(4)=X(1) and similarly for T(N+2), T(N+3), T(N+4) about T(N+1)=X(NDATA). KNTOPT=3 allows the user to make his own selection, in increasing order, for T(1), T(2), T(3) to the left of X(1) and T(N+2), T(N+3), T(N+4) to the right of X(NDATA) in the work array W(1) through W(6). In any case, the interpolation on T(4) .LE. X .LE. T(N+1) by using function DBVALU is unique for given boundary conditions. Description of Arguments Input X,Y,FBCL,FBCR,W are double precision X - X vector of abscissae of length NDATA, distinct and in increasing order Y - Y vector of ordinates of length NDATA NDATA - number of data points, NDATA .GE. 2 IBCL - selection parameter for left boundary condition IBCL = 1 constrain the first derivative at X(1) to FBCL = 2 constrain the second derivative at X(1) to FBCL IBCR - selection parameter for right boundary condition IBCR = 1 constrain first derivative at X(NDATA) to FBCR IBCR = 2 constrain second derivative at X(NDATA) to FBCR FBCL - left boundary values governed by IBCL FBCR - right boundary values governed by IBCR KNTOPT - knot selection parameter KNTOPT = 1 sets knot multiplicity at T(4) and T(N+1) to 4 = 2 sets a symmetric placement of knots about T(4) and T(N+1) = 3 sets T(I)=W(I) and T(N+1+I)=W(3+I),I=1,3 where W(I),I=1,6 is supplied by the user W - work array of dimension at least 5*(NDATA+2) If KNTOPT=3, then W(1),W(2),W(3) are knot values to the left of X(1) and W(4),W(5),W(6) are knot values to the right of X(NDATA) in increasing order to be supplied by the user Output T,BCOEF are double precision T - knot array of length N+4 BCOEF - B spline coefficient array of length N N - number of coefficients, N=NDATA+2 K - order of spline, K=4 Error Conditions Improper input is a fatal error Singular system of equations is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1A`
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

- Canonical provider: `main-src/src/dbint4.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbint4.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbint4.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbint4.f) — `verified_cached`
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
