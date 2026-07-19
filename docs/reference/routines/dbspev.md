# DBSPEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the spline and its derivatives from the B-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPEV is the BSPLEV routine of the reference. DBSPEV calculates the value of the spline and its derivatives at X from the B-representation (T,A,N,K) and returns them in SVALUE(I),I=1,NDERIV, T(K) .LE. X .LE. T(N+1). AD(I) can be the B-spline coefficients A(I), I=1,N) if NDERIV=1. Otherwise AD must be computed before hand by a call to DBSPDR (T,A,N,K, NDERIV,AD). If X=T(I),I=K,N), right limiting values are obtained. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBSPEV calls DINTRV, DBSPVN Description of Arguments Input T,AD,X, are double precision T - knot vector of length N+K AD - vector of length (2*N-NDERIV+1)*NDERIV/2 containing the difference table from DBSPDR. N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 NDERIV - number of derivatives, 1 .LE. NDERIV .LE. K. NDERIV=1 gives the zero-th derivative = function value X - argument, T(K) .LE. X .LE. T(N+1) INEV - an initialization parameter which must be set to 1 the first time DBSPEV is called. Output SVALUE,WORK are double precision INEV - INEV contains information for efficient processing after the initial call and INEV must not be changed by the user. Distinct splines require distinct INEV parameters. SVALUE - vector of length NDERIV containing the spline value in SVALUE(1) and the NDERIV-1 derivatives in the remaining components. WORK - work vector of length 3*K Error Conditions Improper input is a fatal error.

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
- GAMS classifications: `E3`
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

- Canonical provider: `main-src/src/dbspev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspev.f) — `verified_cached`
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
