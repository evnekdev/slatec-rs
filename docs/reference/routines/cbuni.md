# CBUNI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CBUNI COMPUTES THE I BESSEL FUNCTION FOR LARGE ABS(Z).GT. FNUL AND FNU+N-1.LT.FNUL. THE ORDER IS INCREASED FROM FNU+N-1 GREATER THAN FNUL BY ADDING NUI AND COMPUTING ACCORDING TO THE UNIFORM ASYMPTOTIC EXPANSION FOR I(FNU,Z) ON IFORM=1 AND THE EXPANSION FOR J(FNU,Z) ON IFORM=2

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

- Canonical provider: `main-src/src/cbuni.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbuni.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbuni.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbuni.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
