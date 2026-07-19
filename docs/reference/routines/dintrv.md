# DINTRV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the largest integer ILEFT in 1 .LE. ILEFT .LE. LXT such that XT(ILEFT) .LE. X where XT(*) is a subdivision of the X interval.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DINTRV is the INTERV routine of the reference. DINTRV computes the largest integer ILEFT in 1 .LE. ILEFT .LE. LXT such that XT(ILEFT) .LE. X where XT(*) is a subdivision of the X interval. Precisely, X .LT. XT(1) 1 -1 if XT(I) .LE. X .LT. XT(I+1) then ILEFT=I , MFLAG=0 XT(LXT) .LE. X LXT 1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT. Description of Arguments Input XT,X are double precision XT - XT is a knot or break point vector of length LXT LXT - length of the XT vector X - argument ILO - an initialization parameter which must be set to 1 the first time the spline array XT is processed by DINTRV.

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

- Canonical provider: `main-src/src/dintrv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dintrv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dintrv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
