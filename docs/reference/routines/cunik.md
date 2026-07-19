# CUNIK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CUNIK COMPUTES PARAMETERS FOR THE UNIFORM ASYMPTOTIC EXPANSIONS OF THE I AND K FUNCTIONS ON IKFLG= 1 OR 2 RESPECTIVELY BY W(FNU,ZR) = PHI*EXP(ZETA)*SUM WHERE ZETA=-ZETA1 + ZETA2 OR ZETA1 - ZETA2 THE FIRST CALL MUST HAVE INIT=0. SUBSEQUENT CALLS WITH THE SAME ZR AND FNU WILL RETURN THE I OR K FUNCTION ON IKFLG= 1 OR 2 WITH NO CHANGE IN INIT. CWRK IS A COMPLEX WORK ARRAY. IPMTR=0 COMPUTES ALL PARAMETERS. IPMTR=1 COMPUTES PHI, ZETA1,ZETA2.

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

- Canonical provider: `main-src/src/cunik.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cunik.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cunik.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cunik.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
