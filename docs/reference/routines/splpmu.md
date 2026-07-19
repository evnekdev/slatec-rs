# SPLPMU

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

THE EDITING REQUIRED TO CONVERT THIS SUBROUTINE FROM SINGLE TO DOUBLE PRECISION INVOLVES THE FOLLOWING CHARACTER STRING CHANGES. USE AN EDITING COMMAND (CHANGE) /STRING-1/(TO)STRING-2/. /REAL (12 BLANKS)/DOUBLE PRECISION/, /SASUM/DASUM/,/SCOPY/DCOPY/,/SDOT/DDOT/, /.E0/.D0/ THIS SUBPROGRAM IS FROM THE SPLP( ) PACKAGE. IT PERFORMS THE TASKS OF UPDATING THE PRIMAL SOLUTION, EDGE WEIGHTS, REDUCED COSTS, AND MATRIX DECOMPOSITION. IT IS THE MAIN PART OF THE PROCEDURE (MAKE MOVE AND UPDATE). REVISED 821122-1100 REVISED YYMMDD

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

- Canonical provider: `main-src/src/splpmu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/splpmu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/splpmu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/splpmu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
