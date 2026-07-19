# MPNZR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DQDOTA and DQDOTI

## Description

Modified for use with BLAS. Blank COMMON changed to named COMMON. Assumes long (i.e. (t+4)-DIGIT) fraction in R, sign = RS, exponent = RE. Normalizes, and returns 'mp' result in Z. Integer arguments RS and RE are not preserved. R*-rounding is used if TRUNC.EQ.0 The argument Z(*) and the variable R in COMMON are INTEGER arrays of size 30. See the comments in the routine MPBLAS for the reason for this choice.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DQDOTA, DQDOTI`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/mpnzr.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/mpnzr.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/mpnzr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/mpnzr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
