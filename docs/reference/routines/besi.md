# BESI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.

## Description

Abstract BESI computes an N member sequence of I Bessel functions I/sub(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity, and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values not covered by one of these formulae, the order is incremented by an integer so that one of these formulae apply. Backward recursion is used to reduce orders by integer values. The asymptotic expansion for X to infinity is used only when the entire sequence (specifically the last member) lies within the region covered by the expansion. Leading terms of these expansions are used to test for over or underflow where appropriate. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all are set to zero. An overflow cannot occur with scaling. Description of Arguments

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

- Canonical provider: `main-src/src/besi.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/besi.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/besi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/besi.f) — `verified_cached`
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
