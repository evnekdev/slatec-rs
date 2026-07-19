# LSSUDS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BVSUP

## Description

LSSUDS solves the underdetermined system of equations A Z = B, where A is N by M and N .LE. M. In particular, if rank A equals IRA, a vector X and a matrix U are determined such that X is the UNIQUE solution of smallest length, satisfying A X = B, and the columns of U form an orthonormal basis for the null space of A, satisfying A U = 0 . Then all solutions Z are given by Z = X + C(1)*U(1) + ..... + C(M-IRA)*U(M-IRA) where U(J) represents the J-th column of U and the C(J) are arbitrary constants. If the system of equations are not compatible, only the least squares solution of minimal length is computed. *********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/lssuds.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/lssuds.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/lssuds.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/lssuds.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
