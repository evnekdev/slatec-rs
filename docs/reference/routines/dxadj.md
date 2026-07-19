# DXADJ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To provide double-precision floating-point arithmetic with an extended exponent range.

## Description

DOUBLE PRECISION X INTEGER IX TRANSFORMS (X,IX) SO THAT RADIX**(-L) .LE. ABS(X) .LT. RADIX**L. ON MOST COMPUTERS THIS TRANSFORMATION DOES NOT CHANGE THE MANTISSA OF X PROVIDED RADIX IS THE NUMBER BASE OF DOUBLE-PRECISION ARITHMETIC.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/dxadj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxadj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxadj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dxadj.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
