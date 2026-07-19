# PRODP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BLKTRI

## Description

PRODP applies a sequence of matrix operations to the vector X and stores the result in Y (periodic boundary conditions). BD,BM1,BM2 are arrays containing roots of certain B polynomials. ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. AA Array containing scalar multipliers of the vector X. NA is the length of the array AA. X,Y The matrix operations are applied to X and the result is Y. A,B,C are arrays which contain the tridiagonal matrix. M is the order of the matrix. D,W,U are working arrays. IS determines whether or not a change in sign is made.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BLKTRI`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/prodp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/prodp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/prodp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
