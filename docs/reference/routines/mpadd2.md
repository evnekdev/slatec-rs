# MPADD2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DQDOTA and DQDOTI

## Description

Called by MPADD, MPSUB etc. X, Y and Z are MP numbers, Y1 and TRUNC are integers. To force call by reference rather than value/result, Y1 is declared as an array, but only Y1(1) is ever used. Sets Z = X + Y1(1)*ABS(Y), where Y1(1) = +- Y(1). If TRUNC .EQ. 0, R*-rounding is used; otherwise, truncation. R*-rounding is defined in the Kuki and Cody reference. The arguments X(*), Y(*), and Z(*) are all INTEGER arrays of size 30. See the comments in the routine MPBLAS for the reason for this choice.

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

- Canonical provider: `main-src/src/mpadd2.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/mpadd2.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/mpadd2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/mpadd2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
