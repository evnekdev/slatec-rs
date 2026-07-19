# PRWVIR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

PRWVIR LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SPARSE MATRIX STORAGE SCHEME. THE PAGE STORAGE IS ON RANDOM ACCESS DISK. PRWVIR IS PART OF THE SPARSE LP PACKAGE, SPLP. KEY IS A FLAG WHICH INDICATES WHETHER A READ OR WRITE OPERATION IS TO BE PERFORMED. A VALUE OF KEY=1 INDICATES A READ. A VALUE OF KEY=2 INDICATES A WRITE. IPAGE IS THE PAGE OF MATRIX MN WE ARE ACCESSING. LPG IS THE LENGTH OF THE PAGE. SX(*),IX(*) IS THE MATRIX DATA. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LRWVIR, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON

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

- Canonical provider: `main-src/src/prwvir.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/prwvir.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/prwvir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/prwvir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
