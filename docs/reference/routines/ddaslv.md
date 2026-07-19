# DDASLV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Linear system solver for DDASSL.

## Description

THIS ROUTINE MANAGES THE SOLUTION OF THE LINEAR SYSTEM ARISING IN THE NEWTON ITERATION. MATRICES AND REAL TEMPORARY STORAGE AND REAL INFORMATION ARE STORED IN THE ARRAY WM. INTEGER MATRIX INFORMATION IS STORED IN THE ARRAY IWM. FOR A DENSE MATRIX, THE LINPACK ROUTINE DGESL IS CALLED. FOR A BANDED MATRIX,THE LINPACK ROUTINE DGBSL IS CALLED.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `ode-dae-families`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ddaslv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddaslv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddaslv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddaslv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
