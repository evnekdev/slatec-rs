# BSPEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the spline and its derivatives from the B-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BSPEV is the BSPLEV routine of the reference. BSPEV calculates the value of the spline and its derivatives at X from the B-representation (T,A,N,K) and returns them in SVALUE(I),I=1,NDERIV, T(K) .LE. X .LE. T(N+1). AD(I) can be the B-spline coefficients A(I), I=1,N if NDERIV=1. Otherwise AD must be computed before hand by a call to BSPDR (T,A, N,K,NDERIV,AD). If X=T(I),I=K,N, right limiting values are obtained. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. BSPEV calls INTRV, BSPVN Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/bspev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bspev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bspev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bspev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
