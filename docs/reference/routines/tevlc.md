# TEVLC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBLKTR

## Description

This subroutine finds the eigenvalues of a symmetric tridiagonal matrix by the rational QL method. On Input- N is the order of the matrix, D contains the diagonal elements of the input matrix, E2 contains the subdiagonal elements of the input matrix in its last N-1 positions. E2(1) is arbitrary. On Output- D contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct and ordered for indices 1,2,...IERR-1, but may not be the smallest eigenvalues, E2 has been destroyed, IERR is set to ZERO for normal return, J if the J-th eigenvalue has not been determined after 30 iterations.

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
- Parent-family evidence: `CBLKTR`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/tevlc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/tevlc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/tevlc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
