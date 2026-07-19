# XRED

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To provide single-precision floating-point arithmetic with an extended exponent range.

## Description

REAL X INTEGER IX IF RADIX**(-2L) .LE. (ABS(X),IX) .LE. RADIX**(2L) THEN XRED TRANSFORMS (X,IX) SO THAT IX=0. IF (X,IX) IS OUTSIDE THE ABOVE RANGE, THEN XRED TAKES NO ACTION. THIS SUBROUTINE IS USEFUL IF THE RESULTS OF EXTENDED-RANGE CALCULATIONS ARE TO BE USED IN SUBSEQUENT ORDINARY SINGLE-PRECISION CALCULATIONS.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Arithmetic and extended-range arithmetic`
- Mathematical domain: `numeric-support`
- Package provenance: `unknown`
- GAMS classifications: `A3D`
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

- Canonical provider: `main-src/src/xred.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xred.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xred.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
