# XADD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To provide single-precision floating-point arithmetic with an extended exponent range.

## Description

REAL X, Y, Z INTEGER IX, IY, IZ FORMS THE EXTENDED-RANGE SUM (Z,IZ) = (X,IX) + (Y,IY). (Z,IZ) IS ADJUSTED BEFORE RETURNING. THE INPUT OPERANDS NEED NOT BE IN ADJUSTED FORM, BUT THEIR PRINCIPAL PARTS MUST SATISFY RADIX**(-2L).LE.ABS(X).LE.RADIX**(2L), RADIX**(-2L).LE.ABS(Y).LE.RADIX**(2L).

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

- Canonical provider: `main-src/src/xadd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xadd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xadd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xadd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
