# DBVDER

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBVSUP

## Description

********************************************************************** NFC = Number of base solution vectors NCOMP = Number of components per solution vector 1 -- Nonzero particular solution INHOMO = 2 or 3 -- Zero particular solution 0 -- Inhomogeneous vector term G(X) identically zero IGOFX = 1 -- Inhomogeneous vector term G(X) not identically zero G = Inhomogeneous vector term G(X) XSAV = Previous value of X C = Normalization factor for the particular solution 0 ( if NEQIVP = 0 ) IVP = Number of differential equations integrated due to the original boundary value problem ( if NEQIVP .GT. 0 ) NOFST - For problems with auxiliary initial value equations, NOFST communicates to the routine DFMAT how to access the dependent variables corresponding to this initial value problem. For example, during any call to DFMAT, the first dependent variable for the initial value problem is in position Y(NOFST + 1). See example in SAND77-1328. **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbvder.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvder.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvder.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvder.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
