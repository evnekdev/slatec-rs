# DLPDP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DLSEI

## Description

**** Double Precision version of LPDP **** DIMENSION A(MDA,N+1),PRGOPT(*),X(N),WS((M+2)*(N+7)),IS(M+N+1), where N=N1+N2. This is a slight overestimate for WS(*). Determine an N1-vector W, and an N2-vector Z which minimizes the Euclidean length of W subject to G*W+H*Z .GE. Y. This is the least projected distance problem, LPDP. The matrices G and H are of respective dimensions M by N1 and M by N2. Called by subprogram DLSI( ). The matrix (G H Y) occupies rows 1,...,M and cols 1,...,N1+N2+1 of A(*,*). The solution (W) is returned in X(*). (Z) The value of MODE indicates the status of the computation after returning to the user. MODE=1 The solution was successfully obtained. MODE=2 The inequalities are inconsistent.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DLSEI`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dlpdp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dlpdp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dlpdp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dlpdp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
