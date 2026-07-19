# LA05BS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

THIS SUBPROGRAM IS A SLIGHT MODIFICATION OF A SUBPROGRAM FROM THE C. 1979 AERE HARWELL LIBRARY. THE NAME OF THE CORRESPONDING HARWELL CODE CAN BE OBTAINED BY DELETING THE FINAL LETTER =S= IN THE NAMES USED HERE. REVISED SEP. 13, 1979. ROYALTIES HAVE BEEN PAID TO AERE-UK FOR USE OF THEIR CODES IN THE PACKAGE GIVEN HERE. ANY PRIMARY USAGE OF THE HARWELL SUBROUTINES REQUIRES A ROYALTY AGREEMENT AND PAYMENT BETWEEN THE USER AND AERE-UK. ANY USAGE OF THE SANDIA WRITTEN CODES SPLP( ) (WHICH USES THE HARWELL SUBROUTINES) IS PERMITTED. IP(I,1),IP(I,2) POINT TO START OF ROW/COLUMN I OF U. IW(I,1),IW(I,2) ARE LENGTHS OF ROW/COL I OF U. IW(.,3),IW(.,4) HOLD ROW/COL NUMBERS IN PIVOTAL ORDER.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/la05bs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/la05bs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/la05bs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/la05bs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
