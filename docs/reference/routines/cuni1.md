# CUNI1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CUNI1 COMPUTES I(FNU,Z) BY MEANS OF THE UNIFORM ASYMPTOTIC EXPANSION FOR I(FNU,Z) IN -PI/3.LE.ARG Z.LE.PI/3. FNUL IS THE SMALLEST ORDER PERMITTED FOR THE ASYMPTOTIC EXPANSION. NLAST=0 MEANS ALL OF THE Y VALUES WERE SET. NLAST.NE.0 IS THE NUMBER LEFT TO BE COMPUTED BY ANOTHER FORMULA FOR ORDERS FNU TO FNU+NLAST-1 BECAUSE FNU+NLAST-1.LT.FNUL. Y(I)=CZERO FOR I=NLAST+1,N

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

- Canonical provider: `main-src/src/cuni1.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cuni1.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cuni1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cuni1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
