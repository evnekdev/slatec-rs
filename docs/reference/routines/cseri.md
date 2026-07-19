# CSERI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CSERI COMPUTES THE I BESSEL FUNCTION FOR REAL(Z).GE.0.0 BY MEANS OF THE POWER SERIES FOR LARGE ABS(Z) IN THE REGION ABS(Z).LE.2*SQRT(FNU+1). NZ=0 IS A NORMAL RETURN. NZ.GT.0 MEANS THAT THE LAST NZ COMPONENTS WERE SET TO ZERO DUE TO UNDERFLOW. NZ.LT.0 MEANS UNDERFLOW OCCURRED, BUT THE CONDITION ABS(Z).LE.2*SQRT(FNU+1) WAS VIOLATED AND THE COMPUTATION MUST BE COMPLETED IN ANOTHER ROUTINE WITH N=N-ABS(NZ).

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `CBESI, CBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cseri.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cseri.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cseri.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cseri.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
