# DFULMT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

DECODES A STANDARD TWO-DIMENSIONAL FORTRAN ARRAY PASSED IN THE ARRAY DATTRV(IA,*). THE ROW DIMENSION IA AND THE MATRIX DIMENSIONS MRELAS AND NVARS MUST SIMULTANEOUSLY BE PASSED USING THE OPTION ARRAY, PRGOPT(*). IT IS AN ERROR IF THIS DATA IS NOT PASSED TO DFULMT( ). EXAMPLE-- (FOR USE TOGETHER WITH DSPLP().) EXTERNAL DUSRMT DIMENSION DATTRV(IA,*) PRGOPT(01)=7 PRGOPT(02)=68 PRGOPT(03)=1 PRGOPT(04)=IA PRGOPT(05)=MRELAS PRGOPT(06)=NVARS PRGOPT(07)=1 CALL DSPLP( ... DFULMT INSTEAD OF DUSRMT...)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DSPLP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dfulmt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dfulmt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dfulmt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dfulmt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
