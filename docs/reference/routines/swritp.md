# SWRITP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

WRITE RECORD NUMBER IRECN, OF LENGTH LPG, FROM STORAGE ARRAY LIST(*) ONTO UNIT NUMBER IPAGEF. WRITE RECORD NUMBER IRECN+1, OF LENGTH LPG, ONTO UNIT NUMBER IPAGEF FROM THE STORAGE ARRAY RLIST(*). TO CHANGE THIS PROGRAM UNIT TO DOUBLE PRECISION CHANGE /REAL (12 BLANKS)/ TO /DOUBLE PRECISION/.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SPLP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/swritp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/swritp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/swritp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/swritp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
