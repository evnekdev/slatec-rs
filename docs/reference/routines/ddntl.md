# DDNTL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subroutine DDNTL is called to set parameters on the first call to DDSTP, on an internal restart, or when the user has altered MINT, MITER, and/or H.

## Description

On the first call, the order is set to 1 and the initial derivatives are calculated. RMAX is the maximum ratio by which H can be increased in one step. It is initially RMINIT to compensate for the small initial H, but then is normally equal to RMNORM. If a failure occurs (in corrector convergence or error test), RMAX is set at RMFAIL for the next increase. If the caller has changed MINT, or if JTASK = 0, DDCST is called to set the coefficients of the method. If the caller has changed H, YH must be rescaled. If H or MINT has been changed, NWAIT is reset to NQ + 2 to prevent further increases in H for that many steps. Also, RC is reset. RC is the ratio of new to old values of the coefficient L(0)*H. If the caller has changed MITER, RC is set to 0 to force the partials to be updated, if partials are used.

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

- Canonical provider: `main-src/src/ddntl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddntl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddntl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddntl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
