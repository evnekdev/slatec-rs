# DSOSSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSOS

## Description

DSOSSL solves an upper triangular type of linear system by back substitution. The matrix C is upper trapezoidal and stored as a linear array by rows. The equations have been normalized so that the diagonal entries of C are understood to be unity. The off diagonal entries and the elements of the constant right hand side vector B have already been stored as the negatives of the corresponding equation values. With each call to DSOSSL a (K-1) by (K-1) triangular system is resolved. For L greater than K, column L of C is included in the right hand side vector.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DSOS`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dsossl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dsossl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dsossl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
