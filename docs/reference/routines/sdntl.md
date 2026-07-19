# SDNTL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subroutine SDNTL is called to set parameters on the first call to SDSTP, on an internal restart, or when the user has altered MINT, MITER, and/or H.

## Description

On the first call, the order is set to 1 and the initial derivatives are calculated. RMAX is the maximum ratio by which H can be increased in one step. It is initially RMINIT to compensate for the small initial H, but then is normally equal to RMNORM. If a failure occurs (in corrector convergence or error test), RMAX is set at RMFAIL for the next increase. If the caller has changed MINT, or if JTASK = 0, SDCST is called to set the coefficients of the method. If the caller has changed H, YH must be rescaled. If H or MINT has been changed, NWAIT is reset to NQ + 2 to prevent further increases in H for that many steps. Also, RC is reset. RC is the ratio of new to old values of the coefficient L(0)*H. If the caller has changed MITER, RC is set to 0 to force the partials to be updated, if partials are used.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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

- Canonical provider: `main-src/src/sdntl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdntl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdntl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdntl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
