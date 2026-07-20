# SDZRO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

SDZRO searches for a zero of a function F(N, T, Y, IROOT) between the given values B and C until the width of the interval (B, C) has collapsed to within a tolerance specified by the stopping criterion, ABS(B - C) .LE. 2.*(RW*ABS(B) + AE).

## Description

This is a special purpose version of ZEROIN, modified for use with the SDRIV package. Sandia Mathematical Program Library Mathematical Computing Services Division 5422 Sandia Laboratories P. O. Box 5800 Albuquerque, New Mexico 87115 Control Data 6600 Version 4.5, 1 November 1971 PARAMETERS F - Name of the external function, which returns a real result. This name must be in an EXTERNAL statement in the calling program. B - One end of the interval (B, C). The value returned for B usually is the better approximation to a zero of F. C - The other end of the interval (B, C). RE - Relative error used for RW in the stopping criterion. If the requested RE is less than machine precision, then RW is set to approximately machine precision. AE - Absolute error used in the stopping criterion. If the given interval (B, C) contains the origin, then a nonzero value should be chosen for AE.

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

- Canonical provider: `main-src/src/sdzro.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdzro.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdzro.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdzro.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
