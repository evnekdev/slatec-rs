# XERDMP

[Family: Error handling](../families/error-handling.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Print the error tables and then clear them.

## Description

Abstract XERDMP prints the error tables, then clears them.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Error handling`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-error`
- GAMS classifications: `R3C`
- Family evidence: `package_provenance` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xerdmp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xerdmp.f` (`live-main-source`)
  - `err/xerdmp.f` (`legacy-error-directory`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xerdmp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xerdmp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `support_unit_minimal`
- Description provenance: `source_prologue`
- Assessment: the support identity records its role, side-effect boundary, and non-public disposition
- Dedicated family page: [Error handling](../families/error-handling.md)

No independently callable argument list is present in the selected interface inventory for this identity.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `support-routine`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
