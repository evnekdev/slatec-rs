# BESK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.

## Description

Abstract BESK implements forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/sub(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/sub(FNU+I-1)/(X), I=1,...,N for real X .GT. 0.0E0 and non-negative orders FNU. If FNU .LT. NULIM, orders FNU and FNU+1 are obtained from BESKNU to start the recursion. If FNU .GE. NULIM, the uniform asymptotic expansion is used for orders FNU and FNU+1 to start the recursion. NULIM is 35 or 70 depending on whether N=1 or N .GE. 2. Under and overflow tests are made on the leading term of the asymptotic expansion before any extensive computation is done. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/besk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/besk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/besk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/besk.f) — `verified_cached`
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
