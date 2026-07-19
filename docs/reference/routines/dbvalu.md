# DBVALU

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Evaluate the B-representation of a B-spline at X for the function value or any of its derivatives.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. Right limiting values (right derivatives) are returned except at the right end point X=T(N+1) where left limiting values are computed. The spline is defined on T(K) .LE. X .LE. T(N+1). DBVALU returns a fatal error message when X is outside of this interval. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV Description of Arguments Input T,A,X are double precision T - knot vector of length N+K A - B-spline coefficient vector of length N N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 IDERIV - order of the derivative, 0 .LE. IDERIV .LE. K-1 IDERIV = 0 returns the B-spline value X - argument, T(K) .LE. X .LE. T(N+1) INBV - an initialization parameter which must be set to 1 the first time DBVALU is called. Output WORK,DBVALU are double precision INBV - INBV contains information for efficient processing after the initial call and INBV must not be changed by the user. Distinct splines require distinct INBV parameters. WORK - work vector of length 3*K. DBVALU - value of the IDERIV-th derivative at X Error Conditions An improper input is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
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

- Canonical provider: `main-src/src/dbvalu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvalu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::bspline::dbvalu`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
